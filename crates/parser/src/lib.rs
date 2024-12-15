use ast::AbstractSyntaxTree;
use lexer::lexer::token::Token;
use parser::parse;

pub mod ast;
pub mod parser;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, input: &[Token]) -> AbstractSyntaxTree {
        AbstractSyntaxTree::new(parse(input).0)
    }
}
