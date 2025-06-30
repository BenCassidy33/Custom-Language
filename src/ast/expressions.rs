#![allow(warnings)]
use crate::ast::ast;

pub struct ExpressionStatement {
    start: usize,
    end: usize,

    kind: ExpressionType,
}

pub enum ExpressionType {
    CallExpression(CallExpression),
    MemberExpression(ast::Member),
    BinaryExpression(BinaryExpression),
}

pub struct CallExpression {
    start: usize,
    end: usize,
    calle: ast::Member,
    arguments: Vec<ast::Term>,
}

// Expressions for if statements
pub struct BinaryExpression {
    left: ast::Term,
    right: ast::Term,
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
