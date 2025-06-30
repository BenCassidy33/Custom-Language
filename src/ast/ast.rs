#![allow(warnings)]
use crate::ast::{declerations, items, statements};

pub struct Program {
    kind: String,
    //loc: todo!()
    range: (usize, usize),
    body: Vec<statements::Statement>,
}

pub enum Item {
    Statement(statements::Statement),
    Decleration(declerations::Decleration),
}

pub enum LiteralOrIdentifier {
    Literal(Literal),
    Identifier(Identifier),
}

pub struct Identifier {
    name: String,
    start: usize,
    end: usize,
}

pub struct Literal {
    start: usize,
    end: usize,
    raw: String,
    value: String,
}
