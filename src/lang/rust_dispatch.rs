//! Recognise exhaustive strategy forwarding over project-declared unit enums.

use std::collections::{BTreeMap, BTreeSet};

use super::{LanguageId, SourceFile, SourceNode};
use crate::walk::descendants;

pub struct EnumDispatch {
    variants: BTreeMap<String, Option<BTreeSet<String>>>,
}

impl EnumDispatch {
    pub fn new(files: &[SourceFile]) -> Self {
        let mut variants = BTreeMap::new();
        for file in files
            .iter()
            .filter(|file| file.language == LanguageId::Rust)
        {
            for node in descendants(file.root())
                .into_iter()
                .filter(|node| node.kind() == "enum_item")
            {
                let Some(name) = node.child_by_field_name("name") else {
                    continue;
                };
                variants
                    .entry(file.text(name).to_owned())
                    .and_modify(|known| *known = None)
                    .or_insert_with(|| unit_variants(file.at(node)));
            }
        }
        Self { variants }
    }

    pub fn is_forwarding(&self, source: SourceNode<'_>) -> bool {
        source.file().language == LanguageId::Rust && self.forwarding_enum(source).is_some()
    }

    fn forwarding_enum(&self, source: SourceNode<'_>) -> Option<()> {
        let body = source.node().child_by_field_name("body")?;
        let mut cursor = body.walk();
        let mut patterns = Vec::new();
        let mut calls = Vec::new();
        for arm in body.named_children(&mut cursor) {
            if source.model().is_comment(arm) {
                continue;
            }
            if arm.kind() != "match_arm" {
                return None;
            }
            patterns.extend(arm_patterns(source.file().at(arm))?);
            calls.push(forwarded_call(source.file().at(arm))?);
        }
        let (enum_name, _) = patterns.first()?;
        let declared = self.variants.get(enum_name)?.as_ref()?;
        let covered: BTreeSet<_> = patterns
            .iter()
            .map(|(_, variant)| variant.clone())
            .collect();
        if patterns.iter().any(|(name, _)| name != enum_name)
            || covered.len() != patterns.len()
            || &covered != declared
            || calls.len() < 2
        {
            return None;
        }
        let (_, arguments) = calls.first()?;
        let backends: BTreeSet<_> = calls.iter().map(|(name, _)| name).collect();
        (backends.len() >= 2 && calls.iter().all(|(_, args)| args == arguments)).then_some(())
    }
}

fn unit_variants(source: SourceNode<'_>) -> Option<BTreeSet<String>> {
    let body = source.node().child_by_field_name("body")?;
    let mut cursor = body.walk();
    let mut names = BTreeSet::new();
    for variant in body.named_children(&mut cursor) {
        if source.model().is_comment(variant) {
            continue;
        }
        if variant.kind() != "enum_variant" || variant.child_by_field_name("body").is_some() {
            return None;
        }
        names.insert(
            source
                .file()
                .text(variant.child_by_field_name("name")?)
                .to_owned(),
        );
    }
    (names.len() >= 2).then_some(names)
}

fn arm_patterns(source: SourceNode<'_>) -> Option<Vec<(String, String)>> {
    let pattern = source.node().child_by_field_name("pattern")?;
    if pattern.child_by_field_name("condition").is_some() {
        return None;
    }
    let pattern = pattern.named_child(0)?;
    let alternatives = if pattern.kind() == "or_pattern" {
        let mut cursor = pattern.walk();
        pattern.named_children(&mut cursor).collect()
    } else {
        vec![pattern]
    };
    alternatives
        .into_iter()
        .map(|pattern| variant_pattern(source.file().at(pattern)))
        .collect()
}

fn variant_pattern(source: SourceNode<'_>) -> Option<(String, String)> {
    let pattern = source.node();
    if pattern.kind() != "scoped_identifier" {
        return None;
    }
    let path = pattern.child_by_field_name("path")?;
    let name = pattern.child_by_field_name("name")?;
    if path.kind() != "identifier" {
        return None;
    }
    Some((
        source.file().text(path).to_owned(),
        source.file().text(name).to_owned(),
    ))
}

fn forwarded_call(source: SourceNode<'_>) -> Option<(String, Vec<String>)> {
    let call = source.node().child_by_field_name("value")?;
    if call.kind() != "call_expression" {
        return None;
    }
    let function = call.child_by_field_name("function")?;
    if !matches!(function.kind(), "identifier" | "scoped_identifier") {
        return None;
    }
    let arguments = call.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let args: Option<Vec<_>> = arguments
        .named_children(&mut cursor)
        .map(|argument| {
            (argument.kind() == "identifier").then(|| source.file().text(argument).to_owned())
        })
        .collect();
    Some((source.file().text(function).to_owned(), args?))
}
