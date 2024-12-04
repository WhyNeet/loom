use std::{cell::RefCell, rc::Rc};

use inkwell::{builder::Builder, context::Context};
use parser::ast::{
    expression::Expression,
    operation::{AlgebraicOperation, AssignmentOperation, Operation},
    unit::ASTUnit,
};

use super::{
    common::generate_for_literal,
    function::{StackFrame, SSA},
};

pub struct LLVMExpressionGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
}

impl<'ctx> LLVMExpressionGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
        ssa: Rc<RefCell<SSA<'ctx>>>,
    ) -> Self {
        Self {
            builder,
            context,
            stack_frame,
            ssa,
        }
    }

    pub fn generate_from_ast(&self, store_in: &str, expression: &'ctx Expression) {
        match expression {
            Expression::Literal(literal) => {
                self.ssa.borrow_mut().insert(
                    store_in.to_string(),
                    generate_for_literal(self.context, literal),
                );
            }
            Expression::Identifier(ident) => {
                let value = if let Some(&basic) = self.ssa.borrow().get(ident) {
                    basic
                } else {
                    let sf = self.stack_frame.borrow();
                    let variable_data = sf.get(ident).unwrap();
                    self.builder
                        .build_load(variable_data.ty(), variable_data.ptr(), ident)
                        .unwrap()
                };

                self.ssa.borrow_mut().insert(store_in.to_string(), value);
            }
            Expression::FunctionInvokation { .. } => {
                panic!("function invokation isnt implemented yet")
            }
            Expression::BinaryExpression {
                left,
                right,
                operation,
            } => {
                let left_store_in = format!("{store_in}_lhs");
                let right_store_in = format!("{store_in}_rhs");

                self.generate_from_ast(
                    &left_store_in,
                    match &(**left) {
                        ASTUnit::Expression(expr) => expr,
                        _ => todo!(),
                    },
                );

                self.generate_from_ast(
                    &right_store_in,
                    match &(**right) {
                        ASTUnit::Expression(expr) => expr,
                        _ => todo!(),
                    },
                );

                let lhs = self
                    .ssa
                    .borrow()
                    .get(&left_store_in)
                    .unwrap()
                    .into_int_value();
                let rhs = self
                    .ssa
                    .borrow()
                    .get(&right_store_in)
                    .unwrap()
                    .into_int_value();

                let op_res = match operation {
                    Operation::Algebraic(alg) => match alg {
                        AlgebraicOperation::Addition => {
                            self.builder.build_int_add(lhs, rhs, store_in)
                        }
                        AlgebraicOperation::Division => {
                            self.builder.build_int_signed_div(lhs, rhs, store_in)
                        }
                        AlgebraicOperation::Multiplication => {
                            self.builder.build_int_mul(lhs, rhs, store_in)
                        }
                        AlgebraicOperation::Subtraction => {
                            self.builder.build_int_sub(lhs, rhs, store_in)
                        }
                    },
                    _ => panic!("logical operations are not yet implemented"),
                }
                .unwrap();

                self.ssa
                    .borrow_mut()
                    .insert(store_in.to_string(), op_res.into());
            }
        };
    }
}
