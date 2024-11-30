use super::unit::Block;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Block),
    ControlFlow { condition: Block, execute: Block },
    Loop(LoopStatement),
}

#[derive(Debug, PartialEq)]
pub enum LoopStatement {
    While { condition: Block, execute: Block },
}
