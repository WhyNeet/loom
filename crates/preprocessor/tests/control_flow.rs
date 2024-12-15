use std::rc::Rc;

use lexer::lexer::Lexer;
use parser::{ast::literal::Literal, Parser};
use preprocessor::{
    last::{
        declaration::{Declaration, VariableAllocation},
        expression::Expression,
        operation::Operation,
        statement::Statement,
        unit::LASTUnit,
    },
    Preprocessor,
};

#[test]
pub fn control_flow_works() {
    let code = r#"
    if a > b {
      1
    } else {
      2
    }
    "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);

    let root = last.root();

    assert_eq!(
        root[0],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "2".to_string(),
            expression: Rc::new(Expression::Identifier("b".to_string()))
        })
    );

    assert_eq!(
        root[1],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "1".to_string(),
            expression: Rc::new(Expression::Identifier("a".to_string()))
        })
    );

    assert_eq!(
        root[2],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "0".to_string(),
            expression: Rc::new(Expression::BinaryExpression {
                left: Rc::new(Expression::Identifier("1".to_string())),
                right: Rc::new(Expression::Identifier("2".to_string())),
                operation: Operation::Logical(parser::ast::operation::LogicalOperation::Greater)
            })
        })
    );

    assert_eq!(
        root[3],
        LASTUnit::Statement(Statement::ControlFlow {
            condition: Rc::new(Expression::Identifier("0".to_string())),
            execute: vec![
                LASTUnit::Declaration(Declaration::VariableDeclaration {
                    allocation: VariableAllocation::SSA,
                    identifier: "4".to_string(),
                    expression: Rc::new(Expression::Literal(Literal::Int32(1)))
                }),
                LASTUnit::Declaration(Declaration::VariableDeclaration {
                    allocation: VariableAllocation::SSA,
                    identifier: "3".to_string(),
                    expression: Rc::new(Expression::Identifier("4".to_string()))
                })
            ],
            alternative: Some(vec![
                LASTUnit::Declaration(Declaration::VariableDeclaration {
                    allocation: VariableAllocation::SSA,
                    identifier: "5".to_string(),
                    expression: Rc::new(Expression::Literal(Literal::Int32(2)))
                }),
                LASTUnit::Declaration(Declaration::VariableDeclaration {
                    allocation: VariableAllocation::SSA,
                    identifier: "3".to_string(),
                    expression: Rc::new(Expression::Identifier("5".to_string()))
                })
            ])
        })
    );
}
