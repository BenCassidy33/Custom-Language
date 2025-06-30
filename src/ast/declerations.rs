#![allow(warnings)]
use crate::ast::{ast, statements};

pub enum Decleration {
    FunDecleration(FunDecleration),
    VariableDecleration,
    ConstDecleration,
    TypeDecleration,
    ImportDecleration,
}

pub struct FunDecleration {
    ident: ast::Identifier,
    params: Vec<ast::Identifier>,
    asyncronous: bool,
    body: statements::BlockStatement,
}
