//! Best-effort Divergent Change heuristic for Java classes.
//!
//! The heuristic looks for low field cohesion, repeated switches/actions, and
//! the same field driving conditionals in multiple methods. These source-level
//! signals approximate unrelated reasons to change; historical edits are not
//! available to the analyser.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, LanguageModel, SourceFile, java};
use crate::walk::descendants;

pub const SMELL_NAME: &str = "Divergent Change";
const MIN_FIELD_USING_METHODS: usize = 3;
const MIN_DISJOINT_PAIR_RATIO: f64 = 0.75;

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .filter(|file| file.language == LanguageId::Java)
        .flat_map(file_findings)
        .collect()
}

fn file_findings(file: &SourceFile) -> Vec<Finding> {
    java::JAVA
        .types(file)
        .into_iter()
        .filter(|ty| ty.kind() == "class_declaration")
        .filter(|ty| ClassAnalysis { file, class: *ty }.is_divergent())
        .filter_map(|ty| {
            Some(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: file.rel.clone(),
                class_name: java::JAVA.type_name(file.at(ty))?.to_owned(),
                line_numbers: vec![java::JAVA.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

struct ClassAnalysis<'a> {
    file: &'a SourceFile,
    class: tree_sitter::Node<'a>,
}
impl ClassAnalysis<'_> {
    fn is_divergent(&self) -> bool {
        if self.has_repeated_action_or_switches() || self.same_field_drives_multiple_methods() {
            return true;
        }
        let fields = self.fields();
        if fields.is_empty() {
            return false;
        }
        let method_fields = java::JAVA
            .methods(self.file, self.class)
            .into_iter()
            .map(|method| {
                descendants(method)
                    .into_iter()
                    .filter(|node| node.kind() == "field_access")
                    .filter_map(|access| {
                        let (Some(object), Some(field)) = (
                            access.child_by_field_name("object"),
                            access.child_by_field_name("field"),
                        ) else {
                            return None;
                        };
                        (self.file.text(object) == "this" && fields.contains(self.file.text(field)))
                            .then(|| self.file.text(field).to_owned())
                    })
                    .collect::<BTreeSet<_>>()
            })
            .filter(|used| !used.is_empty())
            .collect::<Vec<_>>();
        if method_fields.len() < MIN_FIELD_USING_METHODS {
            return false;
        }
        let mut disjoint = 0;
        let mut total = 0;
        for left in 0..method_fields.len() {
            for right in left + 1..method_fields.len() {
                total += 1;
                if method_fields[left].is_disjoint(&method_fields[right]) {
                    disjoint += 1;
                }
            }
        }
        total > 0 && disjoint as f64 / total as f64 >= MIN_DISJOINT_PAIR_RATIO
    }

    fn same_field_drives_multiple_methods(&self) -> bool {
        let mut method_counts = std::collections::BTreeMap::<String, usize>::new();
        for method in java::JAVA.methods(self.file, self.class) {
            let fields = descendants(method)
                .into_iter()
                .filter(|node| node.kind() == "if_statement")
                .filter_map(|branch| branch.child_by_field_name("condition"))
                .flat_map(descendants)
                .filter(|access| access.kind() == "field_access")
                .filter_map(|access| {
                    let (Some(object), Some(field)) = (
                        access.child_by_field_name("object"),
                        access.child_by_field_name("field"),
                    ) else {
                        return None;
                    };
                    (self.file.text(object) == "this").then(|| self.file.text(field).to_owned())
                })
                .collect::<BTreeSet<_>>();
            for field in fields {
                *method_counts.entry(field).or_default() += 1;
            }
        }
        method_counts.values().any(|count| *count >= 2)
    }

    fn has_repeated_action_or_switches(&self) -> bool {
        let methods = java::JAVA.methods(self.file, self.class);
        let switch_methods = methods
            .iter()
            .filter(|method| {
                descendants(**method).into_iter().any(|node| {
                    node.kind() == "switch_expression" || node.kind() == "switch_statement"
                })
            })
            .count();
        if switch_methods >= MIN_FIELD_USING_METHODS {
            return true;
        }

        let mut action_counts = std::collections::BTreeMap::<String, usize>::new();
        for method in methods {
            let actions = descendants(method)
                .into_iter()
                .filter(|node| node.kind() == "method_invocation")
                .filter_map(|call| {
                    let (Some(receiver), Some(name)) = (
                        call.child_by_field_name("object"),
                        call.child_by_field_name("name"),
                    ) else {
                        return None;
                    };
                    Some(format!(
                        "{}.{}",
                        self.file.text(receiver),
                        self.file.text(name)
                    ))
                })
                .collect::<BTreeSet<_>>();
            for action in actions {
                *action_counts.entry(action).or_default() += 1;
            }
        }
        action_counts
            .values()
            .any(|count| *count >= MIN_FIELD_USING_METHODS)
    }

    fn fields(&self) -> BTreeSet<String> {
        let Some(body) = self.class.child_by_field_name("body") else {
            return BTreeSet::new();
        };
        let mut cursor = body.walk();
        body.named_children(&mut cursor)
            .filter(|child| child.kind() == "field_declaration")
            .flat_map(|declaration| {
                let mut cursor = declaration.walk();
                declaration
                    .named_children(&mut cursor)
                    .filter(|child| child.kind() == "variable_declarator")
                    .filter_map(|field| field.child_by_field_name("name"))
                    .collect::<Vec<_>>()
            })
            .map(|name| self.file.text(name).to_owned())
            .collect()
    }
}
