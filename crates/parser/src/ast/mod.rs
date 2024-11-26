use common::types::Type;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    root: Block,
}

impl AbstractSyntaxTree {
    pub fn new(root: Block) -> Self {
        Self { root }
    }

    pub fn get_root(&self) -> &Block {
        &self.root
    }
}

pub type Block = Vec<ASTUnit>;

#[derive(Debug)]
pub enum ASTUnit {
    Declaration(Declaration),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Declaration {
    // TypeDeclaration, // not implemented yet
    VariableDeclaration {
        keyword: VariableDeclarationKeyword,
        identifier: String,
        expression: Block,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type: Type,
        expression: Block,
    },
}

#[derive(Debug)]
pub enum Statement {
    Return(Block),
}

#[derive(Debug)]
pub enum Expression {
    BinaryExpression {
        left: Block,
        right: Block,
        operation: Operation,
    },
    Literal(Literal),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum VariableDeclarationKeyword {
    Const,
    Let,
}
