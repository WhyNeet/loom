use lexer::lexer::lexer;
use parser::{
    ast::{ASTUnit, Expression},
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
                        left: vec![ASTUnit::Expression(Expression::Literal(
                            parser::ast::Literal::Int32(1),
                        ))],
                        right: vec![ASTUnit::Expression(Expression::Literal(
                            parser::ast::Literal::Int32(2),
                        ))],
                        operation: parser::ast::Operation::Algebraic(
                            parser::ast::AlgebraicOperation::Addition,
                        ),
                    })],
                    right: vec![ASTUnit::Expression(Expression::Literal(
                        parser::ast::Literal::Int32(3),
                    ))],
                    operation: parser::ast::Operation::Algebraic(
                        parser::ast::AlgebraicOperation::Multiplication,
                    ),
                })],
                right: vec![ASTUnit::Expression(Expression::Identifier("x".to_string()))],
                operation: parser::ast::Operation::Logical(parser::ast::LogicalOperation::Greater),
            })],
            right: vec![ASTUnit::Expression(Expression::BinaryExpression {
                left: vec![ASTUnit::Expression(Expression::Identifier("x".to_string()))],
                right: vec![ASTUnit::Expression(Expression::Identifier("a".to_string()))],
                operation: parser::ast::Operation::Logical(parser::ast::LogicalOperation::Greater),
            })],
            operation: parser::ast::Operation::Logical(parser::ast::LogicalOperation::Or),
        })
    );
    assert_eq!(expr_size, 13);
}
