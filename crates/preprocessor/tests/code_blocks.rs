use std::rc::Rc;

use lexer::lexer::Lexer;
use parser::{ast::operation::AlgebraicOperation, Parser};
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
pub fn code_blocks_work() {
    let code = r#"
    let b = 1;

    let a = {
      let b = 2;

      b + c
    };

    let c = b;
  "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);

    let root = last.root();

    assert_eq!(
        root[0],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "0".to_string(),
            expression: Rc::new(Expression::Literal(parser::ast::literal::Literal::Int32(1)))
        })
    );
    assert_eq!(
        root[1],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::Stack,
            identifier: "b".to_string(),
            expression: Rc::new(Expression::Identifier("0".to_string()))
        })
    );
    assert_eq!(
        root[2],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "1".to_string(),
            expression: Rc::new(Expression::Literal(parser::ast::literal::Literal::Int32(2)))
        })
    );
    assert_eq!(
        root[3],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::Stack,
            identifier: "b0".to_string(),
            expression: Rc::new(Expression::Identifier("1".to_string()))
        })
    );
    assert_eq!(
        root[4],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "4".to_string(),
            expression: Rc::new(Expression::Identifier("c".to_string()))
        })
    );
    assert_eq!(
        root[5],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "3".to_string(),
            expression: Rc::new(Expression::Identifier("b0".to_string()))
        })
    );
    assert_eq!(
        root[6],
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
        root[7],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "0".to_string(),
            expression: Rc::new(Expression::Identifier("2".to_string()))
        })
    );
    assert_eq!(
        root[8],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::Stack,
            identifier: "a".to_string(),
            expression: Rc::new(Expression::Identifier("0".to_string()))
        })
    );
    assert_eq!(
        root[9],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "0".to_string(),
            expression: Rc::new(Expression::Identifier("b".to_string()))
        })
    );
    assert_eq!(
        root[10],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::Stack,
            identifier: "c".to_string(),
            expression: Rc::new(Expression::Identifier("0".to_string()))
        })
    );
}
