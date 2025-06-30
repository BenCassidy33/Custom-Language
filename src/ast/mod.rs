pub mod ast;
pub mod declerations;
pub mod expressions;
pub mod generation;
pub mod items;
pub mod statements;

pub enum Visibility {
    VisInherited,
    VisPrivate,
    VisPublic,
}
