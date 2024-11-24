use super::types::Type;

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Type(Type),
    Literal(Literal),
    Operator(String),
    Punctuation(char),
    Whitespace,
    Comment(String),
    NewLine,
    EOF,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(String),
}
