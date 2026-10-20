//! Conservative, language-independent shapes for straight-line calculations.
//! Unknown syntax rejects a whole body. Names are bound by position; literals,
//! operators and reference identity are retained. This is not semantic proof.

use super::{LanguageId, SourceFile, model_for};
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Expression {
    Binding(usize),
    Literal(String),
    Binary(String, Box<Expression>, Box<Expression>),
    Unary(String, Box<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Statement {
    Bind(Expression),
    Assign(usize, Expression),
    Return(Expression),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fingerprint {
    parameters: usize,
    statements: Vec<Statement>,
}

pub struct Function {
    pub fingerprint: Fingerprint,
    pub file_path: String,
    pub class_name: String,
    pub line: usize,
    pub language: LanguageId,
}

impl Function {
    pub fn cross_language_partners(&self, group: &[Self]) -> String {
        group
            .iter()
            .filter(|other| other.language != self.language)
            .map(|other| format!("{}:{} ({:?})", other.file_path, other.line, other.language))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub fn functions(files: &[SourceFile]) -> Vec<Function> {
    let mut functions = Vec::new();
    for file in files {
        let model = model_for(file.language);
        for ty in model.types(file) {
            let Some(class_name) = model.type_name(file.at(ty)) else {
                continue;
            };
            for method in model.methods(file, ty) {
                if let Some(fingerprint) = Normalizer::function(file, method) {
                    functions.push(Function {
                        fingerprint,
                        file_path: file.rel.clone(),
                        class_name: class_name.into(),
                        line: model.decl_line(method),
                        language: language_family(file.language),
                    });
                }
            }
        }
    }
    functions
}

fn language_family(language: LanguageId) -> LanguageId {
    if language == LanguageId::Tsx {
        LanguageId::TypeScript
    } else {
        language
    }
}

struct Normalizer<'a> {
    file: &'a SourceFile,
    bindings: BTreeMap<String, usize>,
}

impl<'a> Normalizer<'a> {
    fn function(file: &'a SourceFile, method: Node<'_>) -> Option<Fingerprint> {
        if method.has_error() {
            return None;
        }
        let parameters = parameter_names(file, method)?;
        let count = parameters.len();
        let bindings = parameters
            .into_iter()
            .enumerate()
            .map(|(index, name)| (name, index))
            .collect();
        let mut normalizer = Self { file, bindings };
        let body = method.child_by_field_name("body")?;
        if !["block", "statement_block"].contains(&body.kind()) {
            return None;
        }
        let nodes = children(body);
        let mut statements = Vec::new();
        for (index, statement) in nodes.iter().enumerate() {
            if file.language == LanguageId::Rust
                && index + 1 == nodes.len()
                && !["let_declaration", "expression_statement"].contains(&statement.kind())
            {
                statements.push(Statement::Return(normalizer.expression(*statement)?));
            } else {
                statements.extend(normalizer.statement(*statement)?);
            }
        }
        if statements.len() < 3
            || !statements
                .last()
                .is_some_and(|node| matches!(node, Statement::Return(_)))
        {
            return None;
        }
        Some(Fingerprint {
            parameters: count,
            statements,
        })
    }

    fn statement(&mut self, node: Node<'_>) -> Option<Vec<Statement>> {
        if [
            "local_variable_declaration",
            "lexical_declaration",
            "variable_declaration",
        ]
        .contains(&node.kind())
        {
            return children(node)
                .into_iter()
                .filter(|node| node.kind() == "variable_declarator")
                .map(|node| self.declaration(node))
                .collect();
        }
        if node.kind() == "let_declaration" {
            return Some(vec![self.declaration(node)?]);
        }
        if ["return_statement", "return_expression"].contains(&node.kind()) {
            return Some(vec![Statement::Return(
                self.expression(*children(node).first()?)?,
            )]);
        }
        if node.kind() == "expression_statement" {
            let expression = *children(node).first()?;
            if expression.kind() == "return_expression" {
                return self.statement(expression);
            }
            return self.assignment(expression).map(|node| vec![node]);
        }
        None
    }

    fn declaration(&mut self, node: Node<'_>) -> Option<Statement> {
        let name = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("pattern"))?;
        if name.kind() != "identifier" || self.bindings.contains_key(self.file.text(name)) {
            return None;
        }
        let value = self.expression(node.child_by_field_name("value")?)?;
        let index = self.bindings.len();
        self.bindings.insert(self.file.text(name).into(), index);
        Some(Statement::Bind(value))
    }

    fn assignment(&self, node: Node<'_>) -> Option<Statement> {
        if node.kind() != "assignment_expression" || operator(self.file, node)? != "=" {
            return None;
        }
        let left = node.child_by_field_name("left")?;
        if left.kind() != "identifier" {
            return None;
        }
        let index = *self.bindings.get(self.file.text(left))?;
        Some(Statement::Assign(
            index,
            self.expression(node.child_by_field_name("right")?)?,
        ))
    }

    fn expression(&self, node: Node<'_>) -> Option<Expression> {
        ExpressionReader {
            file: self.file,
            bindings: &self.bindings,
            depth: 0,
        }
        .read(node)
    }
}

#[derive(Clone, Copy)]
struct ExpressionReader<'a> {
    file: &'a SourceFile,
    bindings: &'a BTreeMap<String, usize>,
    depth: usize,
}

impl ExpressionReader<'_> {
    fn deeper(self) -> Self {
        Self {
            depth: self.depth + 1,
            ..self
        }
    }

    fn read(&self, node: Node<'_>) -> Option<Expression> {
        if self.depth > 48 {
            return None;
        }
        if node.kind() == "identifier" {
            return self
                .bindings
                .get(self.file.text(node))
                .map(|index| Expression::Binding(*index));
        }
        if node.kind() == "parenthesized_expression" {
            return self.deeper().read(*children(node).first()?);
        }
        if node.kind() == "binary_expression" {
            return self.binary(node);
        }
        if node.kind() == "unary_expression" {
            return Some(Expression::Unary(
                operator(self.file, node)?.into(),
                Box::new(self.deeper().read(*children(node).last()?)?),
            ));
        }
        if [
            "number",
            "integer_literal",
            "decimal_integer_literal",
            "decimal_floating_point_literal",
            "float_literal",
            "boolean_literal",
            "true",
            "false",
        ]
        .contains(&node.kind())
        {
            return Some(Expression::Literal(self.file.text(node).into()));
        }
        None
    }

    fn binary(&self, node: Node<'_>) -> Option<Expression> {
        let left = self.deeper().read(node.child_by_field_name("left")?)?;
        let right = self.deeper().read(node.child_by_field_name("right")?)?;
        Some(Expression::Binary(
            operator(self.file, node)?.into(),
            Box::new(left),
            Box::new(right),
        ))
    }
}

fn parameter_names(file: &SourceFile, method: Node<'_>) -> Option<Vec<String>> {
    let parameters = if let Some(parameters) = method.child_by_field_name("parameters") {
        children(parameters)
    } else {
        vec![method.child_by_field_name("parameter")?]
    };
    parameters
        .into_iter()
        .map(|parameter| {
            if parameter.child_by_field_name("value").is_some() {
                return None;
            }
            let name = if parameter.kind() == "identifier" {
                parameter
            } else {
                parameter
                    .child_by_field_name("name")
                    .or_else(|| parameter.child_by_field_name("pattern"))?
            };
            (name.kind() == "identifier").then(|| file.text(name).to_string())
        })
        .collect()
}

fn operator<'a>(file: &'a SourceFile, node: Node<'_>) -> Option<&'a str> {
    if let Some(operator) = node.child_by_field_name("operator") {
        return Some(file.text(operator));
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|node| {
            !node.is_named()
                && [
                    "+", "-", "*", "/", "%", "=", "!", "~", "&&", "||", "==", "!=", "===", "!==",
                    "<", ">", "<=", ">=",
                ]
                .contains(&file.text(*node))
        })
        .map(|node| file.text(node))
}

fn children(node: Node<'_>) -> Vec<Node<'_>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|node| !["comment", "line_comment", "block_comment"].contains(&node.kind()))
        .collect()
}
