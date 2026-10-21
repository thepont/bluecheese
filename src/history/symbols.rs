//! Stable declaration identities and comment-free body token sequences.

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use tree_sitter::{Node, Parser};

use crate::lang::{LanguageId, SourceFile, SourceNode, ecmascript, model_for};
use crate::walk::descendants;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Token(String, String);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolKey {
    scopes: Vec<Vec<Token>>,
    header: Vec<Token>,
}

pub struct Symbols {
    bodies: BTreeMap<SymbolKey, Option<Vec<Token>>>,
    positions: BTreeMap<usize, Option<SymbolKey>>,
}

impl Symbols {
    pub fn from_file(file: &SourceFile) -> Self {
        let mut bodies = BTreeMap::new();
        let mut positions = BTreeMap::new();
        for node in descendants(file.root())
            .into_iter()
            .filter(|node| is_function(file.at(*node).model().id(), *node))
        {
            let Some(body) = node.child_by_field_name("body") else {
                continue;
            };
            let key = symbol_key(file.at(node), body);
            positions
                .entry(node.start_byte())
                .and_modify(|known| *known = None)
                .or_insert_with(|| Some(key.clone()));
            bodies
                .entry(key)
                .and_modify(|known| *known = None)
                .or_insert_with(|| Some(tokens(file.at(body), None)));
        }
        Self { bodies, positions }
    }

    pub fn parse(source: String, language: LanguageId) -> Result<Option<Self>> {
        let mut parser = Parser::new();
        parser.set_language(&model_for(language).grammar())?;
        let tree = parser
            .parse(&source, None)
            .context("parsing historical source")?;
        if tree.root_node().has_error() {
            return Ok(None);
        }
        Ok(Some(Self::from_file(&SourceFile {
            path: PathBuf::new(),
            rel: String::new(),
            source,
            tree,
            language,
        })))
    }

    pub fn at_offset(&self, offset: usize) -> Option<&SymbolKey> {
        self.positions.get(&offset)?.as_ref()
    }

    pub fn body_changed(&self, later: &Self, key: &SymbolKey) -> bool {
        self.bodies
            .get(key)
            .and_then(Option::as_ref)
            .zip(later.bodies.get(key).and_then(Option::as_ref))
            .is_some_and(|(before, after)| before != after)
    }
}

fn is_function(language: LanguageId, node: Node<'_>) -> bool {
    if language == LanguageId::Java {
        node.kind() == "method_declaration"
    } else if language == LanguageId::Rust {
        node.kind() == "function_item"
    } else {
        ecmascript::is_function(node)
    }
}

fn symbol_key(source: SourceNode<'_>, body: Node<'_>) -> SymbolKey {
    let mut scopes = Vec::new();
    let mut parent = source.node().parent();
    while let Some(node) = parent {
        if node.child_by_field_name("body").is_some() {
            scopes.push(tokens(
                source.file().at(node),
                node.child_by_field_name("body"),
            ));
        }
        parent = node.parent();
    }
    let mut header = tokens(source, Some(body));
    if ecmascript::is_ecmascript(source.model().id())
        && let Some(name) = ecmascript::function_name(source)
    {
        header.insert(0, Token("binding".to_owned(), name));
    }
    SymbolKey { scopes, header }
}

fn tokens(source: SourceNode<'_>, excluded: Option<Node<'_>>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut stack = vec![source.node()];
    while let Some(node) = stack.pop() {
        if excluded.is_some_and(|excluded| excluded.id() == node.id())
            || source.model().is_comment(node)
        {
            continue;
        }
        if node.child_count() == 0 {
            result.push(Token(
                node.kind().to_owned(),
                source.file().text(node).to_owned(),
            ));
        } else {
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();
            stack.extend(children.into_iter().rev());
        }
    }
    result
}
