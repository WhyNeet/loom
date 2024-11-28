use common::constants::keywords::{
    DECLARATION_CONSTANT, DECLARATION_FUNCTION, DECLARATION_VARIABLE, STATEMENT_ELSE, STATEMENT_IF,
    STATEMENT_RETURN, STATEMENT_WHILE,
};

use crate::ast::VariableDeclarationKeyword;

pub enum Keyword {
    FunctionDeclaration,
    VariableDeclaration(VariableDeclarationKeyword),
    ControlFlowIf,
    ControlFlowElse,
    Return,
    While,
}

pub fn parse_keyword(input: &str) -> Option<Keyword> {
    match input {
        DECLARATION_CONSTANT => Some(Keyword::VariableDeclaration(
            VariableDeclarationKeyword::Const,
        )),
        DECLARATION_FUNCTION => Some(Keyword::FunctionDeclaration),
        DECLARATION_VARIABLE => Some(Keyword::VariableDeclaration(
            VariableDeclarationKeyword::Const,
        )),
        STATEMENT_IF => Some(Keyword::ControlFlowIf),
        STATEMENT_ELSE => Some(Keyword::ControlFlowElse),
        STATEMENT_WHILE => Some(Keyword::While),
        STATEMENT_RETURN => Some(Keyword::Return),
        _ => None,
    }
}
