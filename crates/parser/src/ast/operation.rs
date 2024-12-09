#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Algebraic(AlgebraicOperation),
    Logical(LogicalOperation),
    Assignment(AssignmentOperation),
}

impl Operation {
    pub fn from_str(value: &str) -> Option<Self> {
        AlgebraicOperation::from_str(value)
            .map(|alg| Self::Algebraic(alg))
            .or(LogicalOperation::from_str(value).map(|log| Self::Logical(log)))
            .or(AssignmentOperation::from_str(value).map(|assign| Self::Assignment(assign)))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssignmentOperation {
    Assign,
    AdditionAssign,
    SubtractionAssign,
    MultiplicationAssign,
    DivisionAssign,
}

impl AssignmentOperation {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "=" => Some(Self::Assign),
            "+=" => Some(Self::AdditionAssign),
            "-=" => Some(Self::SubtractionAssign),
            "/=" => Some(Self::DivisionAssign),
            "*=" => Some(Self::MultiplicationAssign),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlgebraicOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl AlgebraicOperation {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "+" => Some(Self::Addition),
            "-" => Some(Self::Subtraction),
            "*" => Some(Self::Multiplication),
            "/" => Some(Self::Division),
            _ => None,
        }
    }
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

impl LogicalOperation {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "==" => Some(Self::Equal),
            ">=" => Some(Self::GreaterOrEqual),
            "<=" => Some(Self::LessOrEqual),
            ">" => Some(Self::Greater),
            "<" => Some(Self::Less),
            "||" => Some(Self::Or),
            "&&" => Some(Self::And),
            _ => None,
        }
    }
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
            (Operation::Assignment(_), Operation::Assignment(_)) => Some(std::cmp::Ordering::Equal),
            (Operation::Assignment(_), _) => Some(std::cmp::Ordering::Less),
            (_, Operation::Assignment(_)) => Some(std::cmp::Ordering::Greater),
        }
    }
}
