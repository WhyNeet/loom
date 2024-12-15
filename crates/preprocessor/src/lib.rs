pub mod last;

use std::rc::Rc;

use last::{
    declaration::{Declaration, VariableAllocation},
    expression::Expression,
    unit::LASTUnit,
    LoweredAbstractSyntaxTree,
};
use parser::ast::{unit::ASTUnit, AbstractSyntaxTree};

pub struct Preprocessor {}

impl Preprocessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, ast: AbstractSyntaxTree) -> LoweredAbstractSyntaxTree {
        let root = match ast.root() {
            ASTUnit::Block(block) => block,
            _ => unreachable!(),
        };

        let last_root = root
            .iter()
            .map(Rc::clone)
            .map(|unit| self.run_internal(unit))
            .flatten()
            .collect();

        let last = LoweredAbstractSyntaxTree::new(last_root);

        last
    }

    fn run_internal(&self, unit: Rc<ASTUnit>) -> Vec<LASTUnit> {
        match unit.as_ref() {
            ASTUnit::Declaration(declaration) => self.run_declaration(declaration),
            ASTUnit::Expression(expression) => {
                self.run_expression(expression, "expr_res".to_string())
            }
            _ => todo!(),
        }
    }

    fn run_declaration(
        &self,
        declaration: &parser::ast::declaration::Declaration,
    ) -> Vec<LASTUnit> {
        let mut last_units = vec![];

        let declaration_unit = match declaration {
            parser::ast::declaration::Declaration::FunctionDeclaration {
                identifier,
                parameters,
                return_type,
                expression,
            } => {
                let declaration = Declaration::FunctionDeclaration {
                    identifier: identifier.clone(),
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    body: match expression.as_ref() {
                        ASTUnit::Block(block) => block,
                        _ => unreachable!(),
                    }
                    .iter()
                    .map(Rc::clone)
                    .map(|unit| self.run_internal(unit))
                    .flatten()
                    .collect(),
                };

                LASTUnit::Declaration(declaration)
            }
            parser::ast::declaration::Declaration::VariableDeclaration {
                keyword,
                identifier,
                expression,
            } => {
                let mut expression_result = self.run_expression(
                    match expression.as_ref() {
                        ASTUnit::Expression(expression) => expression,
                        _ => unreachable!(),
                    },
                    format!("{identifier}_tmp"),
                );

                last_units.append(&mut expression_result);

                let declaration = Declaration::VariableDeclaration {
                    allocation: match keyword {
                        parser::ast::declaration::VariableDeclarationKeyword::Const => {
                            VariableAllocation::SSA
                        }
                        parser::ast::declaration::VariableDeclarationKeyword::Let => {
                            VariableAllocation::Stack
                        }
                    },
                    identifier: identifier.clone(),
                    expression: Rc::new(Expression::Identifier(format!("{identifier}_tmp"))),
                };

                LASTUnit::Declaration(declaration)
            }
        };

        last_units.push(declaration_unit);

        last_units
    }

    fn run_expression(
        &self,
        expression: &parser::ast::expression::Expression,
        identifier: String,
    ) -> Vec<LASTUnit> {
        let mut expression_units = vec![];

        let expression_result = match expression {
            parser::ast::expression::Expression::Identifier(ident) => {
                Expression::Identifier(ident.clone())
            }
            parser::ast::expression::Expression::Literal(literal) => {
                Expression::Literal(literal.clone())
            }
            parser::ast::expression::Expression::FunctionInvokation {
                function_name,
                parameters,
            } => {
                let args = vec![];

                // for param in parameters {
                //   match param {
                //     ASTUnit::Declaration(_) => panic!("cannot use declaration as function argument"),
                //     ASTUnit::
                //   }
                // }

                Expression::FunctionInvokation {
                    name: function_name.clone(),
                    args,
                }
            }
            parser::ast::expression::Expression::BinaryExpression {
                left,
                right,
                operation,
            } => {
                let lhs = match left.as_ref() {
                    ASTUnit::Expression(expr) => expr,
                    _ => unreachable!(),
                };
                let mut lhs_expr = self.run_expression(lhs, "expr_lhs_res".to_string());

                expression_units.append(&mut lhs_expr);

                let rhs = match right.as_ref() {
                    ASTUnit::Expression(expr) => expr,
                    _ => unreachable!(),
                };
                let mut rhs_expr = self.run_expression(rhs, "expr_rhs_res".to_string());

                expression_units.append(&mut rhs_expr);

                Expression::BinaryExpression {
                    left: Rc::new(Expression::Identifier("expr_lhs_res".to_string())),
                    right: Rc::new(Expression::Identifier("expr_rhs_res".to_string())),
                    operation: match operation {
                        parser::ast::operation::Operation::Algebraic(_)
                        | parser::ast::operation::Operation::Logical(_) => operation.into(),
                        parser::ast::operation::Operation::Assignment(_) => todo!(),
                    },
                }
            }
        };

        let result_ssa = LASTUnit::Declaration(Declaration::VariableDeclaration {
            allocation: VariableAllocation::SSA,
            identifier,
            expression: Rc::new(expression_result),
        });

        expression_units.push(result_ssa);

        expression_units
    }
}
