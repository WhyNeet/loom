use std::{cell::RefCell, rc::Rc};

use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicMetadataValueEnum, BasicValueEnum},
    IntPredicate,
};
use parser::ast::{
    expression::Expression,
    operation::{AlgebraicOperation, AssignmentOperation, LogicalOperation, Operation},
    unit::ASTUnit,
};

use super::{
    common::generate_for_literal,
    function::{StackFrame, SSA},
    module::FunctionStack,
};

pub struct LLVMExpressionGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
    function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
}

impl<'ctx> LLVMExpressionGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
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
        store_in: &str,
        expression: &'ctx Expression,
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
            Expression::FunctionInvokation {
                function_name,
                parameters,
            } => {
                let params = parameters
                    .into_iter()
                    .enumerate()
                    .map(|(idx, param)| {
                        self.generate_from_ast(
                            &format!("{idx}"),
                            match *param {
                                ASTUnit::Expression(ref expr) => expr,
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
                        *self.function_stack.borrow().get(function_name).unwrap(),
                        &params,
                        store_in,
                    )
                    .unwrap();

                return instruct.try_as_basic_value().left();
            }
            Expression::BinaryExpression {
                left,
                right,
                operation,
            } => {
                let op_res = match operation {
                    Operation::Algebraic(alg) => {
                        let left_store_in = format!("{store_in}_lhs");
                        let right_store_in = format!("{store_in}_rhs");

                        let lhs = self
                            .generate_from_ast(
                                &left_store_in,
                                match &(**left) {
                                    ASTUnit::Expression(expr) => expr,
                                    _ => todo!(),
                                },
                            )
                            .unwrap()
                            .into_int_value();

                        let rhs = self
                            .generate_from_ast(
                                &right_store_in,
                                match &(**right) {
                                    ASTUnit::Expression(expr) => expr,
                                    _ => todo!(),
                                },
                            )
                            .unwrap()
                            .into_int_value();
                        Some(match alg {
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
                        })
                    }
                    Operation::Assignment(assign) => {
                        let identifier = match &**left {
                            ASTUnit::Expression(var_name) => match var_name {
                                Expression::Identifier(ident) => ident,
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };

                        let right_store_in = format!("{store_in}_rhs");

                        let rhs = self
                            .generate_from_ast(
                                &right_store_in,
                                match &(**right) {
                                    ASTUnit::Expression(expr) => expr,
                                    _ => todo!(),
                                },
                            )
                            .unwrap();

                        let lhs = self
                            .stack_frame
                            .borrow()
                            .get(identifier.as_str())
                            .unwrap()
                            .ptr();

                        match assign {
                            AssignmentOperation::Assign => self.builder.build_store(lhs, rhs),
                            _ => {
                                panic!("other assignment operators will be simplifed using parser")
                            }
                        }
                        .unwrap();

                        None
                    }
                    Operation::Logical(log) => match log {
                        LogicalOperation::Or => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_or(lhs, rhs, store_in))
                        }
                        LogicalOperation::And => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_and(lhs, rhs, store_in))
                        }
                        LogicalOperation::Equal => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::EQ,
                                lhs,
                                rhs,
                                store_in,
                            ))
                        }
                        LogicalOperation::Less => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SLT,
                                lhs,
                                rhs,
                                store_in,
                            ))
                        }
                        LogicalOperation::Greater => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SGT,
                                lhs,
                                rhs,
                                store_in,
                            ))
                        }
                        LogicalOperation::LessOrEqual => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SLE,
                                lhs,
                                rhs,
                                store_in,
                            ))
                        }
                        LogicalOperation::GreaterOrEqual => {
                            let left_store_in = format!("{store_in}_lhs");
                            let right_store_in = format!("{store_in}_rhs");

                            let lhs = self
                                .generate_from_ast(
                                    &left_store_in,
                                    match &(**left) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            let rhs = self
                                .generate_from_ast(
                                    &right_store_in,
                                    match &(**right) {
                                        ASTUnit::Expression(expr) => expr,
                                        _ => todo!(),
                                    },
                                )
                                .unwrap()
                                .into_int_value();

                            Some(self.builder.build_int_compare(
                                IntPredicate::SGE,
                                lhs,
                                rhs,
                                store_in,
                            ))
                        }
                    },
                };

                return op_res.map(|res| res.unwrap()).map(|intv| intv.into());
            }
        };
    }
}
