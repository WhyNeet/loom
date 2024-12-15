use std::rc::Rc;

use super::{literal::Literal, operation::Operation, unit::ASTUnit};

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpression {
        left: Rc<ASTUnit>,
        right: Rc<ASTUnit>,
        operation: Operation,
    },
    Literal(Literal),
    Identifier(String),
    FunctionInvokation {
        function_name: String,
        parameters: Vec<Rc<ASTUnit>>,
    },
}
