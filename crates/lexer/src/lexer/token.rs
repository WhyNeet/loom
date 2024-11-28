use common::types::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Type(Type),
    Literal(Literal),
    Operator(String),
    Punctuation(char),
    Comment(String),
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    String(String),
    Number(String),
}
