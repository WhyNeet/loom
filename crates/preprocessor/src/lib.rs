pub mod last;
pub mod mangler;
pub mod scope;

use std::{borrow::Cow, collections::HashMap, rc::Rc};

use last::{
    declaration::{Declaration, VariableAllocation},
    expression::Expression,
    statement::Statement,
    unit::LASTUnit,
    LoweredAbstractSyntaxTree,
};
use mangler::Mangler;
use parser::ast::{unit::ASTUnit, AbstractSyntaxTree};
use scope::{Remapper, Scope};

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

        let root_scope = Scope::new();

        let last_root = root
            .iter()
            .map(Rc::clone)
            .map(|unit| self.run_internal(unit, &Mangler::new(), None, Some(&root_scope), None))
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
        scope: Option<&Scope>,
        remap: Option<&Remapper>,
    ) -> Vec<LASTUnit> {
        match unit.as_ref() {
            ASTUnit::Declaration(declaration) => {
                self.run_declaration(declaration, mangler, scope.unwrap_or(&Scope::new()), remap)
            }
            ASTUnit::Expression(expression) => self.run_expression(
                expression,
                store_result_in.unwrap_or_else(|| mangler.rng()),
                mangler,
                remap,
            ),
            ASTUnit::Statement(statement) => self.run_statement(
                statement,
                mangler,
                store_result_in,
                scope.unwrap_or(&Scope::new()),
                remap,
            ),
            ASTUnit::Block(block) => self.run_block(
                block,
                mangler,
                store_result_in,
                scope.unwrap_or(&Scope::new()),
                remap,
            ),
        }
    }

    fn run_block(
        &self,
        block: &Vec<Rc<ASTUnit>>,
        mangler: &Mangler,
        store_result_in: Option<String>,
        scope: &Scope,
        remap: Option<&Remapper>,
    ) -> Vec<LASTUnit> {
        let remaps_new = Remapper::new();

        block
            .iter()
            .map(Rc::clone)
            .map(|unit| {
                self.run_internal(
                    unit,
                    mangler,
                    store_result_in.clone(),
                    Some(scope),
                    Some(remap.unwrap_or(&remaps_new)),
                )
            })
            .flatten()
            .collect()
    }

    fn run_statement(
        &self,
        statement: &parser::ast::statement::Statement,
        mangler: &Mangler,
        store_result_in: Option<String>,
        scope: &Scope,
        remap: Option<&Remapper>,
    ) -> Vec<LASTUnit> {
        let mut last_units = vec![];

        let statement_unit = match statement {
            parser::ast::statement::Statement::Return(ret) => {
                let ret_ssa_name = mangler.rng();
                let mut ret_value = self.run_internal(
                    Rc::clone(ret),
                    mangler,
                    Some(ret_ssa_name.clone()),
                    Some(scope),
                    remap,
                );
                last_units.append(&mut ret_value);

                LASTUnit::Statement(Statement::Return(Expression::Identifier(ret_ssa_name)))
            }
            parser::ast::statement::Statement::ImplicitReturn(ret) => {
                let ret_ssa_name = mangler.rng();
                let mut ret_value = self.run_internal(
                    Rc::clone(ret),
                    mangler,
                    Some(ret_ssa_name.clone()),
                    Some(scope),
                    remap,
                );
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
            parser::ast::statement::Statement::ControlFlow {
                condition,
                execute,
                alternative,
            } => {
                let condition_ssa_name = mangler.rng();
                let mut condition_value = self.run_internal(
                    Rc::clone(condition),
                    mangler,
                    Some(condition_ssa_name.clone()),
                    Some(scope),
                    remap,
                );

                last_units.append(&mut condition_value);

                let result_ssa_name = mangler.rng();

                let execute_value = self.run_internal(
                    Rc::clone(execute),
                    mangler,
                    Some(result_ssa_name.clone()),
                    Some(scope),
                    remap,
                );

                let alternative_value = if let Some(alternative) = alternative {
                    Some(self.run_internal(
                        Rc::clone(alternative),
                        mangler,
                        Some(result_ssa_name.clone()),
                        Some(scope),
                        remap,
                    ))
                } else {
                    None
                };

                LASTUnit::Statement(Statement::ControlFlow {
                    condition: Rc::new(Expression::Identifier(condition_ssa_name)),
                    execute: execute_value,
                    alternative: alternative_value,
                })
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
        scope: &Scope,
        remap: Option<&Remapper>,
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

                let fn_scope = Scope::new();

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
                    .map(|unit| self.run_internal(unit, mangler, None, Some(&fn_scope), remap))
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

                let mut expression_result = match expression.as_ref() {
                    ASTUnit::Expression(expression) => {
                        self.run_expression(expression, ident_tmp.clone(), mangler, remap)
                    }
                    ASTUnit::Block(block) => {
                        self.run_block(block, mangler, Some(ident_tmp.clone()), scope, remap)
                    }
                    ASTUnit::Statement(statement) => self.run_statement(
                        statement,
                        mangler,
                        Some(ident_tmp.clone()),
                        scope,
                        remap,
                    ),
                    ASTUnit::Declaration(_) => panic!("cannot use declaration in expression"),
                };

                last_units.append(&mut expression_result);

                let identifier_new = scope.add_to_stack(identifier.clone());

                if &identifier_new != identifier {
                    if let Some(remaps) = remap {
                        remaps.remap(identifier.clone(), identifier_new.clone());
                    }
                }

                let declaration = Declaration::VariableDeclaration {
                    allocation: match keyword {
                        parser::ast::declaration::VariableDeclarationKeyword::Const => {
                            VariableAllocation::SSA
                        }
                        parser::ast::declaration::VariableDeclarationKeyword::Let => {
                            VariableAllocation::Stack
                        }
                    },
                    identifier: identifier_new,
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
        remap: Option<&Remapper>,
    ) -> Vec<LASTUnit> {
        let mut expression_units = vec![];

        let expression_result = match expression {
            parser::ast::expression::Expression::Identifier(ident) => {
                Expression::Identifier(if let Some(remap) = remap {
                    if let Some(remapped) = remap.get_remapped(ident) {
                        remapped
                    } else {
                        ident.clone()
                    }
                } else {
                    ident.clone()
                })
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
                    name: self.fn_mangler.mangle(Cow::Borrowed(function_name)),
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
                let mut lhs_expr = self.run_expression(lhs, lhs_ssa_name.clone(), mangler, remap);

                let rhs = match right.as_ref() {
                    ASTUnit::Expression(expr) => expr,
                    _ => unreachable!(),
                };
                let rhs_ssa_name = mangler.rng();
                let mut rhs_expr = self.run_expression(rhs, rhs_ssa_name.clone(), mangler, remap);

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
