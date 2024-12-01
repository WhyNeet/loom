use super::{literal::Literal, operation::Operation, unit::ASTUnit};

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression {
        left: Box<ASTUnit>,
        right: Box<ASTUnit>,
        operation: Operation,
    },
    Literal(Literal),
    Identifier(String),
    FunctionInvokation {
        function_name: String,
        parameters: Vec<ASTUnit>,
    },
}
