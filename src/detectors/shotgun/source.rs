//! Parallel, independently written membership declarations. Names and complete
//! literal sets are evidence for review, not proof of shared business meaning.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, ensure};
use tree_sitter::Node;

use crate::finding::Finding;
use crate::lang::{LanguageId, SourceFile, SourceNode};
use crate::walk::descendants;

mod positions;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Catalogue {
    domain: Vec<String>,
    identity: String,
    members: BTreeSet<String>,
}

struct Declaration<'a> {
    file: &'a SourceFile,
    name: &'a str,
    line: usize,
    positions: Option<BTreeMap<String, i64>>,
}

impl Declaration<'_> {
    fn shares_encoding(&self, other: &Self) -> bool {
        self.positions
            .as_ref()
            .zip(other.positions.as_ref())
            .is_none_or(|(left, right)| left == right)
    }
}

pub fn source_evidence(files: &[SourceFile]) -> Result<Vec<Finding>> {
    source_evidence_with(files, &super::BundleRules::default())
}

/// Analyse declaration copies, member bundles and repeated discriminator cases.
pub fn source_evidence_with(
    files: &[SourceFile],
    rules: &super::BundleRules,
) -> Result<Vec<Finding>> {
    let mut groups = BTreeMap::<Catalogue, Vec<Declaration<'_>>>::new();
    for file in files {
        ensure!(
            !file.root().has_error(),
            "cannot establish complete Shotgun Surgery source evidence: parse errors in {}",
            file.rel
        );
        if generated(file) {
            continue;
        }
        for node in descendants(file.root()) {
            if let Some((catalogue, declaration)) = catalogue(file.at(node)) {
                groups.entry(catalogue).or_default().push(declaration);
            }
        }
    }
    let mut findings = Vec::new();
    for (catalogue, declarations) in groups {
        if declarations
            .iter()
            .map(|d| &d.file.rel)
            .collect::<BTreeSet<_>>()
            .len()
            < 2
        {
            continue;
        }
        for declaration in &declarations {
            let peers: Vec<_> = declarations
                .iter()
                .filter(|other| declaration.shares_encoding(other))
                .collect();
            if peers
                .iter()
                .map(|other| &other.file.rel)
                .collect::<BTreeSet<_>>()
                .len()
                < 2
            {
                continue;
            }
            let locations = peers
                .iter()
                .map(|other| format!("{}:{}", other.file.rel, other.line))
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding {
                smell_name: super::SMELL_NAME.into(),
                file_path: declaration.file.path.to_string_lossy().into_owned(),
                relative_file_path: declaration.file.rel.clone(),
                class_name: declaration.name.into(),
                line_numbers: vec![declaration.line],
                additional_info: format!(
                    "Source evidence: parallel membership declarations for {} with complete literal set {:?}. Locations: {locations}. If these represent one catalogue, adding a member requires coordinated edits. Review independent domains or frozen versions before treating this as Shotgun Surgery; history was not evaluated.",
                    catalogue.identity, catalogue.members
                ),
            });
        }
    }
    findings.extend(super::bundles::detect(files, rules)?);
    Ok(findings)
}

fn catalogue(source: SourceNode<'_>) -> Option<(Catalogue, Declaration<'_>)> {
    let SourceNode { file, node } = source;
    let is_enum = matches!(node.kind(), "enum_declaration" | "enum_item");
    let is_export = node.kind() == "assignment_expression";
    let (name_node, value) = if is_enum {
        (
            node.child_by_field_name("name")?,
            node.child_by_field_name("body")?,
        )
    } else if is_export {
        (exported_name(source)?, node.child_by_field_name("right")?)
    } else if matches!(
        node.kind(),
        "variable_declarator" | "const_item" | "static_item"
    ) {
        (
            node.child_by_field_name("name")?,
            node.child_by_field_name("value")?,
        )
    } else {
        return None;
    };
    let positions = if is_enum {
        positions::enum_members(file.at(value))
    } else if is_export {
        Some(positions::array_members(file.at(value))?)
    } else {
        None
    };
    let members = if let Some(positions) = &positions {
        positions.keys().cloned().collect()
    } else if is_enum {
        enum_members(file.at(value))?
    } else {
        list_members(file.at(value))?
    };
    if members.len() < 2 {
        return None;
    }
    if !matches!(
        name_node.kind(),
        "identifier" | "type_identifier" | "property_identifier"
    ) {
        return None;
    }
    let name = file.text(name_node);
    let mut identity: String = name
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    if identity.ends_with('s') && !identity.ends_with("ss") {
        identity.pop();
    }
    Some((
        Catalogue {
            domain: domain(source),
            identity,
            members,
        },
        Declaration {
            file,
            name,
            line: file.line(node),
            positions,
        },
    ))
}

fn exported_name(source: SourceNode<'_>) -> Option<Node<'_>> {
    let node = source.node();
    if node.parent()?.kind() != "expression_statement"
        || node.parent()?.parent()?.kind() != "program"
    {
        return None;
    }
    let left = node.child_by_field_name("left")?;
    if left.kind() != "member_expression" {
        return None;
    }
    let receiver = source.file().text(left.child_by_field_name("object")?);
    if !matches!(receiver, "exports" | "module.exports") {
        return None;
    }
    let root = receiver.split('.').next()?;
    if crate::lang::ecmascript::calls::has_module_binding(source.file(), root) {
        return None;
    }
    left.child_by_field_name("property")
}

fn enum_members(source: SourceNode<'_>) -> Option<BTreeSet<String>> {
    let mut members = BTreeSet::new();
    for member in children(source) {
        if member.kind() != "enum_body_declarations" {
            members.insert(enum_member(source.file().at(member))?);
        }
    }
    Some(members)
}

fn enum_member(source: SourceNode<'_>) -> Option<String> {
    let SourceNode { file, node } = source;
    if node.kind() == "property_identifier" {
        return Some(source.text().into());
    }
    if node.kind() == "enum_assignment" {
        return literal(file.at(node.child_by_field_name("value")?));
    }
    if !matches!(node.kind(), "enum_constant" | "enum_variant")
        || ["body", "arguments", "value"]
            .iter()
            .any(|field| node.child_by_field_name(field).is_some())
    {
        return None;
    }
    Some(file.text(node.child_by_field_name("name")?).into())
}

fn list_members(source: SourceNode<'_>) -> Option<BTreeSet<String>> {
    let SourceNode {
        file,
        node: mut value,
    } = source;
    if value.kind() == "as_expression" {
        value = value.named_child(0)?;
    }
    if !matches!(
        value.kind(),
        "array" | "array_expression" | "array_initializer"
    ) {
        return None;
    }
    let mut members = BTreeSet::new();
    let mut expects_member = true;
    let mut cursor = value.walk();
    for item in value.children(&mut cursor) {
        if file.at(item).model().is_comment(item) || matches!(item.kind(), "[" | "]" | "{" | "}") {
            continue;
        }
        if item.kind() == "," {
            if expects_member {
                return None;
            }
            expects_member = true;
        } else {
            members.insert(literal(file.at(item))?);
            expects_member = false;
        }
    }
    Some(members)
}

/// Escapes and raw strings require language-specific decoding before comparison;
/// this initial source check accepts only unescaped quoted strings.
fn literal(source: SourceNode<'_>) -> Option<String> {
    let SourceNode { file, node } = source;
    if !matches!(node.kind(), "string" | "string_literal") {
        return None;
    }
    let raw = file.text(node);
    let quote = raw.chars().next()?;
    if !matches!(quote, '\'' | '"') || !raw.ends_with(quote) || raw.len() < 2 {
        return None;
    }
    let content = &raw[1..raw.len() - 1];
    if content.is_empty() || content.contains('\\') {
        return None;
    }
    Some(content.into())
}

fn children(source: SourceNode<'_>) -> Vec<Node<'_>> {
    let SourceNode { file, node } = source;
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|child| !file.at(*child).model().is_comment(*child))
        .collect()
}

fn domain(source: SourceNode<'_>) -> Vec<String> {
    let SourceNode { file, node } = source;
    let mut domain = Vec::new();
    if file.language == LanguageId::Java {
        let package = crate::lang::java::calls::package_name(file);
        if !package.is_empty() {
            domain.push(package);
        }
    }
    let mut parent = node.parent();
    while let Some(scope) = parent {
        if matches!(scope.kind(), "internal_module" | "mod_item")
            && let Some(name) = scope.child_by_field_name("name")
        {
            domain.push(file.text(name).into());
        }
        parent = scope.parent();
    }
    domain
}

pub(super) fn generated(file: &SourceFile) -> bool {
    let mut cursor = file.root().walk();
    file.root()
        .named_children(&mut cursor)
        .take_while(|node| file.at(*node).model().is_comment(*node))
        .any(|comment| {
            let text = file.text(comment).to_ascii_lowercase();
            text.contains("@generated")
                || (text.contains("generated") && text.contains("do not edit"))
        })
}
