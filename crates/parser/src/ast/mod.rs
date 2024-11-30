use unit::Block;

pub mod declaration;
pub mod expression;
pub mod literal;
pub mod operation;
pub mod statement;
pub mod unit;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: Block,
}

impl AbstractSyntaxTree {
    pub fn new(root: Block) -> Self {
        Self { root }
    }

    pub fn get_root(&self) -> &Block {
        &self.root
    }
}
