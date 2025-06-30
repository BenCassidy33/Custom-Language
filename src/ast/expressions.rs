#![allow(warnings)]
use crate::ast::ast;

pub struct ExpressionStatement {
    start: usize,
    end: usize,

    kind: ExpressionType,
}

pub enum ExpressionType {
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
    BinaryExpression(BinaryExpression),
}

pub struct CallExpression {
    start: usize,
    end: usize,
    calle: MemberExpression,
    arguments: Vec<ast::LiteralOrIdentifier>,
}

pub enum IdentOrMember {
    Identifier(ast::Identifier),
    Member(MemberExpression),
}

pub struct MemberExpression {
    start: usize,
    end: usize,

    property: Box<IdentOrMember>,
}

// Expressions for if statements
pub struct BinaryExpression {
    left: ast::LiteralOrIdentifier,
    right: ast::LiteralOrIdentifier,
    operator: String,
}

pub struct LogicalExpression {
    start: usize,
    end: usize,
    left: BinaryExpression,
    right: BinaryExpression,
    operator: String,
}

pub enum TestExpression {
    LogicalExpression(LogicalExpression),
    BinaryExpression(BinaryExpression),
}
