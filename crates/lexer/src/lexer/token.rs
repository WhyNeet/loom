#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Type(String),
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
