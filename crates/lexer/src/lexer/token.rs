use common::types::Type;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Type(Type),
    Literal(Literal),
    Operator(String),
    Punctuation(char),
    Whitespace,
    Comment(String),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Number(String),
}
