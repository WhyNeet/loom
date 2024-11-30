use super::{literal::Literal, operation::Operation, unit::Block};

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression {
        left: Block,
        right: Block,
        operation: Operation,
    },
    Literal(Literal),
    Identifier(String),
}
