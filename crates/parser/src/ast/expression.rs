use super::{
    literal::Literal,
    operation::Operation,
    unit::{ASTUnit, Block},
};

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression {
        left: Block,
        right: Block,
        operation: Operation,
    },
    Literal(Literal),
    Identifier(String),
    FunctionInvokation {
        function_name: String,
        parameters: Vec<ASTUnit>,
    },
}
