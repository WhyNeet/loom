use super::unit::{ASTUnit, Block};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Block),
    ControlFlow {
        condition: Box<ASTUnit>,
        execute: Box<ASTUnit>,
        alternative: Option<Box<ASTUnit>>,
    },
    Loop(LoopStatement),
}

#[derive(Debug, PartialEq)]
pub enum LoopStatement {
    While {
        condition: Box<ASTUnit>,
        execute: Box<ASTUnit>,
    },
}
