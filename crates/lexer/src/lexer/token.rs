use std::mem;

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

// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         if mem::discriminant(self) != mem::discriminant(other) {
//             return false;
//         }

//         match self {
//             Self::EOF => true,
//             Self::Comment(comment) => comment == other.as_comment().unwrap(),
//             Self::Identifier(ident) => ident == other.as_identifier().unwrap(),
//             Self::Keyword(keyword) => keyword == other.as_keyword().unwrap(),
//             Self::Literal(literal) => literal == other.as_literal().unwrap(),
//             Self::Operator(operator) => operator == other.as_operator().unwrap(),
//             Self::Punctuation(punct) => *punct == other.as_punctuation().unwrap(),
//             Self::Type(t) => t == other.as_type().unwrap(),
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(String),
}

// impl PartialEq for Literal {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::String(s1), Self::String(s2)) => s1 == s2,
//             (Self::Number(n1), Self::Number(n2)) => n1 == n2,
//             _ => false,
//         }
//     }
// }
