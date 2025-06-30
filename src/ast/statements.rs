#![allow(warnings)]
use crate::ast::{
    ast,
    expressions::{self, BinaryExpression},
};

pub struct BlockStatement {
    start: usize,
    end: usize,

    body: Vec<ast::Item>,
}

pub struct IfStatement {
    start: usize,
    end: usize,

    consequent: BlockStatement,
    alternate: Box<IfAlternate>,
    test: expressions::TestExpression,
}

pub enum IfAlternate {
    IfStatement(IfStatement),
    BlockStatement(BlockStatement),
}

pub struct ReturnStatement {
    start: usize,
    end: usize,
    arguments: ast::LiteralOrIdentifier,
}

pub enum Statement {
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    ReturnStatement(ReturnStatement),

    ExpressionStatement(expressions::ExpressionStatement),
}
