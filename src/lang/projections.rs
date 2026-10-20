//! Resolved member reads grouped by the expression constructing a representation.

use std::collections::{BTreeMap, BTreeSet};

use super::{SourceFile, SourceNode};

pub struct Projection<'a> {
    pub consumer: SourceNode<'a>,
    pub target: SourceNode<'a>,
    pub target_name: String,
    pub members: BTreeSet<String>,
}

pub struct MemberRead<'a> {
    pub usage: SourceNode<'a>,
    pub consumer: SourceNode<'a>,
    pub target: SourceNode<'a>,
    pub target_name: String,
    pub receiver: String,
    pub member: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct GroupKey {
    target: (String, usize),
    consumer: (String, usize),
    representation: usize,
    receiver: String,
}

#[derive(Default)]
pub struct Reads<'a> {
    groups: BTreeMap<GroupKey, Projection<'a>>,
}

impl<'a> Reads<'a> {
    pub fn record(&mut self, read: MemberRead<'a>) {
        if is_written(read.usage) {
            return;
        }
        let Some(representation) = representation(read.usage, read.consumer) else {
            return;
        };
        let key = GroupKey {
            target: (
                read.target.file().rel.clone(),
                read.target.node().start_byte(),
            ),
            consumer: (
                read.consumer.file().rel.clone(),
                read.consumer.node().start_byte(),
            ),
            representation,
            receiver: read.receiver,
        };
        self.groups
            .entry(key)
            .or_insert_with(|| Projection {
                consumer: read.consumer,
                target: read.target,
                target_name: read.target_name,
                members: BTreeSet::new(),
            })
            .members
            .insert(read.member);
    }

    pub fn into_projections(self) -> Vec<Projection<'a>> {
        self.groups.into_values().collect()
    }
}

pub fn collect(files: &[SourceFile]) -> Vec<Projection<'_>> {
    let mut reads = Reads::default();
    super::ecmascript::calls::projection_reads(files, &mut reads);
    super::java::calls::projection_reads(files, &mut reads);
    reads.into_projections()
}

pub fn is_getter(name: &str) -> bool {
    ["get", "is", "has"].iter().any(|prefix| {
        name.strip_prefix(prefix)
            .and_then(|suffix| suffix.chars().next())
            .is_some_and(|ch| ch == '_' || ch.is_uppercase())
    })
}

fn representation(usage: SourceNode<'_>, consumer: SourceNode<'_>) -> Option<usize> {
    let mut parent = usage.node().parent();
    while let Some(node) = parent {
        if node.id() == consumer.node().id() || super::ecmascript::is_function(node) {
            return None;
        }
        if matches!(
            node.kind(),
            "object"
                | "array"
                | "template_string"
                | "array_initializer"
                | "object_creation_expression"
        ) {
            return Some(node.start_byte());
        }
        parent = node.parent();
    }
    None
}

fn is_written(source: SourceNode<'_>) -> bool {
    let mut node = source.node();
    while let Some(parent) = node.parent() {
        if parent.kind() == "parenthesized_expression" {
            node = parent;
            continue;
        }
        return parent.kind() == "update_expression"
            || matches!(
                parent.kind(),
                "assignment_expression" | "augmented_assignment_expression"
            ) && parent
                .child_by_field_name("left")
                .is_some_and(|left| left.id() == node.id());
    }
    false
}
