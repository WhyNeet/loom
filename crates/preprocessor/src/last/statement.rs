use std::rc::Rc;

use super::{expression::Expression, unit::LASTUnit};

#[derive(Debug, PartialEq)]
pub enum Statement {
    ControlFlow {
        condition: Rc<Expression>,
        execute: Vec<LASTUnit>,
        alternative: Option<Vec<LASTUnit>>,
    },
    Loop {
        condition: Rc<Expression>,
        body: Vec<LASTUnit>,
    },
    Return(Expression),
}
