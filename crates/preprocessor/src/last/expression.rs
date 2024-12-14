use std::rc::Rc;

use parser::ast::literal::Literal;

use super::operation::Operation;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    FunctionInvokation {
        name: String,
        args: Vec<Expression>,
    },
    BinaryExpression {
        left: Rc<Expression>,
        right: Rc<Expression>,
        operation: Operation,
    },
}
