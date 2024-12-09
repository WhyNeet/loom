use std::rc::Rc;

use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicValue, FunctionValue},
};
use parser::ast::{statement::Statement, unit::ASTUnit};

use super::{expression::LLVMExpressionGenerator, function::LLVMFunctionGenerator};

pub struct LLVMStatementGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    expression_gen: &'ctx LLVMExpressionGenerator<'ctx>,
    function: FunctionValue<'ctx>,
    fn_gen: &'ctx LLVMFunctionGenerator<'ctx>,
}

impl<'ctx> LLVMStatementGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        expression_gen: &'ctx LLVMExpressionGenerator<'ctx>,
        fn_gen: &'ctx LLVMFunctionGenerator<'ctx>,
        function: FunctionValue<'ctx>,
    ) -> Self {
        Self {
            builder,
            context,
            expression_gen,
            function,
            fn_gen,
        }
    }

    pub fn generate_from_ast(&self, stmt: &'ctx Statement, next: Vec<Rc<ASTUnit>>) {
        match stmt {
            Statement::Return(ret) => {
                let ret_value = match &ret[0].as_ref() {
                    ASTUnit::Expression(expr) => {
                        self.expression_gen.generate_from_ast("ret_res", &expr)
                    }
                    _ => todo!(),
                };

                self.builder
                    .build_return(ret_value.as_ref().map(|val| val as &dyn BasicValue))
                    .unwrap();
            }
            Statement::ControlFlow {
                condition,
                execute,
                alternative,
            } => {
                let cmp = self
                    .expression_gen
                    .generate_from_ast(
                        "cf_cr",
                        match &**condition {
                            ASTUnit::Expression(expr) => expr,
                            _ => unreachable!(),
                        },
                    )
                    .unwrap()
                    .into_int_value();

                let current_block = self.builder.get_insert_block();

                // generate a block for what's after if-else clause
                let continue_block = self.context.append_basic_block(self.function, "bb");
                self.builder.position_at_end(continue_block);

                let next = ASTUnit::Block(next);
                self.fn_gen.generate_from_ast(Rc::new(next));

                // // generate a then-block
                // let then_block = self.context.append_basic_block(self.function, "bb");
                // self.builder.position_at_end(then_block);

                // self.fn_gen.generate_from_ast(execute);
                // self.builder.build_unconditional_branch(continue_block);

                if let Some(_alternative) = alternative {
                    // generate an alternative block
                    // let else_block = self.context.append_basic_block(function, name);
                }

                self.builder.position_at_end(current_block.unwrap());
                self.builder
                    .build_conditional_branch(cmp, continue_block, continue_block)
                    .unwrap();
            }
            other => panic!("{other:?} is not implemented yet"),
        }
    }
}
