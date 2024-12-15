use parser::ast::operation::{AlgebraicOperation, LogicalOperation};

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Algebraic(AlgebraicOperation),
    Logical(LogicalOperation),
    Assignment,
}

impl Operation {
    pub fn from_str(value: &str) -> Option<Self> {
        AlgebraicOperation::from_str(value)
            .map(|alg| Self::Algebraic(alg))
            .or(LogicalOperation::from_str(value).map(|log| Self::Logical(log)))
            .or(Some(Self::Assignment))
    }
}

impl From<&parser::ast::operation::Operation> for Operation {
    fn from(value: &parser::ast::operation::Operation) -> Self {
        match value {
            parser::ast::operation::Operation::Algebraic(alg) => Self::Algebraic(alg.clone()),
            parser::ast::operation::Operation::Logical(log) => Self::Logical(log.clone()),
            parser::ast::operation::Operation::Assignment(_) => Self::Assignment,
        }
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Operation::Algebraic(_), Operation::Logical(_)) => Some(std::cmp::Ordering::Greater),
            (Operation::Logical(_), Operation::Algebraic(_)) => Some(std::cmp::Ordering::Less),
            (Operation::Algebraic(op1), Operation::Algebraic(op2)) => op1.partial_cmp(op2),
            (Operation::Logical(log1), Operation::Logical(log2)) => log1.partial_cmp(log2),
            (Operation::Assignment, Operation::Assignment) => Some(std::cmp::Ordering::Equal),
            (Operation::Assignment, _) => Some(std::cmp::Ordering::Less),
            (_, Operation::Assignment) => Some(std::cmp::Ordering::Greater),
        }
    }
}
