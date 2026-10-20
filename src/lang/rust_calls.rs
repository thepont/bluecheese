//! Conservative source-only call resolution for Rust coupling metrics.
//!
//! Unqualified calls resolve to free functions in the same source file. Member
//! calls require `self` or an explicit parameter/local type; inferred bindings,
//! closures and unsupported receiver expressions remain unresolved. This is
//! not compiler-backed name, import or trait resolution.

use crate::lang::{
    LanguageId, SourceFile,
    coupling::{self, Callable, MethodCoupling},
    model_for,
};
use crate::walk::descendants;
use tree_sitter::Node;

struct Method<'a> {
    file: &'a SourceFile,
    owner: usize,
    owner_name: String,
    associated: bool,
    node: Node<'a>,
    name: String,
}

pub fn coupling(files: &[SourceFile]) -> Vec<MethodCoupling> {
    let model = model_for(LanguageId::Rust);
    let mut methods = Vec::new();
    let mut next_owner = 0;
    for file in files
        .iter()
        .filter(|file| file.language == LanguageId::Rust)
    {
        for ty in model.types(file) {
            let owner = next_owner;
            next_owner += 1;
            let Some(owner_name) = model.type_name(file.at(ty)).map(str::to_owned) else {
                continue;
            };
            for node in model.methods(file.at(ty)) {
                let Some(name) = node.child_by_field_name("name").map(|node| file.text(node))
                else {
                    continue;
                };
                methods.push(Method {
                    file,
                    owner,
                    owner_name: owner_name.clone(),
                    associated: ty.kind() != "source_file",
                    node,
                    name: name.to_owned(),
                });
            }
        }
    }

    let mut edges = Vec::new();
    for (caller, method) in methods.iter().enumerate() {
        for call in descendants(method.node)
            .into_iter()
            .filter(|node| node.kind() == "call_expression")
        {
            let Some(function) = call.child_by_field_name("function") else {
                continue;
            };
            let Some((name, qualifier)) = method.call_name(function) else {
                continue;
            };
            let candidates: Vec<_> = methods
                .iter()
                .enumerate()
                .filter(|(_, target)| {
                    target.name == name
                        && qualifier.map_or_else(
                            || !target.associated && target.file.rel == method.file.rel,
                            |qualifier| target.associated && target.owner_name == qualifier,
                        )
                })
                .map(|(index, _)| index)
                .collect();
            if candidates.len() == 1 {
                edges.push((caller, candidates[0]));
            }
        }
    }

    let callables: Vec<_> = methods
        .iter()
        .map(|method| Callable {
            owner: method.owner,
            file_path: method.file.rel.clone(),
            class_name: method.owner_name.clone(),
            line: model.decl_line(method.node),
            reportable: method.node.child_by_field_name("body").is_some(),
        })
        .collect();
    coupling::count(&callables, &edges)
}

impl Method<'_> {
    fn call_name(&self, function: Node<'_>) -> Option<(&str, Option<&str>)> {
        let file = self.file;
        match function.kind() {
            "identifier" => Some((file.text(function), None)),
            "scoped_identifier" => {
                let name = function.child_by_field_name("name")?;
                let path = function.child_by_field_name("path")?;
                let qualifier = file.text(path).rsplit("::").next()?;
                Some((
                    file.text(name),
                    Some(if qualifier == "Self" {
                        &self.owner_name
                    } else {
                        qualifier
                    }),
                ))
            }
            "field_expression" => {
                let field = function.child_by_field_name("field")?;
                let receiver = function.child_by_field_name("value")?;
                Some((file.text(field), Some(self.receiver_type(receiver)?)))
            }
            "generic_function" => function
                .named_child(0)
                .and_then(|child| self.call_name(child)),
            _ => None,
        }
    }

    fn receiver_type(&self, receiver: Node<'_>) -> Option<&str> {
        if receiver.kind() == "self" && self.associated {
            return Some(&self.owner_name);
        }
        if receiver.kind() != "identifier" {
            return None;
        }
        let name = self.file.text(receiver);
        let mut scope = receiver.parent();
        while let Some(node) = scope {
            if node.kind() == "block" {
                let mut cursor = node.walk();
                let binding = node
                    .named_children(&mut cursor)
                    .filter(|child| {
                        child.kind() == "let_declaration"
                            && child.start_byte() < receiver.start_byte()
                    })
                    .filter(|child| {
                        child
                            .child_by_field_name("pattern")
                            .is_some_and(|pattern| self.file.text(pattern) == name)
                    })
                    .last();
                if let Some(binding) = binding {
                    return self.declared_type(binding);
                }
            }
            if node.id() == self.node.id() {
                break;
            }
            if node.kind() == "closure_expression" {
                return None;
            }
            scope = node.parent();
        }
        let parameters = self.node.child_by_field_name("parameters")?;
        let mut cursor = parameters.walk();
        parameters
            .named_children(&mut cursor)
            .find(|parameter| {
                parameter
                    .child_by_field_name("pattern")
                    .is_some_and(|pattern| self.file.text(pattern) == name)
            })
            .and_then(|parameter| self.declared_type(parameter))
    }

    fn declared_type(&self, declaration: Node<'_>) -> Option<&str> {
        let mut ty = declaration.child_by_field_name("type")?;
        while ty.kind() == "reference_type" {
            ty = ty.child_by_field_name("type")?;
        }
        if ty.kind() == "scoped_type_identifier" {
            ty = ty.child_by_field_name("name")?;
        }
        (ty.kind() == "type_identifier").then(|| self.file.text(ty))
    }
}
