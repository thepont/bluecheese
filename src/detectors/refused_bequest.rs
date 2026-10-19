//! Port of StaticNose's explicit and implicit refused inheritance checks.

use std::collections::BTreeMap;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Refused Bequest";

struct TypeInfo<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
    name: String,
    package: String,
}

struct Project<'a> {
    types: Vec<TypeInfo<'a>>,
    by_name: BTreeMap<String, Vec<usize>>,
}

struct ChildContext<'p, 'a> {
    project: &'p Project<'a>,
    child: usize,
    methods: Vec<Node<'a>>,
}

struct ParentMethod<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let project = Project::new(files);
    let mut findings: Vec<_> = (0..project.types.len())
        .filter_map(|index| finding_for_type(&project, index))
        .collect();
    findings.extend(ecmascript_findings(files));
    findings.extend(rust_trait_findings(files));
    findings
}

fn rust_trait_findings(files: &[SourceFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.language == LanguageId::Rust)
    {
        let model = model_for(file.language);
        let types = model.types(file);
        for implementation in descendants(file.root())
            .into_iter()
            .filter(|node| node.kind() == "impl_item")
        {
            let (Some(trait_node), Some(target_node)) = (
                implementation.child_by_field_name("trait"),
                implementation.child_by_field_name("type"),
            ) else {
                continue;
            };
            let trait_name = file
                .text(trait_node)
                .rsplit("::")
                .next()
                .unwrap_or_default();
            let target_name = file
                .text(target_node)
                .rsplit("::")
                .next()
                .unwrap_or_default();
            let Some(trait_type) = types.iter().copied().find(|ty| {
                ty.kind() == "trait_item"
                    && model
                        .type_name(file, *ty)
                        .is_some_and(|name| name == trait_name)
            }) else {
                continue;
            };
            let Some(target_type) = types.iter().copied().find(|ty| {
                matches!(ty.kind(), "struct_item" | "enum_item" | "union_item")
                    && model
                        .type_name(file, *ty)
                        .is_some_and(|name| name == target_name)
            }) else {
                continue;
            };
            let trait_methods = model.methods(file, trait_type);
            let implementation_methods: Vec<_> = descendants(implementation)
                .into_iter()
                .filter(|node| node.kind() == "function_item")
                .collect();
            let mut lines: Vec<_> = implementation_methods
                .iter()
                .copied()
                .filter(|method| rust_is_refusal_stub(file, *method))
                .filter(|method| {
                    let Some(name) = method.child_by_field_name("name") else {
                        return false;
                    };
                    trait_methods.iter().any(|parent| {
                        parent
                            .child_by_field_name("name")
                            .is_some_and(|parent_name| {
                                file.text(parent_name) == file.text(name)
                                    && parent.child_by_field_name("body").is_some()
                            })
                    })
                })
                .map(|method| file.line(method))
                .collect();
            lines.sort_unstable();
            lines.dedup();
            if !lines.is_empty() {
                let name = model.type_name(file, target_type).unwrap_or(target_name);
                findings.push(Finding {
                    smell_name: SMELL_NAME.to_owned(),
                    file_path: file.rel.clone(),
                    class_name: name.to_owned(),
                    line_numbers: lines,
                    relative_file_path: file.rel.clone(),
                    additional_info: String::new(),
                });
            }
        }
    }
    findings
}

fn rust_is_refusal_stub(file: &SourceFile, method: Node<'_>) -> bool {
    let Some(statements) = model_for(file.language).body_children(method) else {
        return false;
    };
    statements.len() == 1
        && statements[0].kind() == "macro_invocation"
        && (file.text(statements[0]).starts_with("todo!")
            || file.text(statements[0]).starts_with("unimplemented!"))
}

struct EcmaClass<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
    name: String,
}

impl EcmaClass<'_> {
    fn parent_name(&self) -> Option<&str> {
        let heritage = descendants(self.node)
            .into_iter()
            .find(|node| node.kind() == "class_heritage")?;
        descendants(heritage)
            .into_iter()
            .find(|node| matches!(node.kind(), "type_identifier" | "identifier"))
            .map(|node| self.file.text(node))
    }

    fn refusal_lines(&self, parent: &Self) -> Vec<usize> {
        let model = model_for(self.file.language);
        let child_methods = model.methods(self.file, self.node);
        let parent_methods = model.methods(parent.file, parent.node);
        let mut lines: Vec<_> = child_methods
            .into_iter()
            .filter(|method| self.refuses(*method, parent, &parent_methods))
            .map(|method| model.decl_line(method))
            .collect();
        lines.sort_unstable();
        lines.dedup();
        lines
    }

    fn refuses(&self, method: Node<'_>, parent: &Self, parent_methods: &[Node<'_>]) -> bool {
        if !(ecma_empty_body(self.file, method) || ecma_refusal_throw(self.file, method)) {
            return false;
        }
        let Some(name) = method.child_by_field_name("name") else {
            return false;
        };
        let model = model_for(self.file.language);
        parent_methods.iter().any(|parent_method| {
            parent_method
                .child_by_field_name("name")
                .is_some_and(|parent_name| {
                    parent.file.text(parent_name) == self.file.text(name)
                        && model.parameters(*parent_method) == model.parameters(method)
                })
        })
    }
}

fn ecmascript_findings(files: &[SourceFile]) -> Vec<Finding> {
    let classes = ecmascript_classes(files);
    classes
        .iter()
        .filter_map(|child| ecmascript_finding(child, &classes))
        .collect()
}

fn ecmascript_classes(files: &[SourceFile]) -> Vec<EcmaClass<'_>> {
    files
        .iter()
        .filter(|file| {
            matches!(
                file.language,
                LanguageId::JavaScript | LanguageId::TypeScript | LanguageId::Tsx
            )
        })
        .flat_map(|file| {
            model_for(file.language)
                .types(file)
                .into_iter()
                .filter(|node| {
                    matches!(
                        node.kind(),
                        "class_declaration" | "abstract_class_declaration"
                    )
                })
                .filter_map(move |node| {
                    model_for(file.language)
                        .type_name(file, node)
                        .map(|name| EcmaClass {
                            file,
                            node,
                            name: name.to_owned(),
                        })
                })
        })
        .collect()
}

fn ecmascript_finding(child: &EcmaClass<'_>, classes: &[EcmaClass<'_>]) -> Option<Finding> {
    let parent_name = child.parent_name()?;
    let parents: Vec<_> = classes
        .iter()
        .filter(|parent| parent.name == parent_name)
        .collect();
    if parents.len() != 1 {
        return None;
    }
    let lines = child.refusal_lines(parents[0]);
    (!lines.is_empty()).then(|| Finding {
        smell_name: SMELL_NAME.to_owned(),
        file_path: child.file.rel.clone(),
        class_name: child.name.clone(),
        line_numbers: lines,
        relative_file_path: child.file.rel.clone(),
        additional_info: String::new(),
    })
}

fn ecma_empty_body(_file: &SourceFile, method: Node<'_>) -> bool {
    method.child_by_field_name("body").is_some_and(|body| {
        let mut cursor = body.walk();
        body.named_children(&mut cursor)
            .all(|node| node.kind() == "comment")
    })
}

fn ecma_refusal_throw(file: &SourceFile, method: Node<'_>) -> bool {
    let Some(statements) = model_for(file.language).body_statements(method) else {
        return false;
    };
    statements.len() == 1
        && statements[0].kind() == "throw_statement"
        && (file.text(statements[0]).contains("UnsupportedOperation")
            || file.text(statements[0]).contains("NotImplemented"))
}

fn finding_for_type(project: &Project<'_>, index: usize) -> Option<Finding> {
    let ty = &project.types[index];
    if ty.node.kind() == "enum_declaration" {
        return None;
    }
    let parents = project.parents(index);
    if parents.is_empty() {
        return None;
    }
    let methods = model_for(LanguageId::Java).methods(ty.file, ty.node);
    let context = ChildContext {
        project,
        child: index,
        methods: methods.clone(),
    };
    let mut lines = parent_empty_method_findings(&context, &parents);
    for method in &methods {
        if has_override(ty.file, *method)
            && method.child_by_field_name("body").is_some()
            && (is_empty_body(ty.file, *method) || is_refusal_throw(ty.file, *method))
        {
            lines.push(model_for(LanguageId::Java).decl_line(*method));
        }
    }
    for parent in parents {
        if implicit_refusal(&context, parent) {
            lines.push(model_for(LanguageId::Java).decl_line(ty.node));
        }
    }
    lines.sort_unstable();
    (!lines.is_empty()).then(|| Finding {
        smell_name: SMELL_NAME.to_string(),
        file_path: ty.file.rel.clone(),
        class_name: ty.name.clone(),
        line_numbers: lines,
        relative_file_path: ty.file.rel.clone(),
        additional_info: String::new(),
    })
}

fn parent_empty_method_findings(context: &ChildContext<'_, '_>, parents: &[usize]) -> Vec<usize> {
    let project = context.project;
    parents
        .iter()
        .flat_map(|parent| {
            project
                .methods(*parent)
                .into_iter()
                .filter_map(move |method| {
                    let file = project.types[*parent].file;
                    (is_empty_body(file, method)
                        && !has_override(file, method)
                        && !overrides(context, *parent, method))
                    .then(|| {
                        model_for(LanguageId::Java).decl_line(project.types[context.child].node)
                    })
                })
        })
        .collect()
}

impl<'a> Project<'a> {
    fn new(files: &'a [SourceFile]) -> Self {
        let mut types = Vec::new();
        for file in files
            .iter()
            .filter(|file| file.language == LanguageId::Java)
        {
            let package = package_name(file);
            for node in model_for(LanguageId::Java).types(file) {
                if let Some(name) = model_for(LanguageId::Java).type_name(file, node) {
                    types.push(TypeInfo {
                        file,
                        node,
                        name: name.to_owned(),
                        package: package.clone(),
                    });
                }
            }
        }
        let mut by_name = BTreeMap::<String, Vec<usize>>::new();
        for (index, ty) in types.iter().enumerate() {
            by_name.entry(ty.name.clone()).or_default().push(index);
        }
        Self { types, by_name }
    }

    fn parents(&self, index: usize) -> Vec<usize> {
        let ty = &self.types[index];
        let mut parents = Vec::new();
        if let Some(superclass) = ty.node.child_by_field_name("superclass") {
            let reference = descendants(superclass)
                .into_iter()
                .find(|node| matches!(node.kind(), "type_identifier" | "scoped_type_identifier"));
            if let Some(parent) =
                reference.and_then(|reference| self.resolve_reference(index, reference))
            {
                parents.push(parent);
            }
        }
        if let Some(interfaces) = ty.node.child_by_field_name("interfaces") {
            for reference in descendants(interfaces)
                .into_iter()
                .filter(|node| matches!(node.kind(), "type_identifier" | "scoped_type_identifier"))
            {
                if let Some(parent) = self.resolve_reference(index, reference)
                    && !parents.contains(&parent)
                {
                    parents.push(parent);
                }
            }
        }
        parents
    }

    fn resolve_reference(&self, from: usize, reference: Node<'_>) -> Option<usize> {
        let name = self.types[from].file.text(reference).rsplit('.').next()?;
        let candidates = self.by_name.get(name)?;
        candidates
            .iter()
            .copied()
            .find(|candidate| self.types[*candidate].package == self.types[from].package)
            .or_else(|| (candidates.len() == 1).then_some(candidates[0]))
    }

    fn methods(&self, index: usize) -> Vec<Node<'a>> {
        model_for(LanguageId::Java).methods(self.types[index].file, self.types[index].node)
    }
}

fn overrides(context: &ChildContext<'_, '_>, parent: usize, parent_method: Node<'_>) -> bool {
    let project = context.project;
    let parent_file = project.types[parent].file;
    context.methods.iter().any(|child_method| {
        signature(project.types[context.child].file, *child_method)
            == signature(parent_file, parent_method)
    })
}

fn signature(file: &SourceFile, method: Node<'_>) -> String {
    let name = method
        .child_by_field_name("name")
        .map(|node| file.text(node))
        .unwrap_or_default();
    let params = method
        .child_by_field_name("parameters")
        .map(|node| file.text(node))
        .unwrap_or_default();
    format!("{name}{params}")
}

fn implicit_refusal(context: &ChildContext<'_, '_>, parent: usize) -> bool {
    let project = context.project;
    let inherited: Vec<_> = project
        .methods(parent)
        .into_iter()
        .filter(|method| {
            (has_modifier(project.types[parent].file, *method, "public")
                || has_modifier(project.types[parent].file, *method, "protected"))
                && !is_object_method(project.types[parent].file, *method)
        })
        .collect();
    if inherited.len() < 3 {
        return false;
    }
    let child_file = project.types[context.child].file;
    let child_calls: Vec<_> = descendants(project.types[context.child].node)
        .into_iter()
        .filter(|node| node.kind() == "method_invocation")
        .collect();
    let mut used = 0;
    for parent_method in &inherited {
        let parent_sig = signature(project.types[parent].file, *parent_method);
        if context
            .methods
            .iter()
            .any(|method| signature(child_file, *method) == parent_sig)
            || child_calls.iter().any(|call| {
                call_matches(
                    child_file,
                    *call,
                    ParentMethod {
                        node: *parent_method,
                        file: project.types[parent].file,
                    },
                )
            })
        {
            used += 1;
        }
    }
    let ratio = used as f64 / inherited.len() as f64;
    let new_methods = context
        .methods
        .iter()
        .filter(|method| !has_override(child_file, **method))
        .count();
    ratio < 0.34 && new_methods >= 3
}

fn call_matches(child_file: &SourceFile, call: Node<'_>, parent: ParentMethod<'_>) -> bool {
    let Some(name) = call.child_by_field_name("name") else {
        return false;
    };
    let Some(parent_name) = parent.node.child_by_field_name("name") else {
        return false;
    };
    if child_file.text(name) != parent.file.text(parent_name) {
        return false;
    }
    let argument_count = call
        .child_by_field_name("arguments")
        .map_or(0, |node| node.named_child_count());
    let parameter_count = parent
        .node
        .child_by_field_name("parameters")
        .map_or(0, |node| node.named_child_count());
    argument_count == parameter_count
}

fn is_empty_body(file: &SourceFile, method: Node<'_>) -> bool {
    if method
        .child_by_field_name("type")
        .is_none_or(|ty| file.text(ty) != "void")
    {
        return false;
    }
    let Some(statements) = model_for(file.language).body_statements(method) else {
        return false;
    };
    statements.is_empty()
        || (statements.len() == 1
            && statements[0].kind() == "return_statement"
            && statements[0].named_child_count() == 0)
}

fn is_refusal_throw(file: &SourceFile, method: Node<'_>) -> bool {
    let Some(statements) = model_for(file.language).body_statements(method) else {
        return false;
    };
    statements.len() == 1
        && statements[0].kind() == "throw_statement"
        && (file.text(statements[0]).contains("UnsupportedOperation")
            || file.text(statements[0]).contains("NotImplemented"))
}

fn has_override(file: &SourceFile, method: Node<'_>) -> bool {
    descendants(method)
        .into_iter()
        .any(|node| node.kind() == "marker_annotation" && file.text(node).contains("Override"))
}

fn is_object_method(file: &SourceFile, method: Node<'_>) -> bool {
    method.child_by_field_name("name").is_some_and(|name| {
        matches!(
            file.text(name),
            "equals"
                | "hashCode"
                | "toString"
                | "clone"
                | "getClass"
                | "notify"
                | "notifyAll"
                | "wait"
        )
    })
}

fn package_name(file: &SourceFile) -> String {
    let mut cursor = file.root().walk();
    file.root()
        .named_children(&mut cursor)
        .find(|node| node.kind() == "package_declaration")
        .map(|node| {
            file.text(node)
                .trim_start_matches("package")
                .trim_end_matches(';')
                .trim()
                .to_owned()
        })
        .unwrap_or_default()
}

fn has_modifier(file: &SourceFile, node: Node<'_>, modifier: &str) -> bool {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
        .is_some_and(|modifiers| {
            file.text(modifiers)
                .split_whitespace()
                .any(|word| word == modifier)
        })
}
