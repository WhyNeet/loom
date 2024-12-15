use lexer::lexer;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Char(char),
}

impl Literal {
    pub fn from_literal_token(value: &lexer::token::Literal) -> Self {
        match value {
            lexer::token::Literal::String(string) => Self::String(string.clone()),
            lexer::token::Literal::Number(num_str) => Self::Int32(num_str.parse().unwrap()),
            lexer::token::Literal::Boolean(bool) => Self::Bool(bool.parse().unwrap()),
        }
    }
}
