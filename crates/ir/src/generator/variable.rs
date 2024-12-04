use std::{cell::RefCell, rc::Rc};

use inkwell::{builder::Builder, context::Context};
use parser::ast::{
    declaration::{Declaration, VariableDeclarationKeyword},
    expression::Expression,
    unit::ASTUnit,
};

use super::{
    common::{generate_for_literal, VariableData},
    expression::LLVMExpressionGenerator,
    function::{StackFrame, SSA},
};

pub struct LLVMVariableGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
}

impl<'ctx> LLVMVariableGenerator<'ctx> {
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

    pub fn generate_for_ast(
        &self,
        keyword: &VariableDeclarationKeyword,
        identifier: &String,
        expression: &'ctx ASTUnit,
    ) {
        let var_type = self.context.i32_type();

        if *keyword == VariableDeclarationKeyword::Let {
            // mutable variable declaration

            let var = self
                .builder
                .build_alloca(var_type, identifier.as_str())
                .unwrap();
            self.stack_frame.borrow_mut().insert(
                identifier.to_string(),
                VariableData::new(var, var_type.into()),
            );

            match expression {
                ASTUnit::Expression(expr) => {
                    LLVMExpressionGenerator::new(
                        self.context,
                        self.builder,
                        Rc::clone(&self.stack_frame),
                        Rc::clone(&self.ssa),
                    )
                    .generate_from_ast(&format!("{identifier}_tmp"), expr);
                }
                other => panic!("exprected expression, got: {other:?}"),
            };

            self.builder
                .build_store(
                    var,
                    *self.ssa.borrow().get(&format!("{identifier}_tmp")).unwrap(),
                )
                .unwrap();
        } else {
            // immutable variable declaration

            let value = generate_for_literal(
                self.context,
                match expression {
                    ASTUnit::Expression(expr) => match expr {
                        Expression::Literal(literal) => literal,
                        other => panic!("unimplemented: {other:?}"),
                    },
                    other => panic!("exprected expression, got: {other:?}"),
                },
            );

            self.ssa.borrow_mut().insert(identifier.to_string(), value);
        }
    }
}
