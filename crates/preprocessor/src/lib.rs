pub mod last;
pub mod mangler;

use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use last::{
    declaration::{Declaration, VariableAllocation},
    expression::Expression,
    statement::Statement,
    unit::LASTUnit,
    LoweredAbstractSyntaxTree,
};
use mangler::Mangler;
use parser::ast::{unit::ASTUnit, AbstractSyntaxTree};

pub struct Preprocessor {
    fn_mangler: Mangler,
}

impl Preprocessor {
    pub fn new() -> Self {
        Self {
            fn_mangler: Mangler::new(),
        }
    }

    pub fn run(&self, ast: AbstractSyntaxTree) -> LoweredAbstractSyntaxTree {
        let root = match ast.root() {
            ASTUnit::Block(block) => block,
            _ => unreachable!(),
        };

        let last_root = root
            .iter()
            .map(Rc::clone)
            .map(|unit| self.run_internal(unit, &Mangler::new(), None))
            .flatten()
            .collect();

        let last = LoweredAbstractSyntaxTree::new(last_root);

        last
    }

    fn run_internal(
        &self,
        unit: Rc<ASTUnit>,
        mangler: &Mangler,
        store_result_in: Option<String>,
    ) -> Vec<LASTUnit> {
        match unit.as_ref() {
            ASTUnit::Declaration(declaration) => self.run_declaration(declaration, mangler),
            ASTUnit::Expression(expression) => self.run_expression(
                expression,
                store_result_in.unwrap_or_else(|| mangler.rng()),
                mangler,
            ),
            ASTUnit::Statement(statement) => self.run_statement(statement, mangler, None),
            _ => todo!(),
        }
    }

    fn run_statement(
        &self,
        statement: &parser::ast::statement::Statement,
        mangler: &Mangler,
        store_result_in: Option<String>,
    ) -> Vec<LASTUnit> {
        let mut last_units = vec![];

        let statement_unit = match statement {
            parser::ast::statement::Statement::Return(ret) => {
                let ret_ssa_name = mangler.rng();
                let mut ret_value =
                    self.run_internal(Rc::clone(ret), mangler, Some(ret_ssa_name.clone()));
                last_units.append(&mut ret_value);

                LASTUnit::Statement(Statement::Return(Expression::Identifier(ret_ssa_name)))
            }
            parser::ast::statement::Statement::ImplicitReturn(ret) => {
                let ret_ssa_name = mangler.rng();
                let mut ret_value =
                    self.run_internal(Rc::clone(ret), mangler, Some(ret_ssa_name.clone()));
                last_units.append(&mut ret_value);

                if let Some(store_result_in) = store_result_in {
                    LASTUnit::Declaration(Declaration::VariableDeclaration {
                        allocation: VariableAllocation::SSA,
                        identifier: store_result_in,
                        expression: Rc::new(Expression::Identifier(ret_ssa_name)),
                    })
                } else {
                    LASTUnit::Statement(Statement::Return(Expression::Identifier(ret_ssa_name)))
                }
            }
            _ => todo!(),
        };

        last_units.push(statement_unit);

        last_units
    }

    fn run_declaration(
        &self,
        declaration: &parser::ast::declaration::Declaration,
        mangler: &Mangler,
    ) -> Vec<LASTUnit> {
        let mut last_units = vec![];

        let declaration_unit = match declaration {
            parser::ast::declaration::Declaration::FunctionDeclaration {
                identifier,
                parameters,
                return_type,
                expression,
            } => {
                let identifier = self.fn_mangler.mangle(Cow::Borrowed(identifier));

                let declaration = Declaration::FunctionDeclaration {
                    identifier,
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    body: match expression.as_ref() {
                        ASTUnit::Block(block) => block,
                        _ => unreachable!(),
                    }
                    .iter()
                    .map(Rc::clone)
                    .map(|unit| self.run_internal(unit, mangler, None))
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
                let ident_tmp = mangler.rng();

                let expr_mangler = mangler.submangler();
                let mut expression_result = self.run_expression(
                    match expression.as_ref() {
                        ASTUnit::Expression(expression) => expression,
                        _ => unreachable!(),
                    },
                    ident_tmp.clone(),
                    &expr_mangler,
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
                    expression: Rc::new(Expression::Identifier(ident_tmp)),
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
        mangler: &Mangler,
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
                let lhs_ssa_name = mangler.rng();
                let mut lhs_expr = self.run_expression(lhs, lhs_ssa_name.clone(), mangler);

                let rhs = match right.as_ref() {
                    ASTUnit::Expression(expr) => expr,
                    _ => unreachable!(),
                };
                let rhs_ssa_name = mangler.rng();
                let mut rhs_expr = self.run_expression(rhs, rhs_ssa_name.clone(), mangler);

                expression_units.append(&mut rhs_expr);
                expression_units.append(&mut lhs_expr);

                Expression::BinaryExpression {
                    left: Rc::new(Expression::Identifier(lhs_ssa_name)),
                    right: Rc::new(Expression::Identifier(rhs_ssa_name)),
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
