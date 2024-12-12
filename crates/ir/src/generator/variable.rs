use std::{cell::RefCell, rc::Rc};

use inkwell::{builder::Builder, context::Context};
use parser::ast::{
    declaration::{Declaration, VariableDeclarationKeyword},
    expression::Expression,
    unit::ASTUnit,
};

use super::{
    common::VariableData,
    expression::LLVMExpressionGenerator,
    function::{LLVMFunctionGenerator, StackFrame, SSA},
    module::FunctionStack,
};

pub struct LLVMVariableGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
    function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
    fn_gen: &'ctx LLVMFunctionGenerator<'ctx>,
}

impl<'ctx> LLVMVariableGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
        ssa: Rc<RefCell<SSA<'ctx>>>,
        function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
        fn_gen: &'ctx LLVMFunctionGenerator<'ctx>,
    ) -> Self {
        Self {
            builder,
            context,
            stack_frame,
            ssa,
            function_stack,
            fn_gen,
        }
    }

    pub fn generate_for_ast(
        &self,
        keyword: &VariableDeclarationKeyword,
        identifier: &String,
        expression: &'ctx ASTUnit,
    ) {
        let var_type = self.context.i32_type();

        let value = match expression {
            ASTUnit::Expression(expr) => LLVMExpressionGenerator::new(
                self.context,
                self.builder,
                Rc::clone(&self.stack_frame),
                Rc::clone(&self.ssa),
                Rc::clone(&self.function_stack),
            )
            .generate_from_ast(&format!("{identifier}_tmp"), expr),
            ASTUnit::Block(block) => {
                let store_in = format!("{}_block_res", identifier.as_str());

                self.fn_gen.internal_generate_from_ast(
                    Rc::new(ASTUnit::Block(block.clone())),
                    Some(&store_in),
                );

                self.ssa.borrow().get(&store_in).map(|val| *val)
            }
            other => panic!("expected expression, got: {other:?}"),
        }
        .unwrap();

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
            self.builder.build_store(var, value).unwrap();
        } else {
            // immutable variable declaration

            self.ssa.borrow_mut().insert(identifier.to_string(), value);
        }
    }
}
