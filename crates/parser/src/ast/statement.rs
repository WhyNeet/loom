use super::unit::{ASTUnit, Block};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Block),
    ControlFlow {
        condition: Block,
        execute: Block,
        alternative: Option<Box<ASTUnit>>,
    },
    Loop(LoopStatement),
}

#[derive(Debug, PartialEq)]
pub enum LoopStatement {
    While { condition: Block, execute: Block },
}
