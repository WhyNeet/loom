use std::rc::Rc;

use common::types::Type;

use super::unit::ASTUnit;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    // TypeDeclaration, // not implemented yet
    VariableDeclaration {
        keyword: VariableDeclarationKeyword,
        identifier: String,
        expression: Rc<ASTUnit>,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        expression: Rc<ASTUnit>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclarationKeyword {
    Const,
    Let,
}
