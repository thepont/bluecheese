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
                    && !project.type_resolver().interface_contract(method),
            }
        })
        .collect();
    coupling::count(&methods, &edges)
}

impl<'a> Project<'a> {
    fn type_resolver(&self) -> TypeResolver<'_, 'a> {
        TypeResolver {
            project: self,
            depth: 0,
        }
    }

    fn expression(&self, context: Context, node: Node<'a>) -> Option<Target> {
        ExpressionResolver {
            project: self,
            context,
        }
        .read(node)
    }

    fn module(&self, file_index: usize, depth: usize) -> ModuleResolver<'_, 'a> {
        ModuleResolver {
            project: self,
            file_index,
            depth,
        }
    }

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
}

#[derive(Clone, Copy)]
struct TypeResolver<'query, 'source> {
    project: &'query Project<'source>,
    depth: usize,
}

impl<'a> TypeResolver<'_, 'a> {
    fn at_depth(self, depth: usize) -> Self {
        Self { depth, ..self }
    }

    fn deeper(self) -> Self {
        self.at_depth(self.depth + 1)
    }

    fn named_type(&self, file: usize, name: &str) -> Option<usize> {
        if self.depth > 24 {
            return None;
        }
        let target = self.project.local_symbol(file, name).or_else(|| {
            self.project
                .module(file, self.deeper().depth)
                .imported(name)
        })?;
        if let Target::Class(owner) = target {
            Some(owner)
        } else {
            None
        }
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
        let owner = &self.project.owners[index];
        let file = self.project.files[owner.file];
        children(owner.node)
            .into_iter()
            .filter(|node| ["class_heritage", "extends_type_clause"].contains(&node.kind()))
            .flat_map(descendants)
            .filter(|node| ["identifier", "type_identifier"].contains(&node.kind()))
            .filter_map(|node| self.at_depth(0).named_type(owner.file, file.text(node)))
            .collect()
    }

    fn method_named(&self, owner: usize, name: &str) -> Option<usize> {
        for owner in self.hierarchy(owner) {
            let methods: Vec<_> = self
                .project
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
            self.project.owners[owner].node.kind() == "interface_declaration"
                && self
                    .project
                    .methods
                    .iter()
                    .any(|other| other.owner == owner && other.name == method.name)
        })
    }

    fn field_type(&self, index: usize, name: &str) -> Option<usize> {
        for owner_index in self.hierarchy(index) {
            let owner = &self.project.owners[owner_index];
            let file = self.project.files[owner.file];
            for field in descendants(owner.node).into_iter().filter(|node| {
                super::owner(*node).id() == owner.node.id() && field_binding(file, *node, name)
            }) {
                if owner_index != index && field.kind() == "required_parameter" {
                    continue;
                }
                if let Some(annotation) = field.child_by_field_name("type") {
                    return self
                        .deeper()
                        .named_type(owner.file, annotation_name(file, annotation)?);
                }
                if let Some(value) = field
                    .child_by_field_name("value")
                    .or_else(|| field.child_by_field_name("right"))
                    && value.kind() == "new_expression"
                {
                    return self.deeper().named_type(
                        owner.file,
                        file.text(value.child_by_field_name("constructor")?),
                    );
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy)]
struct ExpressionResolver<'query, 'source> {
    project: &'query Project<'source>,
    context: Context,
}

impl<'a> ExpressionResolver<'_, 'a> {
    fn deeper(self) -> Self {
        Self {
            context: self.context.deeper(),
            ..self
        }
    }

    fn read(&self, node: Node<'a>) -> Option<Target> {
        if self.context.depth > 24 {
            return None;
        }
        let file = self.project.file_index(self.context);
        let source = self.project.files[file];
        if node.kind() == "identifier" {
            return self.identifier(node, source.text(node));
        }
        if node.kind() == "this" {
            let owner = self.project.methods[self.context.caller].owner;
            return is_owner(self.project.owners[owner].node).then_some(Target::Class(owner));
        }
        if node.kind() == "object" {
            return self
                .project
                .owners
                .iter()
                .position(|owner| owner.file == file && owner.node.id() == node.id())
                .map(Target::Class);
        }
        if node.kind() == "new_expression" {
            return self
                .project
                .type_resolver()
                .at_depth(self.context.deeper().depth)
                .named_type(file, source.text(node.child_by_field_name("constructor")?))
                .map(Target::Class);
        }
        if node.kind() == "member_expression" {
            return self.member(node);
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
                .and_then(|child| self.deeper().read(*child));
        }
        None
    }

    fn identifier(&self, usage: Node<'a>, name: &str) -> Option<Target> {
        let file = self.project.file_index(self.context);
        if let Some(binding) = find_binding(self.project.files[file], usage, name) {
            if let Some(annotation) = binding.child_by_field_name("type") {
                return self
                    .project
                    .type_resolver()
                    .at_depth(self.context.deeper().depth)
                    .named_type(file, annotation_name(self.project.files[file], annotation)?)
                    .map(Target::Class);
            }
            return binding.child_by_field_name("value").and_then(|value| {
                self.project
                    .method_node(file, value)
                    .map(Target::Function)
                    .or_else(|| self.deeper().read(value))
            });
        }
        self.project.local_symbol(file, name).or_else(|| {
            self.project
                .module(file, self.context.deeper().depth)
                .imported(name)
        })
    }

    fn member(&self, node: Node<'a>) -> Option<Target> {
        let file = self.project.files[self.project.file_index(self.context)];
        let name = file.text(node.child_by_field_name("property")?);
        let receiver = self.deeper().read(node.child_by_field_name("object")?)?;
        if let Target::Namespace(module) = receiver {
            return self
                .project
                .module(module, self.context.deeper().depth)
                .exported(name);
        }
        if let Target::Class(owner) = receiver {
            return self
                .project
                .type_resolver()
                .method_named(owner, name)
                .map(Target::Function)
                .or_else(|| {
                    self.project
                        .type_resolver()
                        .at_depth(self.context.deeper().depth)
                        .field_type(owner, name)
                        .map(Target::Class)
                });
        }
        None
    }
}

#[derive(Clone, Copy)]
struct ModuleResolver<'query, 'source> {
    project: &'query Project<'source>,
    file_index: usize,
    depth: usize,
}

impl<'a> ModuleResolver<'_, 'a> {
    fn source(&self) -> Option<&'a SourceFile> {
        (self.depth <= 24).then(|| self.project.files[self.file_index])
    }
    fn in_module(self, file_index: usize) -> Self {
        Self {
            file_index,
            depth: self.depth + 1,
            ..self
        }
    }

    fn imported(&self, name: &str) -> Option<Target> {
        let file = self.source()?;
        for import in children(file.root())
            .into_iter()
            .filter(|node| node.kind() == "import_statement")
        {
            let Some(source) = import.child_by_field_name("source") else {
                continue;
            };
            let Some(module) = self.module_index(string_value(file.text(source))?) else {
                continue;
            };
            for specifier in descendants(import) {
                if specifier.kind() == "import_specifier" {
                    let (original, local) = specifier_names(specifier)?;
                    if file.text(local) == name {
                        return self.in_module(module).exported(file.text(original));
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
                    return self.in_module(module).exported("default");
                }
            }
        }
        None
    }

    fn exported(&self, name: &str) -> Option<Target> {
        let file = self.source()?;
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
                        return declared_name
                            .and_then(|n| self.project.local_symbol(self.file_index, n));
                    }
                    if descendants(declaration).iter().any(|n| {
                        n.kind() == "variable_declarator"
                            && n.child_by_field_name("name")
                                .is_some_and(|n| file.text(n) == name)
                    }) {
                        return self.project.local_symbol(self.file_index, name);
                    }
                }
            }
            for specifier in descendants(export)
                .into_iter()
                .filter(|node| node.kind() == "export_specifier")
            {
                let (original, alias) = specifier_names(specifier)?;
                if file.text(alias) == name {
                    return self
                        .project
                        .local_symbol(self.file_index, file.text(original));
                }
            }
        }
        None
    }

    fn module_index(&self, specifier: &str) -> Option<usize> {
        if !specifier.starts_with('.') || specifier.contains('\\') {
            return None;
        }
        let parent = Path::new(&self.project.files[self.file_index].rel).parent()?;
        let mut parts = Vec::new();
        for component in parent.join(specifier).components() {
            if component == Component::ParentDir {
                parts.pop()?;
            } else if let Component::Normal(part) = component {
                parts.push(part.to_str()?.to_string());
            }
        }
        let base = parts.join("/");
        if let Some(index) = self.project.files.iter().position(|file| file.rel == base) {
            return Some(index);
        }
        for (runtime, source) in [(".mjs", ".mts"), (".cjs", ".cts")] {
            if let Some(stem) = base.strip_suffix(runtime) {
                return self
                    .project
                    .files
                    .iter()
                    .position(|file| file.rel == format!("{stem}{source}"));
            }
        }
        let base = base.strip_suffix(".js").unwrap_or(&base);
        let candidates: Vec<_> = self
            .project
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

fn specifier_names(specifier: Node<'_>) -> Option<(Node<'_>, Node<'_>)> {
    let original = specifier.child_by_field_name("name")?;
    let alias = specifier.child_by_field_name("alias").unwrap_or(original);
    Some((original, alias))
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
