//! Port of StaticNose's pure delegation and fan out rule.

use std::collections::BTreeSet;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Middle Man";

struct Analysis<'a> {
    file: &'a SourceFile,
    project_types: &'a BTreeSet<String>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    files
        .iter()
        .flat_map(|file| {
            let project_types = project_type_names(files, file.language);
            let analysis = Analysis {
                file,
                project_types: &project_types,
            };
            let stats: StatsReader = if file.language == LanguageId::Java {
                |query| query.java_stats()
            } else {
                |query| query.non_java_stats()
            };
            file_findings(&analysis, stats)
        })
        .collect()
}

fn project_type_names(files: &[SourceFile], language: LanguageId) -> BTreeSet<String> {
    files
        .iter()
        .filter(|file| compatible_language(file.language, language))
        .flat_map(|file| {
            let model = model_for(file.language);
            model
                .types(file)
                .into_iter()
                .filter(|ty| !matches!(ty.kind(), "source_file" | "program"))
                .filter_map(move |ty| model.type_name(file.at(ty)).map(str::to_owned))
        })
        .collect()
}

fn compatible_language(left: LanguageId, right: LanguageId) -> bool {
    match left {
        LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => matches!(
            right,
            LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx
        ),
        _ => left == right,
    }
}

struct DelegationStats {
    methods: usize,
    delegates: usize,
    targets: usize,
}

impl DelegationStats {
    fn is_middle_man(&self) -> bool {
        self.delegates >= 2 && self.delegates * 2 >= self.methods && self.targets > 0
    }
}

type StatsReader = fn(&DelegationAnalysis<'_>) -> Option<DelegationStats>;

fn non_java_field_types(
    file: &SourceFile,
    ty: Node<'_>,
) -> std::collections::BTreeMap<String, String> {
    let mut fields = std::collections::BTreeMap::new();
    if file.language == LanguageId::Rust {
        let Some(body) = ty.child_by_field_name("body") else {
            return fields;
        };
        let mut cursor = body.walk();
        for field in body
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "field_declaration")
        {
            if let (Some(name), Some(field_type)) = (
                field.child_by_field_name("name"),
                field.child_by_field_name("type"),
            ) {
                fields.insert(file.text(name).to_owned(), file.text(field_type).to_owned());
            }
        }
        return fields;
    }
    let Some(body) = ty.child_by_field_name("body") else {
        return fields;
    };
    for field in descendants(body).into_iter().filter(|node| {
        matches!(
            node.kind(),
            "field_definition" | "public_field_definition" | "property_definition"
        )
    }) {
        let Some(name) = field
            .child_by_field_name("property")
            .or_else(|| field.child_by_field_name("name"))
        else {
            continue;
        };
        if let Some(field_type) = field.child_by_field_name("type") {
            fields.insert(
                file.text(name).trim_start_matches('#').to_owned(),
                file.text(field_type).to_owned(),
            );
        } else if let Some(value) = field.child_by_field_name("value")
            && value.kind() == "new_expression"
            && let Some(constructor) = value.child_by_field_name("constructor")
        {
            fields.insert(
                file.text(name).to_owned(),
                file.text(constructor).to_owned(),
            );
        }
    }
    for constructor in model_for(file.language).constructors(file.at(ty)) {
        for assignment in descendants(constructor)
            .into_iter()
            .filter(|node| node.kind() == "assignment_expression")
        {
            let (Some(left), Some(right)) = (
                assignment.child_by_field_name("left"),
                assignment.child_by_field_name("right"),
            ) else {
                continue;
            };
            if left.kind() != "member_expression" || right.kind() != "new_expression" {
                continue;
            }
            let (Some(object), Some(property), Some(constructor)) = (
                left.child_by_field_name("object"),
                left.child_by_field_name("property"),
                right.child_by_field_name("constructor"),
            ) else {
                continue;
            };
            if file.text(object) == "this" {
                fields.insert(
                    file.text(property).to_owned(),
                    file.text(constructor).to_owned(),
                );
            }
        }
    }
    fields
}

fn non_java_pure_delegate(
    analysis: &Analysis<'_>,
    method: Node<'_>,
    fields: &std::collections::BTreeMap<String, String>,
) -> bool {
    let file = analysis.file;
    let project_types = analysis.project_types;
    let Some(statements) = model_for(file.language).body_statements(method) else {
        return false;
    };
    if statements.len() != 1 {
        return false;
    }
    let statement = statements[0];
    if !matches!(
        statement.kind(),
        "return_statement" | "return_expression" | "expression_statement" | "call_expression"
    ) {
        return false;
    }
    let call = descendants(statement)
        .into_iter()
        .find(|node| matches!(node.kind(), "call_expression" | "method_invocation"));
    let Some(call) = call else {
        return false;
    };
    let receiver = match file.language {
        LanguageId::Rust => call
            .child_by_field_name("function")
            .filter(|function| function.kind() == "field_expression")
            .and_then(|function| function.child_by_field_name("value")),
        LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx => call
            .child_by_field_name("function")
            .filter(|function| function.kind() == "member_expression")
            .and_then(|function| function.child_by_field_name("object")),
        LanguageId::Java => return false,
    };
    let Some(receiver) = receiver else {
        return false;
    };
    let field_name = if receiver.kind() == "field_expression" {
        receiver
            .child_by_field_name("field")
            .map(|node| file.text(node))
    } else if receiver.kind() == "member_expression" {
        receiver
            .child_by_field_name("property")
            .map(|node| file.text(node))
    } else {
        None
    };
    let Some(field_name) = field_name else {
        return false;
    };
    fields
        .get(field_name)
        .is_some_and(|target| project_types.contains(&simple_name(target)))
}

fn simple_name(name: &str) -> String {
    name.trim()
        .trim_start_matches(':')
        .trim()
        .rsplit("::")
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
        .trim()
        .to_owned()
}

fn file_findings(analysis: &Analysis<'_>, read_stats: StatsReader) -> Vec<Finding> {
    let file = analysis.file;
    let model = model_for(file.language);
    model
        .types(file)
        .into_iter()
        .filter_map(|ty| {
            let name = model.type_name(file.at(ty))?;
            let stats = read_stats(&DelegationAnalysis {
                analysis,
                owner: ty,
            })?;
            stats.is_middle_man().then(|| Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: file.rel.clone(),
                class_name: name.to_string(),
                line_numbers: vec![model.decl_line(ty)],
                relative_file_path: file.rel.clone(),
                additional_info: String::new(),
            })
        })
        .collect()
}

fn statement_call(statement: Node<'_>) -> Option<Node<'_>> {
    if statement.kind() == "expression_statement" {
        let mut cursor = statement.walk();
        return statement
            .named_children(&mut cursor)
            .find(|node| node.kind() == "method_invocation");
    }
    if statement.kind() == "return_statement" {
        return descendants(statement)
            .into_iter()
            .find(|node| node.kind() == "method_invocation");
    }
    None
}

fn first_named(node: Node<'_>) -> Option<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).next()
}

struct DelegationAnalysis<'a> {
    analysis: &'a Analysis<'a>,
    owner: Node<'a>,
}
impl DelegationAnalysis<'_> {
    fn is_pure_delegate(&self, method: Node<'_>) -> bool {
        let Some(statements) = model_for(self.analysis.file.language).body_statements(method)
        else {
            return false;
        };
        if statements.is_empty() {
            return false;
        }

        let mut found_delegate = false;
        for statement in statements {
            if let Some(call) = statement_call(statement) {
                if self.delegates_to_field(call) {
                    found_delegate = true;
                } else if statement.kind() != "expression_statement" {
                    return false;
                }
            } else if statement.kind() == "return_statement" {
                if let Some(expression) = first_named(statement)
                    && !matches!(
                        expression.kind(),
                        "identifier"
                            | "decimal_integer_literal"
                            | "string_literal"
                            | "true"
                            | "false"
                            | "null_literal"
                    )
                {
                    return false;
                }
            } else {
                return false;
            }
        }
        found_delegate
    }

    fn delegates_to_field(&self, call: Node<'_>) -> bool {
        let Some(object) = call.child_by_field_name("object") else {
            return false;
        };
        let object_name = if object.kind() == "field_access" {
            object
                .child_by_field_name("field")
                .map(|field| self.analysis.file.text(field))
                .unwrap_or_default()
        } else {
            self.analysis.file.text(object)
        };
        let Some(owner_body) = self.owner.child_by_field_name("body") else {
            return false;
        };
        let mut cursor = owner_body.walk();
        owner_body
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "field_declaration")
            .any(|field| {
                let type_name = field.child_by_field_name("type").map(|ty| {
                    self.analysis
                        .file
                        .text(ty)
                        .rsplit('.')
                        .next()
                        .unwrap_or(self.analysis.file.text(ty))
                });
                if !type_name.is_some_and(|name| self.analysis.project_types.contains(name)) {
                    return false;
                }
                let mut cursor = field.walk();
                field
                    .named_children(&mut cursor)
                    .filter(|node| node.kind() == "variable_declarator")
                    .any(|variable| {
                        variable
                            .child_by_field_name("name")
                            .is_some_and(|name| self.analysis.file.text(name) == object_name)
                    })
            })
    }
}

impl DelegationAnalysis<'_> {
    fn non_java_stats(&self) -> Option<DelegationStats> {
        let file = self.analysis.file;
        let fields = non_java_field_types(file, self.owner);
        if fields.is_empty() {
            return None;
        }
        let methods = model_for(file.language).methods(file.at(self.owner));
        if methods.is_empty() {
            return None;
        }
        let delegates = methods
            .iter()
            .filter(|method| non_java_pure_delegate(self.analysis, **method, &fields))
            .count();
        let targets = fields
            .values()
            .filter(|target| self.analysis.project_types.contains(&simple_name(target)))
            .map(|target| simple_name(target))
            .collect::<BTreeSet<_>>()
            .len();
        Some(DelegationStats {
            methods: methods.len(),
            delegates,
            targets,
        })
    }

    fn java_stats(&self) -> Option<DelegationStats> {
        let methods = model_for(LanguageId::Java).methods(self.analysis.file.at(self.owner));
        if methods.is_empty() {
            return None;
        }
        let delegates = methods
            .iter()
            .filter(|method| self.is_pure_delegate(**method))
            .count();
        Some(DelegationStats {
            methods: methods.len(),
            delegates,
            targets: self.referenced_project_types(),
        })
    }

    fn referenced_project_types(&self) -> usize {
        let mut names = BTreeSet::new();
        for node in descendants(self.owner) {
            if node.kind() == "type_identifier" {
                let name = self
                    .analysis
                    .file
                    .text(node)
                    .rsplit('.')
                    .next()
                    .unwrap_or(self.analysis.file.text(node));
                if name
                    != self
                        .analysis
                        .file
                        .text(self.owner.child_by_field_name("name").unwrap())
                    && self.analysis.project_types.contains(name)
                {
                    names.insert(name.to_owned());
                }
            }
        }
        names.len()
    }
}
