use unit::ASTUnit;

pub mod declaration;
pub mod expression;
pub mod literal;
pub mod operation;
pub mod statement;
pub mod unit;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: ASTUnit,
}

impl AbstractSyntaxTree {
    pub fn new(root: ASTUnit) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &ASTUnit {
        &self.root
    }
}
