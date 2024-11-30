use common::types::Type;

use super::unit::Block;

#[derive(Debug, PartialEq)]
pub enum Declaration {
    // TypeDeclaration, // not implemented yet
    VariableDeclaration {
        keyword: VariableDeclarationKeyword,
        identifier: String,
        expression: Block,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        expression: Block,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclarationKeyword {
    Const,
    Let,
}
