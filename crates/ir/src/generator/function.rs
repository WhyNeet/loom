use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicValueEnum, FunctionValue},
};
use parser::ast::{declaration::Declaration, unit::ASTUnit};
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

        Self {
            context,
            builder,
            stack_frame: Rc::new(RefCell::new(HashMap::new())),
            ssa: Rc::new(RefCell::new(ssa)),
            function_stack,
            function,
            is_void,
        }
    }

    pub fn generate_from_ast(&'ctx self, ast: Rc<ASTUnit>) {
        self.internal_generate_from_ast(ast);
        if self.is_void {
            self.builder.build_return(None).unwrap();
        }
    }

    fn internal_generate_from_ast(&'ctx self, ast: Rc<ASTUnit>) {
        let root = match unsafe { (ast.as_ref() as *const ASTUnit).as_ref().unwrap() } {
            ASTUnit::Block(root) => root,
            _ => panic!("expected root block"),
        };

        for idx in 0..root.len() {
            let unit = &root[idx].as_ref();

            match unit {
                ASTUnit::Declaration(decl) => match decl {
                    Declaration::FunctionDeclaration { .. } => {
                        panic!("dont declare functions within functions pls")
                    }
                    Declaration::VariableDeclaration {
                        keyword,
                        identifier,
                        expression,
                    } => {
                        let var_gen = LLVMVariableGenerator::new(
                            self.context,
                            &self.builder,
                            Rc::clone(&self.stack_frame),
                            Rc::clone(&self.ssa),
                            Rc::clone(&self.function_stack),
                        );
                        var_gen.generate_for_ast(&keyword, &identifier, &expression);
                    }
                },
                ASTUnit::Expression(expr) => {
                    LLVMExpressionGenerator::new(
                        self.context,
                        &self.builder,
                        Rc::clone(&self.stack_frame),
                        Rc::clone(&self.ssa),
                        Rc::clone(&self.function_stack),
                    )
                    .generate_from_ast(&format!("expr_tmp"), &expr);
                }
                ASTUnit::Statement(stmt) => {
                    let expr = LLVMExpressionGenerator::new(
                        self.context,
                        &self.builder,
                        Rc::clone(&self.stack_frame),
                        Rc::clone(&self.ssa),
                        Rc::clone(&self.function_stack),
                    );

                    let remaining_instruct = root
                        .iter()
                        .skip(idx + 1)
                        .map(|unit| Rc::clone(unit))
                        .collect::<Vec<Rc<ASTUnit>>>();

                    let stmt_gen = LLVMStatementGenerator::new(
                        self.context,
                        &self.builder,
                        unsafe { (&expr as *const LLVMExpressionGenerator).as_ref().unwrap() },
                        self,
                        self.function,
                    );
                    stmt_gen.generate_from_ast(&stmt, remaining_instruct);

                    break;
                }
                _ => unimplemented!("code blocks are not implemented yet"),
            }
        }
    }
}
