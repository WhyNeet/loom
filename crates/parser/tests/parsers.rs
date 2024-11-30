use lexer::lexer::lexer;
use parser::{
    ast::{
        expression::Expression,
        literal::Literal,
        operation::{AlgebraicOperation, LogicalOperation, Operation},
        unit::ASTUnit,
    },
    parser::parsers,
};

#[test]
pub fn expression_parser_works() {
    let input = r#"(1 + 2) * 3 > x || x > a"#;

    let tokens = lexer(input);
    let (ast, expr_size) = parsers::parse_expression(&tokens);

    assert_eq!(
        ast,
        ASTUnit::Expression(Expression::BinaryExpression {
            left: vec![ASTUnit::Expression(Expression::BinaryExpression {
                left: vec![ASTUnit::Expression(Expression::BinaryExpression {
                    left: vec![ASTUnit::Expression(Expression::BinaryExpression {
                        left: vec![ASTUnit::Expression(Expression::Literal(Literal::Int32(1),))],
                        right: vec![ASTUnit::Expression(Expression::Literal(Literal::Int32(2),))],
                        operation: Operation::Algebraic(AlgebraicOperation::Addition,),
                    })],
                    right: vec![ASTUnit::Expression(Expression::Literal(Literal::Int32(3),))],
                    operation: Operation::Algebraic(AlgebraicOperation::Multiplication,),
                })],
                right: vec![ASTUnit::Expression(Expression::Identifier("x".to_string()))],
                operation: Operation::Logical(LogicalOperation::Greater),
            })],
            right: vec![ASTUnit::Expression(Expression::BinaryExpression {
                left: vec![ASTUnit::Expression(Expression::Identifier("x".to_string()))],
                right: vec![ASTUnit::Expression(Expression::Identifier("a".to_string()))],
                operation: Operation::Logical(LogicalOperation::Greater),
            })],
            operation: Operation::Logical(LogicalOperation::Or),
        })
    );
    assert_eq!(expr_size, 13);
}
