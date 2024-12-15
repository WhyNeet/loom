pub mod declaration;
pub mod expression;
pub mod operation;
pub mod statement;
pub mod unit;

use unit::LASTUnit;

#[derive(Debug)]
pub struct LoweredAbstractSyntaxTree {
    root: Vec<LASTUnit>,
}

impl LoweredAbstractSyntaxTree {
    pub fn new(root: Vec<LASTUnit>) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Vec<LASTUnit> {
        &self.root
    }
}
