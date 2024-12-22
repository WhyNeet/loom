use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicValueEnum, FunctionValue},
};
use preprocessor::last::{declaration::Declaration, statement::Statement, unit::LASTUnit};
use std::{cell::RefCell, collections::HashMap, ptr, rc::Rc};

use super::{
    common::VariableData, expression::LLVMExpressionGenerator, module::FunctionStack,
    statement::LLVMStatementGenerator, variable::LLVMVariableGenerator,
};

pub type StackFrame<'ctx> = HashMap<String, VariableData<'ctx>>;
pub type SSA<'ctx> = HashMap<String, BasicValueEnum<'ctx>>;

pub struct LLVMFunctionGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    stack_frame: Rc<RefCell<StackFrame<'ctx>>>,
    ssa: Rc<RefCell<SSA<'ctx>>>,
    function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
    function: FunctionValue<'ctx>,
    is_void: bool,
    expr_gen: LLVMExpressionGenerator<'ctx>,
}

impl<'ctx> LLVMFunctionGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        function: FunctionValue<'ctx>,
        param_names: &[&str],
        function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
    ) -> Self {
        let entry = context.append_basic_block(function, "entry");

        let builder = context.create_builder();

        builder.position_at_end(entry);

        let is_void = function.get_type().get_return_type().is_none();

        let mut ssa = SSA::new();

        for (idx, param) in function.get_param_iter().enumerate() {
            ssa.insert(param_names[idx].to_string(), param);
        }

        let stack_frame = Rc::new(RefCell::new(HashMap::new()));
        let ssa = Rc::new(RefCell::new(ssa));

        let expr_gen = LLVMExpressionGenerator::new(
            context,
            unsafe { (&builder as *const Builder<'ctx>).as_ref().unwrap() },
            Rc::clone(&stack_frame),
            Rc::clone(&ssa),
            Rc::clone(&function_stack),
        );

        Self {
            context,
            builder,
            stack_frame,
            ssa,
            function_stack,
            function,
            is_void,
            expr_gen,
        }
    }

    pub fn generate_from_ast(&'ctx self, ast: Vec<Rc<LASTUnit>>) {
        self.internal_generate_from_ast(ast);
        if self.is_void {
            self.builder.build_return(None).unwrap();
        }
    }

    pub fn internal_generate_from_ast(&'ctx self, ast: Vec<Rc<LASTUnit>>) {
        for idx in 0..ast.len() {
            let unit = unsafe { (&ast[idx] as *const Rc<LASTUnit>).as_ref().unwrap() };

            match unit.as_ref() {
                LASTUnit::Declaration(decl) => match decl {
                    Declaration::FunctionDeclaration { .. } => {
                        panic!("dont declare functions within functions pls")
                    }
                    Declaration::VariableDeclaration {
                        allocation,
                        expression,
                        identifier,
                    } => {
                        let var_gen = LLVMVariableGenerator::new(
                            self.context,
                            &self.builder,
                            Rc::clone(&self.stack_frame),
                            Rc::clone(&self.ssa),
                            Rc::clone(&self.function_stack),
                            self,
                        );
                        var_gen.generate_for_ast(allocation, identifier, expression.as_ref());
                    }
                },
                LASTUnit::Expression(expr) => {
                    self.expr_gen.generate_from_ast(&expr, None);
                }
                LASTUnit::Statement(stmt) => {
                    let remaining_instruct = ast
                        .iter()
                        .skip(idx + 1)
                        .map(|unit| Rc::clone(unit))
                        .collect::<Vec<Rc<LASTUnit>>>();

                    let stmt_gen = LLVMStatementGenerator::new(
                        self.context,
                        &self.builder,
                        &self.expr_gen,
                        self,
                        self.function,
                    );
                    stmt_gen.generate_from_ast(stmt, remaining_instruct);

                    break;
                }
            }
        }
    }
}
