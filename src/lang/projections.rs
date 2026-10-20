//! Resolved member reads grouped by representation, or imperative Java consumer.

use std::collections::{BTreeMap, BTreeSet};

use super::{SourceFile, SourceNode};

pub struct Projection<'a> {
    pub consumer: SourceNode<'a>,
    pub target: SourceNode<'a>,
    pub target_name: String,
    pub members: BTreeSet<String>,
    pub kind: ProjectionKind,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionKind {
    Members,
    Cases(String),
}

impl Projection<'_> {
    pub fn shares(&self, common: &BTreeSet<String>, min_overlap_percent: u8) -> bool {
        common.is_subset(&self.members)
            && common.len() as u128 * 100
                >= self.members.len() as u128 * u128::from(min_overlap_percent)
    }
}

#[derive(Clone)]
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
    kind: ProjectionKind,
}

impl MemberRead<'_> {
    fn group_key(&self) -> Option<GroupKey> {
        if is_written(self.usage) {
            return None;
        }
        let representation = representation(self.usage, self.consumer)?;
        Some(GroupKey {
            target: (
                self.target.file().rel.clone(),
                self.target.node().start_byte(),
            ),
            consumer: (
                self.consumer.file().rel.clone(),
                self.consumer.node().start_byte(),
            ),
            representation,
            receiver: self.receiver.clone(),
            kind: ProjectionKind::Members,
        })
    }
}

#[derive(Default)]
pub struct Reads<'a> {
    groups: BTreeMap<GroupKey, Projection<'a>>,
}

impl<'a> Reads<'a> {
    pub fn record(&mut self, read: MemberRead<'a>) {
        let Some(key) = read.group_key() else {
            return;
        };
        self.record_group(key, read);
    }

    pub fn record_case(&mut self, read: MemberRead<'a>, value: String) {
        let Some(mut key) = read.group_key() else {
            return;
        };
        key.kind = ProjectionKind::Cases(read.member.clone());
        key.representation = read.consumer.node().start_byte();
        self.record_group(
            key,
            MemberRead {
                member: value,
                ..read
            },
        );
    }

    fn record_group(&mut self, key: GroupKey, read: MemberRead<'a>) {
        let kind = key.kind.clone();
        self.groups
            .entry(key)
            .or_insert_with(|| Projection {
                consumer: read.consumer,
                target: read.target,
                target_name: read.target_name,
                members: BTreeSet::new(),
                kind,
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
        if node.id() == consumer.node().id() {
            return (usage.file().language == super::LanguageId::Java).then_some(node.start_byte());
        }
        if super::ecmascript::is_function(node) || node.kind() == "lambda_expression" {
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
