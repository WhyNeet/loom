pub mod declaration;
pub mod expression;
pub mod operation;
pub mod statement;
pub mod unit;

use unit::LASTUnit;

pub struct LoweredAbstractSyntaxTree {
    root: LASTUnit,
}

impl LoweredAbstractSyntaxTree {
    pub fn new(root: LASTUnit) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &LASTUnit {
        &self.root
    }
}
