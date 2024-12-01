use common::types::Type;

use super::unit::ASTUnit;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    // TypeDeclaration, // not implemented yet
    VariableDeclaration {
        keyword: VariableDeclarationKeyword,
        identifier: String,
        expression: Box<ASTUnit>,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        expression: Box<ASTUnit>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclarationKeyword {
    Const,
    Let,
}
