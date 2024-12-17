// use std::rc::Rc;

// use lexer::lexer::Lexer;
// use parser::ast::{
//     expression::Expression,
//     literal::Literal,
//     operation::{AlgebraicOperation, LogicalOperation, Operation},
//     unit::ASTUnit,
// };

// #[test]
// pub fn expression_parser_works() {
//     let input = r#"(1 + 2) * 3 > x || x > a"#;

//     let tokens = Lexer::new().run(input);
//     let (ast, expr_size) = parsers::parse_expression(&tokens);

//     assert_eq!(
//         ast,
//         ASTUnit::Expression(Expression::BinaryExpression {
//             left: Rc::new(ASTUnit::Expression(Expression::BinaryExpression {
//                 left: Rc::new(ASTUnit::Expression(Expression::BinaryExpression {
//                     left: Rc::new(ASTUnit::Expression(Expression::BinaryExpression {
//                         left: Rc::new(ASTUnit::Expression(Expression::Literal(Literal::Int32(1),))),
//                         right: Rc::new(ASTUnit::Expression(
//                             Expression::Literal(Literal::Int32(2),)
//                         )),
//                         operation: Operation::Algebraic(AlgebraicOperation::Addition,),
//                     })),
//                     right: Rc::new(ASTUnit::Expression(Expression::Literal(Literal::Int32(3),))),
//                     operation: Operation::Algebraic(AlgebraicOperation::Multiplication,),
//                 })),
//                 right: Rc::new(ASTUnit::Expression(Expression::Identifier("x".to_string()))),
//                 operation: Operation::Logical(LogicalOperation::Greater),
//             })),
//             right: Rc::new(ASTUnit::Expression(Expression::BinaryExpression {
//                 left: Rc::new(ASTUnit::Expression(Expression::Identifier("x".to_string()))),
//                 right: Rc::new(ASTUnit::Expression(Expression::Identifier("a".to_string()))),
//                 operation: Operation::Logical(LogicalOperation::Greater),
//             })),
//             operation: Operation::Logical(LogicalOperation::Or),
//         })
//     );
//     assert_eq!(expr_size, 13);
// }
