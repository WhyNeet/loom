use std::{cell::RefCell, rc::Rc};

use inkwell::{builder::Builder, context::Context};
use parser::ast::{
    declaration::{Declaration, VariableDeclarationKeyword},
    expression::Expression,
    unit::ASTUnit,
};

use super::{
    common::{generate_for_literal, VariableData},
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
            self.builder
                .build_store(
                    var,
                    match expression {
                        ASTUnit::Expression(expr) => match expr {
                            Expression::Literal(literal) => {
                                generate_for_literal(self.context, literal)
                            }
                            Expression::Identifier(ident) => {
                                if let Some(&basic) = self.ssa.borrow().get(ident) {
                                    basic
                                } else {
                                    let sf = self.stack_frame.borrow();
                                    let variable_data = sf.get(ident).unwrap();
                                    self.builder
                                        .build_load(variable_data.ty(), variable_data.ptr(), ident)
                                        .unwrap()
                                }
                            }
                            other => panic!("unimplemented: {other:?}"),
                        },
                        other => panic!("exprected expression, got: {other:?}"),
                    },
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
