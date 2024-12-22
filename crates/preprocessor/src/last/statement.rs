use std::rc::Rc;

use super::{expression::Expression, unit::LASTUnit};

#[derive(Debug, PartialEq)]
pub enum Statement {
    ControlFlow {
        condition: Rc<Expression>,
        execute: Vec<Rc<LASTUnit>>,
        alternative: Option<Vec<Rc<LASTUnit>>>,
    },
    Loop {
        condition: Rc<Expression>,
        body: Vec<Rc<LASTUnit>>,
    },
    Return(Rc<Expression>),
}
