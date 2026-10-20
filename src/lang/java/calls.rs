//! Source-only Java call resolution for coupling metrics. Unresolved or
//! ambiguous targets are omitted rather than joined by simple method name.
//! This is not a Java compiler: classpath types and dynamic dispatch are absent.

use std::collections::{BTreeSet, HashSet};

use tree_sitter::Node;

use crate::lang::coupling::{self, Callable, MethodCoupling, MethodLocation};
use crate::lang::{LanguageId, SourceFile, model_for};
use crate::walk::{ancestor_while, descendants};

struct Type<'a> {
    file: &'a SourceFile,
    node: Node<'a>,
    name: String,
    qualified: String,
    package: String,
}

struct Method<'a> {
    owner: usize,
    node: Node<'a>,
    name: String,
    parameters: Vec<String>,
}

struct Project<'a> {
    types: Vec<Type<'a>>,
    methods: Vec<Method<'a>>,
}

type ExpressionReader<'query, 'source> =
    fn(&JavaExpressionResolver<'query, 'source>, Node<'source>) -> Option<String>;

#[derive(Clone, Copy)]
struct CallSite<'a> {
    owner: usize,
    method: Node<'a>,
    depth: usize,
}

impl CallSite<'_> {
    fn deeper(self) -> Self {
        Self {
            depth: self.depth + 1,
            ..self
        }
    }
}

/// Constructors are deliberately absent, as in StaticNose. Interface methods
/// participate in resolution but are not emitted as reportable methods.
pub fn coupling(files: &[SourceFile]) -> Vec<MethodCoupling> {
    let project = Project::new(files);
    let mut edges = Vec::new();
    for (caller, method) in project.methods.iter().enumerate() {
        if method.node.child_by_field_name("body").is_none() {
            continue;
        }
        for call in descendants(method.node)
            .into_iter()
            .filter(|node| node.kind() == "method_invocation")
        {
            let site = CallSite {
                owner: method.owner,
                method: method.node,
                depth: 0,
            };
            let Some(target) = project.resolve_call(site, call) else {
                continue;
            };
            edges.push((caller, target));
        }
    }
    let methods: Vec<_> = project
        .methods
        .iter()
        .map(|method| {
            let ty = &project.types[method.owner];
            Callable {
                owner: method.owner,
                location: MethodLocation {
                    file_path: ty.file.rel.clone(),
                    class_name: ty.name.clone(),
                    line: model_for(LanguageId::Java).decl_line(method.node),
                },
                reportable: ty.node.kind() != "interface_declaration"
                    && method.node.child_by_field_name("body").is_some()
                    && !project.interface_contract(method),
            }
        })
        .collect();
    coupling::count(&methods, &edges)
}

impl<'a> Project<'a> {
    fn expression_type(&self, site: CallSite<'a>, expression: Node<'a>) -> Option<String> {
        JavaExpressionResolver {
            project: self,
            site,
        }
        .expression_type(expression)
    }

    fn type_resolver(&self) -> TypeResolver<'_, 'a> {
        TypeResolver { types: &self.types }
    }

    fn new(files: &'a [SourceFile]) -> Self {
        let mut types = Vec::new();
        for file in files
            .iter()
            .filter(|file| file.language == LanguageId::Java)
        {
            let package = package_name(file);
            for node in descendants(file.root())
                .into_iter()
                .filter(|node| is_type(*node))
            {
                let Some(name) = node.child_by_field_name("name") else {
                    continue;
                };
                let mut names = vec![file.text(name)];
                let mut parent = node.parent();
                while let Some(ancestor) = parent {
                    if is_type(ancestor)
                        && let Some(name) = ancestor.child_by_field_name("name")
                    {
                        names.push(file.text(name));
                    }
                    parent = ancestor.parent();
                }
                names.reverse();
                let nested = names.join(".");
                let qualified = qualify(&package, &nested);
                types.push(Type {
                    file,
                    node,
                    name: file.text(name).into(),
                    qualified,
                    package: package.clone(),
                });
            }
        }
        let mut project = Self {
            types,
            methods: Vec::new(),
        };
        for (owner, ty) in project.types.iter().enumerate() {
            for node in model_for(LanguageId::Java).methods(ty.file.at(ty.node)) {
                let Some(name) = node.child_by_field_name("name") else {
                    continue;
                };
                let parameters = node
                    .child_by_field_name("parameters")
                    .map(|params| {
                        named_children(params)
                            .into_iter()
                            .filter_map(|p| p.child_by_field_name("type"))
                            .map(|p| project.type_resolver().type_name(owner, ty.file.text(p)))
                            .collect()
                    })
                    .unwrap_or_default();
                project.methods.push(Method {
                    owner,
                    node,
                    name: ty.file.text(name).into(),
                    parameters,
                });
            }
        }
        project
    }

    fn interface_contract(&self, method: &Method<'_>) -> bool {
        self.type_resolver()
            .hierarchy(method.owner)
            .into_iter()
            .any(|owner| {
                self.types[owner].node.kind() == "interface_declaration"
                    && self.methods.iter().any(|other| {
                        other.owner == owner
                            && other.name == method.name
                            && other.parameters == method.parameters
                    })
            })
    }

    fn resolve_call(&self, site: CallSite<'a>, call: Node<'a>) -> Option<usize> {
        if site.depth > 24 {
            return None;
        }
        let owner = site.owner;
        let file = self.types[owner].file;
        let name = file.text(call.child_by_field_name("name")?);
        let receiver = if let Some(object) = call.child_by_field_name("object") {
            self.expression_type(site.deeper(), object)?
        } else {
            self.types[owner].qualified.clone()
        };
        let receiver = self.type_resolver().type_index(&receiver)?;
        let arguments = named_children(call.child_by_field_name("arguments")?);
        let argument_types: Vec<_> = arguments
            .into_iter()
            .map(|arg| self.expression_type(site.deeper(), arg))
            .collect();
        let mut best = None;
        let mut ambiguous = false;
        let mut signatures = BTreeSet::new();
        for target_owner in self.type_resolver().hierarchy(receiver) {
            for (index, target) in self.methods.iter().enumerate().filter(|(_, m)| {
                m.owner == target_owner
                    && m.name == name
                    && m.parameters.len() == argument_types.len()
            }) {
                if !signatures.insert(target.parameters.clone()) {
                    continue;
                }
                let Some(score) = self.argument_score(&target.parameters, &argument_types) else {
                    continue;
                };
                if best.is_some_and(|(_, previous)| score == previous) {
                    ambiguous = true;
                } else if best.is_none_or(|(_, previous)| score > previous) {
                    best = Some((index, score));
                    ambiguous = false;
                }
            }
        }
        if ambiguous {
            None
        } else {
            best.map(|(index, _)| index)
        }
    }

    fn argument_score(&self, parameters: &[String], arguments: &[Option<String>]) -> Option<usize> {
        let mut score = 0;
        for (parameter, argument) in parameters.iter().zip(arguments) {
            let Some(argument) = argument else { continue };
            if parameter == argument {
                score += 2;
            } else if self
                .type_resolver()
                .type_index(argument)
                .is_some_and(|owner| {
                    self.type_resolver()
                        .hierarchy(owner)
                        .iter()
                        .any(|p| self.types[*p].qualified == *parameter)
                })
                || numeric_widening(argument, parameter)
                || (argument == "null" && !is_primitive(parameter))
            {
                score += 1;
            } else {
                return None;
            }
        }
        Some(score)
    }
}

#[derive(Clone, Copy)]
struct JavaExpressionResolver<'query, 'source> {
    project: &'query Project<'source>,
    site: CallSite<'source>,
}

impl<'a> JavaExpressionResolver<'_, 'a> {
    fn deeper(self) -> Self {
        Self {
            site: self.site.deeper(),
            ..self
        }
    }

    fn expression_type(&self, expression: Node<'a>) -> Option<String> {
        if self.site.depth > 24 {
            return None;
        }
        let readers: &[(&str, ExpressionReader<'_, 'a>)] = &[
            ("scoped_identifier", Self::qualified_expression_type),
            ("this", Self::receiver_expression_type),
            ("super", Self::receiver_expression_type),
            ("identifier", Self::identifier_expression_type),
            ("object_creation_expression", Self::declared_expression_type),
            ("cast_expression", Self::declared_expression_type),
            (
                "parenthesized_expression",
                Self::parenthesized_expression_type,
            ),
            ("field_access", Self::field_expression_type),
            ("method_invocation", Self::call_expression_type),
        ];
        if let Some((_, read)) = readers.iter().find(|(kind, _)| *kind == expression.kind()) {
            read(self, expression)
        } else {
            literal_type(self.project.types[self.site.owner].file, expression)
        }
    }

    fn qualified_expression_type(&self, expression: Node<'a>) -> Option<String> {
        let name = self.project.type_resolver().type_name(
            self.site.owner,
            self.project.types[self.site.owner].file.text(expression),
        );
        self.project.type_resolver().type_index(&name).map(|_| name)
    }

    fn receiver_expression_type(&self, expression: Node<'a>) -> Option<String> {
        if expression.kind() == "this" {
            Some(self.project.types[self.site.owner].qualified.clone())
        } else {
            self.project
                .type_resolver()
                .parents(self.site.owner)
                .first()
                .map(|p| self.project.types[*p].qualified.clone())
        }
    }

    fn identifier_expression_type(&self, expression: Node<'a>) -> Option<String> {
        self.deeper()
            .variable_type(expression)
            .or_else(|| self.qualified_expression_type(expression))
    }

    fn declared_expression_type(&self, expression: Node<'a>) -> Option<String> {
        expression.child_by_field_name("type").map(|n| {
            self.project.type_resolver().type_name(
                self.site.owner,
                self.project.types[self.site.owner].file.text(n),
            )
        })
    }

    fn parenthesized_expression_type(&self, expression: Node<'a>) -> Option<String> {
        named_children(expression)
            .first()
            .and_then(|n| self.deeper().expression_type(*n))
    }

    fn call_expression_type(&self, expression: Node<'a>) -> Option<String> {
        let target =
            &self.project.methods[self.project.resolve_call(self.site.deeper(), expression)?];
        let file = self.project.types[target.owner].file;
        target.node.child_by_field_name("type").map(|n| {
            self.project
                .type_resolver()
                .type_name(target.owner, file.text(n))
        })
    }

    fn field_expression_type(&self, expression: Node<'a>) -> Option<String> {
        let owner = self.site.owner;
        let file = self.project.types[owner].file;
        let qualified = self
            .project
            .type_resolver()
            .type_name(owner, file.text(expression));
        if self
            .project
            .type_resolver()
            .type_index(&qualified)
            .is_some()
        {
            return Some(qualified);
        }
        let object = expression.child_by_field_name("object")?;
        let receiver = self.deeper().expression_type(object)?;
        let target_owner = self.project.type_resolver().type_index(&receiver)?;
        self.project.type_resolver().field_type(
            target_owner,
            file.text(expression.child_by_field_name("field")?),
        )
    }

    fn variable_type(&self, usage: Node<'a>) -> Option<String> {
        let owner = self.site.owner;
        let file = self.project.types[owner].file;
        let name = file.text(usage);
        let mut scope = usage.parent();
        while let Some(block) = scope {
            let candidate = descendants(block)
                .into_iter()
                .filter(|n| {
                    matches!(
                        n.kind(),
                        "local_variable_declaration"
                            | "formal_parameter"
                            | "catch_formal_parameter"
                            | "enhanced_for_statement"
                    ) && n.start_byte() < usage.start_byte()
                        && binding_scope(*n).is_some_and(|s| s.id() == block.id())
                })
                .filter(|n| declares(file, *n, name))
                .max_by_key(|n| n.start_byte());
            if let Some(binding) = candidate {
                let declared = binding.child_by_field_name("type")?;
                if file.text(declared) != "var" {
                    return Some(
                        self.project
                            .type_resolver()
                            .type_name(owner, file.text(declared)),
                    );
                }
                return descendants(binding)
                    .into_iter()
                    .find(|n| n.kind() == "variable_declarator")
                    .and_then(|n| n.child_by_field_name("value"))
                    .and_then(|value| self.deeper().expression_type(value));
            }
            if block.id() == self.site.method.id() {
                break;
            }
            scope = block.parent();
        }
        self.project.type_resolver().field_type(owner, name)
    }
}

struct TypeResolver<'index, 'source> {
    types: &'index [Type<'source>],
}

impl<'a> TypeResolver<'_, 'a> {
    fn type_index(&self, name: &str) -> Option<usize> {
        let mut matches = self
            .types
            .iter()
            .enumerate()
            .filter(|(_, ty)| ty.qualified == name);
        let (index, _) = matches.next()?;
        matches.next().is_none().then_some(index)
    }

    fn type_name(&self, owner: usize, raw: &str) -> String {
        let raw = raw.split('<').next().unwrap_or(raw).trim();
        let ty = &self.types[owner];
        if self.type_index(raw).is_some() {
            return raw.into();
        }
        let mut enclosing = ty.qualified.as_str();
        loop {
            let candidate = qualify(enclosing, raw);
            if self.type_index(&candidate).is_some() {
                return candidate;
            }
            let Some((prefix, _)) = enclosing.rsplit_once('.') else {
                break;
            };
            enclosing = prefix;
        }
        for node in named_children(ty.file.root())
            .into_iter()
            .filter(|n| n.kind() == "import_declaration")
        {
            let import = ty
                .file
                .text(node)
                .trim_start_matches("import")
                .trim()
                .trim_end_matches(';')
                .trim();
            if import.starts_with("static ") {
                continue;
            }
            if let Some(prefix) = import.strip_suffix(".*") {
                let candidate = qualify(prefix, raw);
                if self.type_index(&candidate).is_some() {
                    return candidate;
                }
            } else if import.rsplit('.').next() == Some(raw) {
                return import.into();
            }
        }
        let candidate = qualify(&ty.package, raw);
        if self.type_index(&candidate).is_some() {
            candidate
        } else {
            raw.into()
        }
    }

    fn parents(&self, owner: usize) -> Vec<usize> {
        let ty = &self.types[owner];
        named_children(ty.node)
            .into_iter()
            .filter(|n| {
                matches!(
                    n.kind(),
                    "superclass" | "super_interfaces" | "extends_interfaces"
                )
            })
            .flat_map(type_references)
            .filter_map(|n| self.type_index(&self.type_name(owner, ty.file.text(n))))
            .collect()
    }

    fn hierarchy(&self, owner: usize) -> Vec<usize> {
        let mut seen = HashSet::new();
        let mut queue = vec![owner];
        let mut result = Vec::new();
        while let Some(next) = queue.pop() {
            if seen.insert(next) {
                result.push(next);
                queue.extend(self.parents(next));
            }
        }
        result
    }

    fn field_type(&self, owner: usize, name: &str) -> Option<String> {
        for parent in self.hierarchy(owner) {
            let ty = &self.types[parent];
            for field in descendants(ty.node)
                .into_iter()
                .filter(|n| n.kind() == "field_declaration")
            {
                if nearest_type(field).is_some_and(|n| n.id() == ty.node.id())
                    && declares(ty.file, field, name)
                {
                    return field
                        .child_by_field_name("type")
                        .map(|n| self.type_name(parent, ty.file.text(n)));
                }
            }
        }
        None
    }
}

fn named_children(node: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|n| !matches!(n.kind(), "line_comment" | "block_comment"))
        .collect()
}

fn is_type(node: Node<'_>) -> bool {
    matches!(
        node.kind(),
        "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration"
    )
}

fn nearest_type(node: Node<'_>) -> Option<Node<'_>> {
    ancestor_while(node, |_| false, is_type)
}

fn binding_scope(node: Node<'_>) -> Option<Node<'_>> {
    ancestor_while(
        node,
        |_| false,
        |parent| {
            matches!(
                parent.kind(),
                "block"
                    | "method_declaration"
                    | "for_statement"
                    | "enhanced_for_statement"
                    | "catch_clause"
                    | "lambda_expression"
            )
        },
    )
}

fn declares(file: &SourceFile, node: Node<'_>, name: &str) -> bool {
    node.child_by_field_name("name")
        .is_some_and(|n| file.text(n) == name)
        || named_children(node)
            .into_iter()
            .filter(|n| n.kind() == "variable_declarator")
            .any(|n| {
                n.child_by_field_name("name")
                    .is_some_and(|n| file.text(n) == name)
            })
}

fn type_references(node: Node<'_>) -> Vec<Node<'_>> {
    if matches!(
        node.kind(),
        "type_identifier" | "scoped_type_identifier" | "generic_type"
    ) {
        return vec![node];
    }
    named_children(node)
        .into_iter()
        .flat_map(type_references)
        .collect()
}

fn package_name(file: &SourceFile) -> String {
    named_children(file.root())
        .into_iter()
        .find(|n| n.kind() == "package_declaration")
        .and_then(|n| {
            named_children(n)
                .into_iter()
                .find(|n| matches!(n.kind(), "identifier" | "scoped_identifier"))
        })
        .map(|n| file.text(n).into())
        .unwrap_or_default()
}

fn qualify(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.into()
    } else {
        format!("{prefix}.{name}")
    }
}

fn is_primitive(name: &str) -> bool {
    matches!(
        name,
        "boolean" | "byte" | "short" | "char" | "int" | "long" | "float" | "double"
    )
}

fn numeric_widening(from: &str, to: &str) -> bool {
    const WIDENING: &[(&str, &[&str])] = &[
        ("byte", &["short", "int", "long", "float", "double"]),
        ("short", &["int", "long", "float", "double"]),
        ("char", &["int", "long", "float", "double"]),
        ("int", &["long", "float", "double"]),
        ("long", &["float", "double"]),
        ("float", &["double"]),
    ];
    WIDENING
        .iter()
        .find(|(name, _)| *name == from)
        .is_some_and(|(_, targets)| targets.contains(&to))
}

fn literal_type(file: &SourceFile, expression: Node<'_>) -> Option<String> {
    const LITERALS: &[(&str, &str)] = &[
        ("string_literal", "String"),
        ("character_literal", "char"),
        ("true", "boolean"),
        ("false", "boolean"),
        ("null_literal", "null"),
    ];
    if let Some((_, name)) = LITERALS.iter().find(|(kind, _)| *kind == expression.kind()) {
        return Some((*name).into());
    }
    let name = if expression.kind().ends_with("integer_literal") {
        if file.text(expression).ends_with(['l', 'L']) {
            "long"
        } else {
            "int"
        }
    } else if expression.kind().ends_with("floating_point_literal") {
        if file.text(expression).ends_with(['f', 'F']) {
            "float"
        } else {
            "double"
        }
    } else {
        return None;
    };
    Some(name.into())
}
