//! Port of StaticNose's direct access to another project type's fields.

use std::collections::{BTreeMap, BTreeSet};

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::descendants;
use tree_sitter::Node;

pub const SMELL_NAME: &str = "Inappropriate Intimacy";

struct TypeInfo<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
    name: String,
    package: String,
    is_enum: bool,
}

struct Analyzer<'a> {
    types: Vec<TypeInfo<'a>>,
    by_name: BTreeMap<String, Vec<usize>>,
}

struct AccessContext {
    owner: usize,
    vars: BTreeMap<String, String>,
}

pub fn detect(files: &[SourceFile]) -> Vec<Finding> {
    let analysis = Analyzer::new(files);
    let mut findings = Vec::new();
    for owner in 0..analysis.types.len() {
        let ty = &analysis.types[owner];
        let model = model_for(LanguageId::Java);
        let Some(class_name) = model.type_name(ty.file, ty.node) else {
            continue;
        };
        let mut lines = Vec::new();
        for access in descendants(ty.node)
            .into_iter()
            .filter(|node| node.kind() == "field_access")
        {
            let mut callable = access.parent();
            while let Some(parent) = callable {
                if matches!(
                    parent.kind(),
                    "method_declaration" | "constructor_declaration" | "class_declaration"
                ) {
                    break;
                }
                callable = parent.parent();
            }
            let callable = callable.unwrap_or(ty.node);
            let context = AccessContext {
                owner,
                vars: analysis.variable_types(owner, callable),
            };
            if analysis.is_external_field_access(&context, access) {
                lines.push(ty.file.line(access));
            }
        }
        if !lines.is_empty() {
            findings.push(Finding {
                smell_name: SMELL_NAME.to_string(),
                file_path: ty.file.rel.clone(),
                class_name: class_name.to_string(),
                line_numbers: lines,
                relative_file_path: ty.file.rel.clone(),
                additional_info: String::new(),
            });
        }
    }
    findings.extend(non_java_findings(files));
    findings
}

struct NonJavaType<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
    name: String,
    fields: BTreeMap<String, String>,
}

fn non_java_findings(files: &[SourceFile]) -> Vec<Finding> {
    let mut types = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.language != LanguageId::Java)
    {
        for node in model_for(file.language).types(file) {
            if matches!(node.kind(), "source_file" | "program") {
                continue;
            }
            let Some(name) = model_for(file.language).type_name(file, node) else {
                continue;
            };
            let fields = non_java_field_types(file, node);
            types.push(NonJavaType {
                file,
                node,
                name: name.to_owned(),
                fields,
            });
        }
    }
    let known: BTreeSet<_> = types.iter().map(|ty| ty.name.clone()).collect();
    let mut findings = Vec::new();
    for owner in &types {
        let model = model_for(owner.file.language);
        let mut lines = Vec::new();
        for method in model.methods(owner.file, owner.node) {
            let vars = non_java_variable_types(owner, method);
            for access in descendants(method) {
                let Some((receiver, field)) = model.field_access_parts(access) else {
                    continue;
                };
                if matches!(receiver.kind(), "this")
                    || owner.file.text(receiver) == "this"
                    || owner.file.text(receiver) == "self"
                {
                    continue;
                }
                let Some(target_type) = expression_type(owner.file, receiver, &vars) else {
                    continue;
                };
                let target_name = simple_type_name(&target_type);
                if target_name == owner.name || !known.contains(&target_name) {
                    continue;
                }
                if types.iter().any(|ty| {
                    ty.name == target_name && ty.fields.contains_key(owner.file.text(field))
                }) {
                    lines.push(owner.file.line(access));
                }
            }
        }
        lines.sort_unstable();
        lines.dedup();
        if !lines.is_empty() {
            findings.push(Finding {
                smell_name: SMELL_NAME.to_owned(),
                file_path: owner.file.rel.clone(),
                class_name: owner.name.clone(),
                line_numbers: lines,
                relative_file_path: owner.file.rel.clone(),
                additional_info: String::new(),
            });
        }
    }
    findings
}

fn non_java_field_types(file: &SourceFile, ty: Node<'_>) -> BTreeMap<String, String> {
    let Some(body) = ty.child_by_field_name("body") else {
        return BTreeMap::new();
    };
    let mut fields = BTreeMap::new();
    if file.language == LanguageId::Rust {
        let mut cursor = body.walk();
        for field in body
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "field_declaration")
        {
            if let (Some(name), Some(kind)) = (
                field.child_by_field_name("name"),
                field.child_by_field_name("type"),
            ) {
                fields.insert(file.text(name).to_owned(), file.text(kind).to_owned());
            }
        }
    } else {
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
            let kind = field
                .child_by_field_name("type")
                .map(|node| file.text(node).to_owned())
                .unwrap_or_default();
            fields.insert(file.text(name).trim_start_matches('#').to_owned(), kind);
        }
    }
    fields
}

fn non_java_variable_types(owner: &NonJavaType<'_>, method: Node<'_>) -> BTreeMap<String, String> {
    let file = owner.file;
    let mut vars = owner.fields.clone();
    if let Some(parameters) = method.child_by_field_name("parameters") {
        for parameter in descendants(parameters).into_iter().filter(|node| {
            matches!(
                node.kind(),
                "parameter" | "required_parameter" | "optional_parameter" | "formal_parameter"
            )
        }) {
            let name_node = parameter
                .child_by_field_name("name")
                .or_else(|| parameter.child_by_field_name("pattern"));
            let Some(name_node) = name_node else {
                continue;
            };
            let Some(kind) = parameter.child_by_field_name("type") else {
                continue;
            };
            vars.insert(
                file.text(name_node)
                    .trim_start_matches('&')
                    .trim()
                    .to_owned(),
                file.text(kind).to_owned(),
            );
        }
    }
    for declaration in descendants(method) {
        if declaration.kind() == "variable_declarator" {
            let (Some(name), Some(value)) = (
                declaration.child_by_field_name("name"),
                declaration.child_by_field_name("value"),
            ) else {
                continue;
            };
            if value.kind() == "new_expression"
                && let Some(constructor) = value.child_by_field_name("constructor")
            {
                vars.insert(
                    file.text(name).to_owned(),
                    file.text(constructor).to_owned(),
                );
            }
        } else if declaration.kind() == "let_declaration" {
            let Some(pattern) = declaration.child_by_field_name("pattern") else {
                continue;
            };
            let Some(value) = declaration.child_by_field_name("value") else {
                continue;
            };
            if value.kind() == "call_expression"
                && let Some(name) = value.child_by_field_name("function")
            {
                vars.insert(file.text(pattern).to_owned(), file.text(name).to_owned());
            }
        }
    }
    vars
}

fn expression_type(
    file: &SourceFile,
    expression: Node<'_>,
    vars: &BTreeMap<String, String>,
) -> Option<String> {
    match expression.kind() {
        "identifier" => vars.get(file.text(expression)).cloned(),
        "reference_type" => Some(file.text(expression).trim_start_matches('&').to_owned()),
        _ => None,
    }
}

fn simple_type_name(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('&')
        .trim_start_matches("mut ")
        .trim_start_matches(':')
        .trim()
        .trim_end_matches("[]")
        .rsplit("::")
        .next()
        .unwrap_or(raw)
        .rsplit('.')
        .next()
        .unwrap_or(raw)
        .trim()
        .to_owned()
}

impl<'a> Analyzer<'a> {
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
                        is_enum: node.kind() == "enum_declaration",
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

    fn variable_types(&self, owner: usize, method: Node<'a>) -> BTreeMap<String, String> {
        let mut variables = BTreeMap::new();
        for field in field_decls(self.types[owner].node) {
            let Some(field_type) = field.child_by_field_name("type") else {
                continue;
            };
            let mut cursor = field.walk();
            for variable in field
                .named_children(&mut cursor)
                .filter(|node| node.kind() == "variable_declarator")
            {
                if let Some(name) = variable.child_by_field_name("name") {
                    variables.insert(
                        self.types[owner].file.text(name).to_owned(),
                        self.types[owner].file.text(field_type).to_owned(),
                    );
                }
            }
        }
        let file = self.types[owner].file;
        if let Some(parameters) = method.child_by_field_name("parameters") {
            for parameter in descendants(parameters)
                .into_iter()
                .filter(|node| matches!(node.kind(), "formal_parameter" | "spread_parameter"))
            {
                if let (Some(name), Some(ty)) = (
                    parameter.child_by_field_name("name"),
                    parameter.child_by_field_name("type"),
                ) {
                    variables.insert(file.text(name).to_owned(), file.text(ty).to_owned());
                }
            }
        }
        for loop_node in descendants(method)
            .into_iter()
            .filter(|node| node.kind() == "enhanced_for_statement")
        {
            if let (Some(name), Some(ty)) = (
                loop_node.child_by_field_name("name"),
                loop_node.child_by_field_name("type"),
            ) {
                variables.insert(file.text(name).to_owned(), file.text(ty).to_owned());
            }
        }
        for declaration in descendants(method)
            .into_iter()
            .filter(|node| node.kind() == "local_variable_declaration")
        {
            let Some(ty) = declaration.child_by_field_name("type") else {
                continue;
            };
            let mut cursor = declaration.walk();
            for variable in declaration
                .named_children(&mut cursor)
                .filter(|node| node.kind() == "variable_declarator")
            {
                if let Some(name) = variable.child_by_field_name("name") {
                    variables.insert(file.text(name).to_owned(), file.text(ty).to_owned());
                }
            }
        }
        variables
    }

    fn is_external_field_access(&self, context: &AccessContext, access: Node<'a>) -> bool {
        let ty = &self.types[context.owner];
        let Some(target) = access.child_by_field_name("object") else {
            return false;
        };
        let Some(field) = access.child_by_field_name("field") else {
            return false;
        };
        let target_type = self.expression_type(context, target, 0);
        let Some(target_index) =
            target_type.and_then(|name| self.resolve_type(context.owner, &name))
        else {
            return false;
        };
        if target_index == context.owner
            || self.same_top_level(context.owner, target_index)
            || self.types[target_index].is_enum
        {
            return false;
        }
        let target_decl = &self.types[target_index];
        self.field_declarations(target_index)
            .into_iter()
            .any(|declaration| {
                if has_modifier(target_decl.file, declaration, "static")
                    && has_modifier(target_decl.file, declaration, "final")
                {
                    return false;
                }
                let mut cursor = declaration.walk();
                declaration
                    .named_children(&mut cursor)
                    .filter(|node| node.kind() == "variable_declarator")
                    .any(|variable| {
                        variable
                            .child_by_field_name("name")
                            .is_some_and(|name| target_decl.file.text(name) == ty.file.text(field))
                    })
            })
    }

    fn expression_type(
        &self,
        context: &AccessContext,
        expression: Node<'a>,
        depth: usize,
    ) -> Option<String> {
        if depth > 6 {
            return None;
        }
        let file = self.types[context.owner].file;
        if expression.kind() == "this" || expression.kind() == "super" {
            return Some(self.types[context.owner].name.clone());
        }
        if expression.kind() == "identifier" {
            let text = file.text(expression);
            return context
                .vars
                .get(text)
                .cloned()
                .or_else(|| self.by_name.contains_key(text).then(|| text.to_owned()));
        }
        if expression.kind() == "field_access" {
            return self.field_access_type(context, expression, depth);
        }
        if expression.kind() == "object_creation_expression" {
            return expression
                .child_by_field_name("type")
                .map(|node| file.text(node).to_owned());
        }
        if expression.kind() == "method_invocation" {
            return self.invocation_type(context, expression, depth);
        }
        None
    }

    fn field_access_type(
        &self,
        context: &AccessContext,
        expression: Node<'a>,
        depth: usize,
    ) -> Option<String> {
        let file = self.types[context.owner].file;
        let field = expression.child_by_field_name("field")?;
        let target = expression.child_by_field_name("object")?;
        let target_type = self.expression_type(context, target, depth + 1)?;
        let target_index = self.resolve_type(context.owner, &target_type)?;
        let target_decl = &self.types[target_index];
        self.field_declarations(target_index)
            .into_iter()
            .find_map(|decl| {
                let field_type = decl.child_by_field_name("type")?;
                let mut cursor = decl.walk();
                let found = decl
                    .named_children(&mut cursor)
                    .filter(|node| node.kind() == "variable_declarator")
                    .any(|variable| {
                        variable
                            .child_by_field_name("name")
                            .is_some_and(|name| target_decl.file.text(name) == file.text(field))
                    });
                found.then(|| target_decl.file.text(field_type).to_owned())
            })
    }

    fn invocation_type(
        &self,
        context: &AccessContext,
        expression: Node<'a>,
        depth: usize,
    ) -> Option<String> {
        let file = self.types[context.owner].file;
        let name = expression
            .child_by_field_name("name")
            .map(|node| file.text(node))?;
        if let Some(generic_type) = self.generic_invocation_type(context, name, expression) {
            return Some(generic_type);
        }
        let target_type = expression
            .child_by_field_name("object")
            .and_then(|target| self.expression_type(context, target, depth + 1))
            .unwrap_or_else(|| self.types[context.owner].name.clone());
        let target_index = self.resolve_type(context.owner, &target_type)?;
        model_for(LanguageId::Java)
            .methods(self.types[target_index].file, self.types[target_index].node)
            .into_iter()
            .find_map(|candidate| {
                let candidate_name = candidate.child_by_field_name("name")?;
                (self.types[target_index].file.text(candidate_name) == name)
                    .then(|| {
                        candidate
                            .child_by_field_name("type")
                            .map(|ty| self.types[target_index].file.text(ty).to_owned())
                    })
                    .flatten()
            })
    }

    fn generic_invocation_type(
        &self,
        context: &AccessContext,
        name: &str,
        expression: Node<'a>,
    ) -> Option<String> {
        if name == "get" {
            let receiver = expression.child_by_field_name("object")?;
            return context
                .vars
                .get(self.types[context.owner].file.text(receiver))?
                .split(',')
                .nth(1)
                .map(|value| value.trim().trim_end_matches('>').to_owned());
        }
        if name == "getValue" {
            return context
                .vars
                .values()
                .find(|value| value.contains("Map.Entry<"))?
                .split(',')
                .nth(1)
                .map(|value| value.trim().trim_end_matches('>').to_owned());
        }
        None
    }

    fn resolve_type(&self, owner: usize, type_name: &str) -> Option<usize> {
        let simple = type_name
            .trim_end_matches("[]")
            .rsplit('.')
            .next()
            .unwrap_or(type_name);
        let candidates = self.by_name.get(simple)?;
        candidates
            .iter()
            .copied()
            .find(|candidate| self.types[*candidate].package == self.types[owner].package)
            .or_else(|| (candidates.len() == 1).then_some(candidates[0]))
    }

    fn field_declarations(&self, type_index: usize) -> Vec<Node<'a>> {
        let ty = &self.types[type_index];
        let mut fields = field_decls(ty.node);
        let Some(superclass) = ty.node.child_by_field_name("superclass") else {
            return fields;
        };
        let parent_name = descendants(superclass)
            .into_iter()
            .find(|node| matches!(node.kind(), "type_identifier" | "scoped_type_identifier"))
            .map(|node| ty.file.text(node).to_owned());
        if let Some(parent) = parent_name.and_then(|name| self.resolve_type(type_index, &name))
            && parent != type_index
        {
            fields.extend(self.field_declarations(parent));
        }
        fields
    }

    fn same_top_level(&self, left: usize, right: usize) -> bool {
        let top_level = |node: Node<'a>| {
            let mut result = node;
            let mut parent = node.parent();
            while let Some(ancestor) = parent {
                if matches!(
                    ancestor.kind(),
                    "class_declaration"
                        | "interface_declaration"
                        | "enum_declaration"
                        | "record_declaration"
                ) {
                    result = ancestor;
                }
                parent = ancestor.parent();
            }
            result
        };
        self.types[left].file.path == self.types[right].file.path
            && top_level(self.types[left].node).id() == top_level(self.types[right].node).id()
    }
}

fn field_decls(ty: Node<'_>) -> Vec<Node<'_>> {
    let Some(body) = ty.child_by_field_name("body") else {
        return Vec::new();
    };
    let mut cursor = body.walk();
    body.named_children(&mut cursor)
        .filter(|node| node.kind() == "field_declaration")
        .collect()
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
