use std::{mem, rc::Rc};

use common::types::Type;
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
        statement::Statement,
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

#[test]
pub fn function_declaration_works() {
    let code = r#"
    fun add(x: i32, y: i32) -> i32 {
      return x + y;
    }
    "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);
    let root = last.root();

    assert_eq!(
        mem::discriminant(&root[0]),
        mem::discriminant(&LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "".to_string(),
            expression: Rc::new(Expression::Literal(Literal::Int8(0)))
        }))
    );

    let root = match &root[0] {
        LASTUnit::Declaration(decl) => {
            assert_eq!(
                mem::discriminant(decl),
                mem::discriminant(&Declaration::FunctionDeclaration {
                    identifier: "".to_string(),
                    parameters: vec![],
                    return_type: common::types::Type::Void,
                    body: vec![]
                })
            );

            match decl {
                Declaration::FunctionDeclaration {
                    identifier,
                    parameters,
                    return_type,
                    body,
                } => {
                    assert_eq!(identifier, "0");
                    assert_eq!(parameters[0], ("x".to_string(), Type::Int32));
                    assert_eq!(parameters[1], ("y".to_string(), Type::Int32));
                    assert_eq!(return_type, &Type::Int32);

                    body
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    assert_eq!(
        root[0],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "2".to_string(),
            expression: Rc::new(Expression::Identifier("y".to_string()))
        })
    );
    assert_eq!(
        root[1],
        LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier: "1".to_string(),
            expression: Rc::new(Expression::Identifier("x".to_string()))
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
                operation: Operation::Algebraic(AlgebraicOperation::Addition)
            })
        })
    );
    assert_eq!(
        root[3],
        LASTUnit::Statement(Statement::Return(Expression::Identifier("0".to_string())))
    )
}

#[test]
pub fn function_implicit_return_works() {
    let code = r#"
  fun add(x: i32, y: i32) -> i32 {
    return x + y;
  }
  "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);

    let root_explicit = last.root();

    let code = r#"
  fun add(x: i32, y: i32) -> i32 {
    x + y
  }
  "#;

    let tokens = Lexer::new().run(code);
    let ast = Parser::new().run(&tokens);

    let last = Preprocessor::new().run(ast);

    let root_implicit = last.root();

    assert_eq!(root_explicit, root_implicit);
}

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
