#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float32,
    Float64,
    Bool,
    Char,
    Void,
}

impl Type {
    pub fn from(input: &str) -> Option<Self> {
        match input {
            "String" => Some(Self::String),
            "i8" => Some(Self::Int8),
            "u8" => Some(Self::UInt8),
            "i16" => Some(Self::Int16),
            "u16" => Some(Self::UInt16),
            "i32" => Some(Self::Int32),
            "u32" => Some(Self::UInt32),
            "i64" => Some(Self::Int64),
            "u64" => Some(Self::UInt64),
            "f32" => Some(Self::Float32),
            "f64" => Some(Self::Float64),
            "bool" => Some(Self::Bool),
            "char" => Some(Self::Char),
            "void" => Some(Self::Void),
            _ => None,
        }
    }
}
