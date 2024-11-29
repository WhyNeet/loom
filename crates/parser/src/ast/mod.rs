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
    ControlFlow { condition: Block, execute: Block },
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

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Algebraic(AlgebraicOperation),
    Logical(LogicalOperation),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlgebraicOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl PartialOrd for AlgebraicOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &AlgebraicOperation::Addition
            || self == &AlgebraicOperation::Subtraction && other == &AlgebraicOperation::Division
            || other == &AlgebraicOperation::Multiplication
        {
            Some(std::cmp::Ordering::Less)
        } else if other == &AlgebraicOperation::Addition
            || other == &AlgebraicOperation::Subtraction && self == &AlgebraicOperation::Division
            || self == &AlgebraicOperation::Multiplication
        {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum LogicalOperation {
    Equal = 6,
    GreaterOrEqual = 5,
    LessOrEqual = 4,
    Greater = 3,
    Less = 2,
    Or = 1,
    And = 0,
}

impl PartialOrd for LogicalOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let op1 = *self as u8;
        let op2 = *other as u8;

        op1.partial_cmp(&op2)
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Operation::Algebraic(_), Operation::Logical(_)) => Some(std::cmp::Ordering::Greater),
            (Operation::Logical(_), Operation::Algebraic(_)) => Some(std::cmp::Ordering::Less),
            (Operation::Algebraic(op1), Operation::Algebraic(op2)) => op1.partial_cmp(op2),
            (Operation::Logical(log1), Operation::Logical(log2)) => log1.partial_cmp(log2),
        }
    }
}

#[derive(Debug)]
pub enum VariableDeclarationKeyword {
    Const,
    Let,
}
