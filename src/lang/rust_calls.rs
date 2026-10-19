//! Conservative source-only call resolution for Rust coupling metrics.

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
            let Some(owner_name) = model.type_name(file, ty).map(str::to_owned) else {
                continue;
            };
            for node in model.methods(file, ty) {
                let Some(name) = node.child_by_field_name("name").map(|node| file.text(node))
                else {
                    continue;
                };
                methods.push(Method {
                    file,
                    owner,
                    owner_name: owner_name.clone(),
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
            let Some((name, qualifier)) = call_name(method.file, function) else {
                continue;
            };
            let candidates: Vec<_> = methods
                .iter()
                .enumerate()
                .filter(|(_, target)| {
                    target.name == name
                        && qualifier.is_none_or(|qualifier| target.owner_name == qualifier)
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

fn call_name<'a>(file: &'a SourceFile, function: Node<'_>) -> Option<(&'a str, Option<&'a str>)> {
    match function.kind() {
        "identifier" => Some((file.text(function), None)),
        "scoped_identifier" => {
            let name = function.child_by_field_name("name")?;
            let path = function.child_by_field_name("path")?;
            Some((file.text(name), file.text(path).rsplit("::").next()))
        }
        "field_expression" => {
            let field = function.child_by_field_name("field")?;
            Some((file.text(field), None))
        }
        "generic_function" => function
            .named_child(0)
            .and_then(|child| call_name(file, child)),
        _ => None,
    }
}
