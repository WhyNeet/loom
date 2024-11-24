#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Type(String),
    Number(String),
    Operator(String),
    Punctuation(char),
    Whitespace,
    Comment(String),
    EOF,
}
