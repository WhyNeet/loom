use std::rc::Rc;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{BasicValue, FunctionValue},
};
use preprocessor::last::{expression::Expression, statement::Statement, unit::LASTUnit};

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

    pub fn generate_from_ast(&self, stmt: &'ctx Statement, next: Vec<Rc<LASTUnit>>) {
        match stmt {
            Statement::Return(ret) => {
                let ret_value = self.expression_gen.generate_from_ast(ret.as_ref(), None);

                self.builder
                    .build_return(ret_value.as_ref().map(|val| val as &dyn BasicValue))
                    .unwrap();
            }
            Statement::Loop {
                body,
                condition,
                header,
            } => self.generate_loop(header.clone(), Rc::clone(condition), body.clone(), next),
            Statement::ControlFlow {
                condition,
                execute,
                alternative,
            } => self.generate_control_flow(
                Rc::clone(condition),
                execute.clone(),
                alternative.clone(),
                next,
                None,
            ),
        }
    }

    fn generate_loop(
        &self,
        header: Vec<Rc<LASTUnit>>,
        condition: Rc<Expression>,
        body: Vec<Rc<LASTUnit>>,
        next: Vec<Rc<LASTUnit>>,
    ) {
        let entry_block = self.builder.get_insert_block().unwrap();

        let header_block = self
            .context
            .append_basic_block(self.function, "while.header");
        self.builder.position_at_end(header_block);

        let body_block = self.context.append_basic_block(self.function, "body");
        self.builder.position_at_end(body_block);

        self.fn_gen.internal_generate_from_ast(body);
        self.builder
            .build_unconditional_branch(header_block)
            .unwrap();

        let exit_block = self.context.append_basic_block(self.function, "exit");
        self.builder.position_at_end(exit_block);

        self.fn_gen.internal_generate_from_ast(next);

        self.builder.position_at_end(header_block);

        self.fn_gen.internal_generate_from_ast(header);

        let cmp = self
            .expression_gen
            .generate_from_ast(
                unsafe { (condition.as_ref() as *const Expression).as_ref().unwrap() },
                None,
            )
            .unwrap()
            .into_int_value();

        self.builder
            .build_conditional_branch(cmp, body_block, exit_block)
            .unwrap();

        self.builder.position_at_end(entry_block);
        self.builder
            .build_unconditional_branch(header_block)
            .unwrap();

        // position at the end of exit
        // because entry block will not execute instructions after "br"
        self.builder.position_at_end(exit_block);
    }

    fn generate_control_flow(
        &self,
        condition: Rc<Expression>,
        execute: Vec<Rc<LASTUnit>>,
        alternative: Option<Vec<Rc<LASTUnit>>>,
        next: Vec<Rc<LASTUnit>>,
        continue_block: Option<BasicBlock<'ctx>>,
    ) {
        let cmp = self
            .expression_gen
            .generate_from_ast(
                unsafe { (condition.as_ref() as *const Expression).as_ref().unwrap() },
                None,
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

            let next_ast = next.iter().map(Rc::clone).collect();
            self.fn_gen.internal_generate_from_ast(next_ast);

            continue_block
        };

        // the block to execute if condition is true
        let execute_block = self.context.append_basic_block(self.function, "execute");
        self.builder.position_at_end(execute_block);

        let execute_ast = execute;
        self.fn_gen.internal_generate_from_ast(execute_ast);

        self.builder
            .build_unconditional_branch(continue_block)
            .unwrap();

        // the block to execute if condition is false
        let alternative_block = if let Some(alternative) = alternative {
            let alternative_block = self
                .context
                .append_basic_block(self.function, "alternative");
            self.builder.position_at_end(alternative_block);

            self.fn_gen.internal_generate_from_ast(alternative.clone());
            self.builder
                .build_unconditional_branch(continue_block)
                .unwrap();

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

        // position at the end of continue
        // because entry block will not execute instructions after "br"
        self.builder.position_at_end(continue_block);
    }
}
