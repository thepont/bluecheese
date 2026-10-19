//! Bounded source-only ES-module resolution shared by JS, JSX, TS and TSX.
//! No modules are executed and no package loader or compiler is invoked.

use super::{
    children, enclosing_function, function_name, is_ecmascript, is_function, is_owner, owner,
    owner_name,
};
use crate::lang::{
    LanguageId, SourceFile,
    coupling::{self, Callable, MethodCoupling},
};
use crate::walk::descendants;
use std::collections::HashSet;
use std::path::{Component, Path};
use tree_sitter::Node;

struct Owner<'a> {
    file: usize,
    node: Node<'a>,
    name: String,
}
struct Method<'a> {
    owner: usize,
    node: Node<'a>,
    name: String,
}
struct Project<'a> {
    files: Vec<&'a SourceFile>,
    owners: Vec<Owner<'a>>,
    methods: Vec<Method<'a>>,
}
#[derive(Clone, Copy)]
struct Context {
    caller: usize,
    depth: usize,
}
impl Context {
    fn deeper(self) -> Self {
        Self {
            depth: self.depth + 1,
            ..self
        }
    }
}
#[derive(Clone, Copy)]
enum Target {
    Class(usize),
    Function(usize),
    Namespace(usize),
}

pub fn coupling(files: &[SourceFile], language: LanguageId) -> Vec<MethodCoupling> {
    let project = Project::new(files);
    let edges = project.edges();
    let methods: Vec<_> = project
        .methods
        .iter()
        .map(|method| {
            let owner = &project.owners[method.owner];
            let file = project.files[owner.file];
            Callable {
                owner: method.owner,
                file_path: file.rel.clone(),
                class_name: owner.name.clone(),
                line: file.line(method.node),
                reportable: file.language == language
                    && method.node.child_by_field_name("body").is_some()
                    && method.name != "constructor"
                    && !project.interface_contract(method),
            }
        })
        .collect();
    coupling::count(&methods, &edges)
}

impl<'a> Project<'a> {
    fn new(files: &'a [SourceFile]) -> Self {
        let files: Vec<_> = files
            .iter()
            .filter(|file| is_ecmascript(file.language))
            .collect();
        let mut owners = Vec::new();
        for (index, file) in files.iter().enumerate() {
            owners.push(Owner {
                file: index,
                node: file.root(),
                name: file.rel.clone(),
            });
            for node in descendants(file.root())
                .into_iter()
                .filter(|node| is_owner(*node))
            {
                if let Some(name) = owner_name(file, node) {
                    owners.push(Owner {
                        file: index,
                        node,
                        name: name.into(),
                    });
                }
            }
        }
        let mut methods = Vec::new();
        for (index, file) in files.iter().enumerate() {
            for node in descendants(file.root())
                .into_iter()
                .filter(|node| is_function(*node))
            {
                let Some(name) = function_name(file, node) else {
                    continue;
                };
                let Some(owner_index) = owners
                    .iter()
                    .position(|o| o.file == index && o.node.id() == owner(node).id())
                else {
                    continue;
                };
                methods.push(Method {
                    owner: owner_index,
                    node,
                    name,
                });
            }
        }
        Self {
            files,
            owners,
            methods,
        }
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for (caller, method) in self.methods.iter().enumerate() {
            if method.name == "constructor" {
                continue;
            }
            for call in descendants(method.node)
                .into_iter()
                .filter(|node| node.kind() == "call_expression")
            {
                if enclosing_function(call).is_none_or(|parent| parent.id() != method.node.id()) {
                    continue;
                }
                let Some(function) = call.child_by_field_name("function") else {
                    continue;
                };
                if let Some(Target::Function(target)) =
                    self.expression(Context { caller, depth: 0 }, function)
                {
                    edges.push((caller, target));
                }
            }
        }
        edges
    }

    fn file_index(&self, context: Context) -> usize {
        self.owners[self.methods[context.caller].owner].file
    }

    fn expression(&self, context: Context, node: Node<'a>) -> Option<Target> {
        if context.depth > 24 {
            return None;
        }
        let file = self.file_index(context);
        let source = self.files[file];
        if node.kind() == "identifier" {
            return self.identifier(context, node, source.text(node));
        }
        if node.kind() == "this" {
            let owner = self.methods[context.caller].owner;
            return is_owner(self.owners[owner].node).then_some(Target::Class(owner));
        }
        if node.kind() == "object" {
            return self
                .owners
                .iter()
                .position(|owner| owner.file == file && owner.node.id() == node.id())
                .map(Target::Class);
        }
        if node.kind() == "new_expression" {
            return self
                .named_type(
                    file,
                    source.text(node.child_by_field_name("constructor")?),
                    context.depth + 1,
                )
                .map(Target::Class);
        }
        if node.kind() == "member_expression" {
            return self.member(context, node);
        }
        if [
            "parenthesized_expression",
            "as_expression",
            "non_null_expression",
        ]
        .contains(&node.kind())
        {
            return children(node)
                .first()
                .and_then(|child| self.expression(context.deeper(), *child));
        }
        None
    }

    fn identifier(&self, context: Context, usage: Node<'a>, name: &str) -> Option<Target> {
        let file = self.file_index(context);
        if let Some(binding) = find_binding(self.files[file], usage, name) {
            if let Some(annotation) = binding.child_by_field_name("type") {
                return self
                    .named_type(
                        file,
                        annotation_name(self.files[file], annotation)?,
                        context.depth + 1,
                    )
                    .map(Target::Class);
            }
            return binding.child_by_field_name("value").and_then(|value| {
                self.method_node(file, value)
                    .map(Target::Function)
                    .or_else(|| self.expression(context.deeper(), value))
            });
        }
        self.local_symbol(file, name)
            .or_else(|| self.imported(file, name, context.depth + 1))
    }

    fn local_symbol(&self, file: usize, name: &str) -> Option<Target> {
        let classes: Vec<_> = self
            .owners
            .iter()
            .enumerate()
            .filter(|(_, owner)| owner.file == file && owner.name == name)
            .collect();
        if classes.len() == 1 {
            return Some(Target::Class(classes[0].0));
        }
        let functions: Vec<_> = self
            .methods
            .iter()
            .enumerate()
            .filter(|(_, method)| {
                let owner = &self.owners[method.owner];
                owner.file == file
                    && owner.node.kind() == "program"
                    && method.name == name
                    && enclosing_function(method.node).is_none()
            })
            .collect();
        (functions.len() == 1).then(|| Target::Function(functions[0].0))
    }

    fn method_node(&self, file: usize, node: Node<'a>) -> Option<usize> {
        self.methods.iter().position(|method| {
            self.owners[method.owner].file == file && method.node.id() == node.id()
        })
    }

    fn named_type(&self, file: usize, name: &str, depth: usize) -> Option<usize> {
        if depth > 24 {
            return None;
        }
        let target = self
            .local_symbol(file, name)
            .or_else(|| self.imported(file, name, depth + 1))?;
        if let Target::Class(owner) = target {
            Some(owner)
        } else {
            None
        }
    }

    fn member(&self, context: Context, node: Node<'a>) -> Option<Target> {
        let file = self.files[self.file_index(context)];
        let name = file.text(node.child_by_field_name("property")?);
        let receiver = self.expression(context.deeper(), node.child_by_field_name("object")?)?;
        if let Target::Namespace(module) = receiver {
            return self.exported(module, name, context.depth + 1);
        }
        if let Target::Class(owner) = receiver {
            return self
                .method_named(owner, name)
                .map(Target::Function)
                .or_else(|| {
                    self.field_type(owner, name, context.depth + 1)
                        .map(Target::Class)
                });
        }
        None
    }

    fn hierarchy(&self, owner: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        let mut pending = vec![owner];
        while let Some(next) = pending.pop() {
            if seen.insert(next) {
                result.push(next);
                pending.extend(self.parents(next));
            }
        }
        result
    }

    fn parents(&self, index: usize) -> Vec<usize> {
        let owner = &self.owners[index];
        let file = self.files[owner.file];
        children(owner.node)
            .into_iter()
            .filter(|node| ["class_heritage", "extends_type_clause"].contains(&node.kind()))
            .flat_map(descendants)
            .filter(|node| ["identifier", "type_identifier"].contains(&node.kind()))
            .filter_map(|node| self.named_type(owner.file, file.text(node), 0))
            .collect()
    }

    fn method_named(&self, owner: usize, name: &str) -> Option<usize> {
        for owner in self.hierarchy(owner) {
            let methods: Vec<_> = self
                .methods
                .iter()
                .enumerate()
                .filter(|(_, method)| method.owner == owner && method.name == name)
                .collect();
            if methods.len() == 1 {
                return Some(methods[0].0);
            }
            if methods.len() > 1 {
                return None;
            }
        }
        None
    }

    fn interface_contract(&self, method: &Method<'a>) -> bool {
        self.hierarchy(method.owner).into_iter().any(|owner| {
            self.owners[owner].node.kind() == "interface_declaration"
                && self
                    .methods
                    .iter()
                    .any(|other| other.owner == owner && other.name == method.name)
        })
    }

    fn field_type(&self, index: usize, name: &str, depth: usize) -> Option<usize> {
        for owner_index in self.hierarchy(index) {
            let owner = &self.owners[owner_index];
            let file = self.files[owner.file];
            for field in descendants(owner.node).into_iter().filter(|node| {
                super::owner(*node).id() == owner.node.id() && field_binding(file, *node, name)
            }) {
                if owner_index != index && field.kind() == "required_parameter" {
                    continue;
                }
                if let Some(annotation) = field.child_by_field_name("type") {
                    return self.named_type(
                        owner.file,
                        annotation_name(file, annotation)?,
                        depth + 1,
                    );
                }
                if let Some(value) = field
                    .child_by_field_name("value")
                    .or_else(|| field.child_by_field_name("right"))
                    && value.kind() == "new_expression"
                {
                    return self.named_type(
                        owner.file,
                        file.text(value.child_by_field_name("constructor")?),
                        depth + 1,
                    );
                }
            }
        }
        None
    }

    fn imported(&self, file_index: usize, name: &str, depth: usize) -> Option<Target> {
        if depth > 24 {
            return None;
        }
        let file = self.files[file_index];
        for import in children(file.root())
            .into_iter()
            .filter(|node| node.kind() == "import_statement")
        {
            let Some(source) = import.child_by_field_name("source") else {
                continue;
            };
            let Some(module) = self.module_index(file_index, string_value(file.text(source))?)
            else {
                continue;
            };
            for specifier in descendants(import) {
                if specifier.kind() == "import_specifier" {
                    let original = specifier.child_by_field_name("name")?;
                    let local = specifier.child_by_field_name("alias").unwrap_or(original);
                    if file.text(local) == name {
                        return self.exported(module, file.text(original), depth + 1);
                    }
                }
                if specifier.kind() == "namespace_import"
                    && children(specifier).iter().any(|n| file.text(*n) == name)
                {
                    return Some(Target::Namespace(module));
                }
                if specifier.kind() == "import_clause"
                    && children(specifier)
                        .iter()
                        .any(|n| n.kind() == "identifier" && file.text(*n) == name)
                {
                    return self.exported(module, "default", depth + 1);
                }
            }
        }
        None
    }

    fn exported(&self, file_index: usize, name: &str, depth: usize) -> Option<Target> {
        if depth > 24 {
            return None;
        }
        let file = self.files[file_index];
        for export in children(file.root())
            .into_iter()
            .filter(|node| node.kind() == "export_statement")
        {
            if let Some(declaration) = export.child_by_field_name("declaration") {
                let default = file.text(export).trim_start().starts_with("export default");
                if default == (name == "default") {
                    let declared_name = declaration
                        .child_by_field_name("name")
                        .map(|n| file.text(n));
                    if default || declared_name == Some(name) {
                        return declared_name.and_then(|n| self.local_symbol(file_index, n));
                    }
                    if descendants(declaration).iter().any(|n| {
                        n.kind() == "variable_declarator"
                            && n.child_by_field_name("name")
                                .is_some_and(|n| file.text(n) == name)
                    }) {
                        return self.local_symbol(file_index, name);
                    }
                }
            }
            for specifier in descendants(export)
                .into_iter()
                .filter(|node| node.kind() == "export_specifier")
            {
                let original = specifier.child_by_field_name("name")?;
                let alias = specifier.child_by_field_name("alias").unwrap_or(original);
                if file.text(alias) == name {
                    return self.local_symbol(file_index, file.text(original));
                }
            }
        }
        None
    }

    fn module_index(&self, from: usize, specifier: &str) -> Option<usize> {
        if !specifier.starts_with('.') || specifier.contains('\\') {
            return None;
        }
        let parent = Path::new(&self.files[from].rel).parent()?;
        let mut parts = Vec::new();
        for component in parent.join(specifier).components() {
            if component == Component::ParentDir {
                parts.pop()?;
            } else if let Component::Normal(part) = component {
                parts.push(part.to_str()?.to_string());
            }
        }
        let base = parts.join("/");
        if let Some(index) = self.files.iter().position(|file| file.rel == base) {
            return Some(index);
        }
        let base = base.strip_suffix(".js").unwrap_or(&base);
        let candidates: Vec<_> = self
            .files
            .iter()
            .enumerate()
            .filter(|(_, file)| {
                ["js", "jsx", "mjs", "cjs", "ts", "tsx", "mts", "cts"]
                    .iter()
                    .any(|ext| {
                        file.rel == format!("{base}.{ext}")
                            || file.rel == format!("{base}/index.{ext}")
                    })
            })
            .map(|(index, _)| index)
            .collect();
        (candidates.len() == 1).then(|| candidates[0])
    }
}

fn string_value(raw: &str) -> Option<&str> {
    let raw = raw.get(1..raw.len().checked_sub(1)?)?;
    (!raw.contains('\\')).then_some(raw)
}

fn annotation_name<'a>(file: &'a SourceFile, annotation: Node<'_>) -> Option<&'a str> {
    children(annotation)
        .into_iter()
        .find(|node| ["type_identifier", "identifier"].contains(&node.kind()))
        .map(|node| file.text(node))
}

fn field_binding(file: &SourceFile, node: Node<'_>, name: &str) -> bool {
    if node.kind() == "assignment_expression" {
        return constructor_field_assignment(file, node, name);
    }
    if ["public_field_definition", "field_definition"].contains(&node.kind()) {
        return node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("property"))
            .is_some_and(|n| file.text(n) == name);
    }
    node.kind() == "required_parameter"
        && children(node)
            .iter()
            .any(|n| n.kind() == "accessibility_modifier")
        && binding_name(file, node) == Some(name)
}

fn constructor_field_assignment(file: &SourceFile, node: Node<'_>, name: &str) -> bool {
    let Some(left) = node.child_by_field_name("left") else {
        return false;
    };
    left.kind() == "member_expression"
        && left
            .child_by_field_name("object")
            .is_some_and(|object| object.kind() == "this")
        && left
            .child_by_field_name("property")
            .is_some_and(|property| file.text(property) == name)
        && enclosing_function(node)
            .is_some_and(|method| function_name(file, method).as_deref() == Some("constructor"))
}

fn binding_name<'a>(file: &'a SourceFile, node: Node<'_>) -> Option<&'a str> {
    if node.kind() == "identifier" {
        return Some(file.text(node));
    }
    node.child_by_field_name("name")
        .or_else(|| node.child_by_field_name("pattern"))
        .map(|n| file.text(n))
}

fn find_binding<'a>(file: &SourceFile, usage: Node<'a>, name: &str) -> Option<Node<'a>> {
    let mut scope = usage.parent();
    while let Some(current) = scope {
        if is_scope(current) {
            let binding = descendants(current).into_iter().find(|node| {
                is_binding(*node)
                    && binding_name(file, *node) == Some(name)
                    && binding_scope(*node).is_some_and(|scope| scope.id() == current.id())
            });
            if binding.is_some() {
                return binding;
            }
        }
        scope = current.parent();
    }
    None
}

fn is_binding(node: Node<'_>) -> bool {
    [
        "variable_declarator",
        "required_parameter",
        "optional_parameter",
    ]
    .contains(&node.kind())
        || node.kind() == "identifier"
            && node.parent().is_some_and(|parent| {
                parent.kind() == "formal_parameters"
                    || parent
                        .child_by_field_name("parameter")
                        .is_some_and(|n| n.id() == node.id())
            })
}

fn is_scope(node: Node<'_>) -> bool {
    is_function(node)
        || [
            "statement_block",
            "program",
            "for_statement",
            "for_in_statement",
            "catch_clause",
        ]
        .contains(&node.kind())
}
fn binding_scope(node: Node<'_>) -> Option<Node<'_>> {
    let mut parent = node.parent();
    let function_scoped = parent.is_some_and(|n| n.kind() == "variable_declaration");
    while let Some(node) = parent {
        if is_scope(node) && (!function_scoped || is_function(node) || node.kind() == "program") {
            return Some(node);
        }
        parent = node.parent();
    }
    None
}
