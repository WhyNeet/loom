use common::types::Type;

#[derive(Debug, PartialEq, Clone)]
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

impl Token {
    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            Self::Keyword(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Self::Identifier(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_type(&self) -> Option<&Type> {
        match self {
            Self::Type(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Self::Literal(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_operator(&self) -> Option<&str> {
        match self {
            Self::Operator(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_punctuation(&self) -> Option<char> {
        match self {
            Self::Punctuation(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_comment(&self) -> Option<&str> {
        match self {
            Self::Comment(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(String),
}
