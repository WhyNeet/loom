use std::rc::Rc;

use inkwell::{
    basic_block::BasicBlock,
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
            } => self.generate_control_flow(condition, execute, alternative, next, None),
            other => panic!("{other:?} is not implemented yet"),
        }
    }

    fn generate_control_flow(
        &self,
        condition: &'ctx Rc<ASTUnit>,
        execute: &Rc<ASTUnit>,
        alternative: &'ctx Option<Rc<ASTUnit>>,
        next: Vec<Rc<ASTUnit>>,
        continue_block: Option<BasicBlock<'ctx>>,
    ) {
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

        // save the control flow entry block
        // later append the "br" instruction with
        // both branches provided
        let cf_entry_block = self.builder.get_insert_block().unwrap();

        // whatever is after the control flow statement
        let continue_block = if let Some(block) = continue_block {
            block
        } else {
            let continue_block = self.context.append_basic_block(self.function, "continue");
            self.builder.position_at_end(continue_block);

            let next_ast = ASTUnit::Block(next.iter().map(Rc::clone).collect());
            self.fn_gen.generate_from_ast(Rc::new(next_ast));

            continue_block
        };

        // the block to execute if condition is true
        let execute_block = self.context.append_basic_block(self.function, "execute");
        self.builder.position_at_end(execute_block);

        let execute_ast = Rc::clone(execute);
        self.fn_gen.generate_from_ast(execute_ast);

        self.builder
            .build_unconditional_branch(continue_block)
            .unwrap();

        // the block to execute if condition is false
        let alternative_block = if let Some(alternative) = alternative {
            let alternative_block = self
                .context
                .append_basic_block(self.function, "alternative");
            self.builder.position_at_end(alternative_block);

            match alternative.as_ref() {
                ASTUnit::Block(block) => match block[0].as_ref() {
                    ASTUnit::Statement(Statement::ControlFlow {
                        condition,
                        execute,
                        alternative,
                    }) => {
                        self.generate_control_flow(
                            condition,
                            execute,
                            alternative,
                            next,
                            Some(continue_block),
                        );
                    }
                    _ => {
                        self.fn_gen.generate_from_ast(Rc::clone(alternative));
                        self.builder
                            .build_unconditional_branch(continue_block)
                            .unwrap();
                    }
                },
                _ => unreachable!(),
            };

            Some(alternative_block)
        } else {
            None
        };

        self.builder.position_at_end(cf_entry_block);
        self.builder
            .build_conditional_branch(
                cmp,
                execute_block,
                alternative_block.unwrap_or(continue_block),
            )
            .unwrap();
    }
}
