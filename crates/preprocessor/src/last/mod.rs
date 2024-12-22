pub mod declaration;
pub mod expression;
pub mod operation;
pub mod statement;
pub mod unit;

use std::rc::Rc;

use unit::LASTUnit;

#[derive(Debug)]
pub struct LoweredAbstractSyntaxTree {
    root: Vec<Rc<LASTUnit>>,
}

impl LoweredAbstractSyntaxTree {
    pub fn new(root: Vec<Rc<LASTUnit>>) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Vec<Rc<LASTUnit>> {
        &self.root
    }
}
