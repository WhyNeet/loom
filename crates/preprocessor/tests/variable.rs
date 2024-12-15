use std::rc::Rc;

use lexer::lexer::Lexer;
use parser::{
    ast::{literal::Literal, operation::AlgebraicOperation},
    Parser,
};
use preprocessor::{
    last::{
        declaration::{Declaration, VariableAllocation},
        expression::Expression,
        operation::Operation,
        unit::LASTUnit,
    },
    Preprocessor,
};

#[test]
pub fn variable_declaration_works() {
    let code = r#"
  let a = 1 + 2 + 3;
    "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);
    let root = last.root();

    assert_eq!(
        root[0],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "4".to_string(),
            expression: Rc::new(Expression::Literal(Literal::Int32(3)))
        })
    );

    assert_eq!(
        root[1],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "3".to_string(),
            expression: Rc::new(Expression::Literal(Literal::Int32(2)))
        })
    );

    assert_eq!(
        root[2],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "2".to_string(),
            expression: Rc::new(Expression::BinaryExpression {
                left: Rc::new(Expression::Identifier("3".to_string())),
                right: Rc::new(Expression::Identifier("4".to_string())),
                operation: Operation::Algebraic(AlgebraicOperation::Addition)
            })
        })
    );

    assert_eq!(
        root[3],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "1".to_string(),
            expression: Rc::new(Expression::Literal(Literal::Int32(1)))
        })
    );

    assert_eq!(
        root[4],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "0".to_string(),
            expression: Rc::new(Expression::BinaryExpression {
                left: Rc::new(Expression::Identifier("1".to_string())),
                right: Rc::new(Expression::Identifier("2".to_string())),
                operation: Operation::Algebraic(AlgebraicOperation::Addition)
            })
        })
    );

    assert_eq!(
        root[5],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::Stack,
            identifier: "a".to_string(),
            expression: Rc::new(Expression::Identifier("0".to_string()))
        })
    );
}
