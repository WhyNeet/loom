use std::{cell::RefCell, rc::Rc};

use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicMetadataValueEnum, BasicValueEnum},
    IntPredicate,
};
use preprocessor::last::{
    expression::Expression,
    operation::{AlgebraicOperation, LogicalOperation, Operation},
};

use super::{
    common::generate_for_literal,
    function::{StackFrame, SSA},
    module::FunctionStack,
};

pub struct LLVMExpressionGenerator<'ctx> {
    builder: Rc<Builder<'ctx>>,
    context: &'ctx Context,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
    function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
}

impl<'ctx> LLVMExpressionGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: Rc<Builder<'ctx>>,
        stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
        ssa: Rc<RefCell<SSA<'ctx>>>,
        function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
    ) -> Self {
        Self {
            builder,
            context,
            stack_frame,
            ssa,
            function_stack,
        }
    }

    pub fn generate_from_ast(
        &self,
        expression: &'ctx Expression,
        store_in: Option<&str>,
    ) -> Option<BasicValueEnum<'ctx>> {
        match expression {
            Expression::Literal(literal) => {
                return Some(generate_for_literal(self.context, literal));
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

                return Some(value);
            }
            Expression::FunctionInvokation { args, name } => {
                let params = args
                    .into_iter()
                    .map(|param| {
                        self.generate_from_ast(
                            param,
                            match param {
                                Expression::Identifier(ident) => Some(ident),
                                _ => unreachable!(),
                            },
                        )
                        .unwrap()
                    })
                    .map(|param| param.into())
                    .collect::<Vec<BasicMetadataValueEnum<'ctx>>>();

                let instruct = self
                    .builder
                    .build_call(
                        *self.function_stack.borrow().get(name).unwrap(),
                        &params,
                        store_in.unwrap(),
                    )
                    .unwrap();

                return instruct.try_as_basic_value().left();
            }
            Expression::BinaryExpression {
                left,
                right,
                operation,
            } => {
                println!("binary expression: {operation:?}");

                let op_res = match operation {
                    Operation::Algebraic(alg) => {
                        let lhs = self
                            .generate_from_ast(
                                left,
                                match left.as_ref() {
                                    Expression::Identifier(ident) => Some(ident),
                                    _ => unreachable!(),
                                },
                            )
                            .unwrap()
                            .into_int_value();

                        let rhs = self
                            .generate_from_ast(
                                right,
                                match right.as_ref() {
                                    Expression::Identifier(ident) => Some(ident),
                                    _ => unreachable!(),
                                },
                            )
                            .unwrap()
                            .into_int_value();
                        Some(match alg {
                            AlgebraicOperation::Addition => {
                                self.builder.build_int_add(lhs, rhs, store_in.unwrap())
                            }
                            AlgebraicOperation::Division => {
                                self.builder
                                    .build_int_signed_div(lhs, rhs, store_in.unwrap())
                            }
                            AlgebraicOperation::Multiplication => {
                                self.builder.build_int_mul(lhs, rhs, store_in.unwrap())
                            }
                            AlgebraicOperation::Subtraction => {
                                self.builder.build_int_sub(lhs, rhs, store_in.unwrap())
                            }
                        })
                    }
                    Operation::Assignment => {
                        let identifier = match left.as_ref() {
                            Expression::Identifier(ident) => ident,
                            _ => unreachable!(),
                        };

                        let rhs = self
                            .generate_from_ast(
                                right,
                                match right.as_ref() {
                                    Expression::Identifier(ident) => Some(ident),
                                    _ => unreachable!(),
                                },
                            )
                            .unwrap()
                            .into_int_value();

                        let lhs = self
                            .stack_frame
                            .borrow()
                            .get(identifier.as_str())
                            .unwrap()
                            .ptr();

                        self.builder.build_store(lhs, rhs).unwrap();

                        println!("build store for: {identifier:?}");

                        None
                    }
                    Operation::Logical(log) => match log {
                        LogicalOperation::Or => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_or(lhs, rhs, store_in.unwrap()))
                        }
                        LogicalOperation::And => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_and(lhs, rhs, store_in.unwrap()))
                        }
                        LogicalOperation::Equal => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::EQ,
                                lhs,
                                rhs,
                                store_in.unwrap(),
                            ))
                        }
                        LogicalOperation::Less => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SLT,
                                lhs,
                                rhs,
                                store_in.unwrap(),
                            ))
                        }
                        LogicalOperation::Greater => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SGT,
                                lhs,
                                rhs,
                                store_in.unwrap(),
                            ))
                        }
                        LogicalOperation::LessOrEqual => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SLE,
                                lhs,
                                rhs,
                                store_in.unwrap(),
                            ))
                        }
                        LogicalOperation::GreaterOrEqual => {
                            let lhs = self
                                .generate_from_ast(
                                    left,
                                    match left.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    right,
                                    match right.as_ref() {
                                        Expression::Identifier(ident) => Some(ident),
                                        _ => unreachable!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SGE,
                                lhs,
                                rhs,
                                store_in.unwrap(),
                            ))
                        }
                    },
                };

                return op_res.map(|res| res.unwrap()).map(|intv| intv.into());
            }
        };
    }
}
