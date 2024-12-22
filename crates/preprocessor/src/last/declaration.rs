use std::rc::Rc;

use common::types::Type;
use parser::ast::declaration::VariableDeclarationKeyword;

use super::{expression::Expression, unit::LASTUnit};

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
        body: Vec<Rc<LASTUnit>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum VariableAllocation {
    SSA,
    Stack,
}

impl From<VariableDeclarationKeyword> for VariableAllocation {
    fn from(value: VariableDeclarationKeyword) -> Self {
        match value {
            VariableDeclarationKeyword::Const => Self::SSA,
            VariableDeclarationKeyword::Let => Self::Stack,
        }
    }
}
