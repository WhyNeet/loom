use std::rc::Rc;

use super::unit::ASTUnit;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Rc<ASTUnit>),
    ImplicitReturn(Rc<ASTUnit>),
    ControlFlow {
        condition: Rc<ASTUnit>,
        execute: Rc<ASTUnit>,
        alternative: Option<Rc<ASTUnit>>,
    },
    Loop(LoopStatement),
}

#[derive(Debug, PartialEq)]
pub enum LoopStatement {
    While {
        condition: Rc<ASTUnit>,
        execute: Rc<ASTUnit>,
    },
}
