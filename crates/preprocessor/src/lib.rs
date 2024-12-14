pub mod last;

use parser::ast::unit::ASTUnit;

pub struct Preprocessor {}

impl Preprocessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, ast: ASTUnit) {
        todo!()
    }
}
