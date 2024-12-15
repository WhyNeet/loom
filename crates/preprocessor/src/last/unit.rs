use super::{declaration::Declaration, expression::Expression, statement::Statement};

#[derive(Debug, PartialEq)]
pub enum LASTUnit {
    Statement(Statement),
    Expression(Expression),
    Declaration(Declaration),
}
