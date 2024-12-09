use std::rc::Rc;

use super::{declaration::Declaration, expression::Expression, statement::Statement};

pub type Block = Vec<Rc<ASTUnit>>;

#[derive(Debug, PartialEq)]
pub enum ASTUnit {
    Declaration(Declaration),
    Statement(Statement),
    Expression(Expression),
    Block(Block),
}
