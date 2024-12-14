use std::rc::Rc;

use common::types::Type;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    VariableDeclaration {
        allocation: VariableAllocation,
        identifier: String,
        expression: Rc<Expression>,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        expression: Rc<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum VariableAllocation {
    SSA,
    Stack,
}
