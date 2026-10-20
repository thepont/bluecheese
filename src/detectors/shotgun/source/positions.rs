//! Numeric enum values and index-preserving runtime catalogue arrays.

use std::collections::BTreeMap;

use crate::lang::{LanguageId, SourceNode};

const SAFE_INTEGER: std::ops::RangeInclusive<i64> = -9_007_199_254_740_991..=9_007_199_254_740_991;

pub(super) fn enum_members(source: SourceNode<'_>) -> Option<BTreeMap<String, i64>> {
    if !matches!(
        source.file().language,
        LanguageId::TypeScript | LanguageId::Tsx
    ) {
        return None;
    }
    let mut members = BTreeMap::new();
    let mut next = 0i64;
    for member in super::children(source) {
        let (name, value) = if member.kind() == "property_identifier" {
            (member, next)
        } else if member.kind() == "enum_assignment" {
            let name = member.child_by_field_name("name")?;
            if name.kind() != "property_identifier" {
                return None;
            }
            let value = member.child_by_field_name("value")?;
            (name, integer(source.file().at(value))?)
        } else {
            return None;
        };
        if !SAFE_INTEGER.contains(&value) {
            return None;
        }
        if members
            .insert(source.file().text(name).into(), value)
            .is_some()
        {
            return None;
        }
        next = value.checked_add(1)?;
    }
    Some(members)
}

pub(super) fn array_members(source: SourceNode<'_>) -> Option<BTreeMap<String, i64>> {
    if source.node().kind() != "array" {
        return None;
    }
    let mut members = BTreeMap::new();
    let mut index = 0i64;
    let mut expects_member = true;
    let mut cursor = source.node().walk();
    for item in source.node().children(&mut cursor) {
        if source.model().is_comment(item) || matches!(item.kind(), "[" | "]") {
            continue;
        }
        if item.kind() == "," {
            if expects_member {
                return None;
            }
            expects_member = true;
        } else {
            if item.kind() != "null" {
                let name = super::literal(source.file().at(item))?;
                if members.insert(name, index).is_some() {
                    return None;
                }
            }
            index = index.checked_add(1)?;
            expects_member = false;
        }
    }
    Some(members)
}

fn integer(source: SourceNode<'_>) -> Option<i64> {
    let (node, sign) = if source.node().kind() == "unary_expression" {
        let operator = source
            .file()
            .text(source.node().child_by_field_name("operator")?);
        if !matches!(operator, "-" | "+") {
            return None;
        }
        (
            source.node().child_by_field_name("argument")?,
            if operator == "-" { -1 } else { 1 },
        )
    } else {
        (source.node(), 1)
    };
    if node.kind() != "number" {
        return None;
    }
    let raw = source.file().text(node).replace('_', "");
    let lower = raw.to_ascii_lowercase();
    let (radix, digits) = [("0x", 16), ("0b", 2), ("0o", 8)]
        .iter()
        .find_map(|(prefix, radix)| lower.strip_prefix(prefix).map(|digits| (*radix, digits)))
        .unwrap_or((10, lower.as_str()));
    i64::from_str_radix(digits, radix).ok()?.checked_mul(sign)
}
