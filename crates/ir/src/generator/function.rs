use inkwell::{
    builder::Builder,
    context::Context,
    values::{BasicValueEnum, FunctionValue},
};
use parser::ast::{declaration::Declaration, statement::Statement, unit::ASTUnit};
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

    pub fn generate_from_ast(&'ctx self, ast: Rc<ASTUnit>) {
        self.internal_generate_from_ast(ast, None);
        if self.is_void {
            self.builder.build_return(None).unwrap();
        }
    }

    pub fn internal_generate_from_ast(&'ctx self, ast: Rc<ASTUnit>, store_return: Option<&str>) {
        let root = match unsafe { (ast.as_ref() as *const ASTUnit).as_ref().unwrap() } {
            ASTUnit::Block(root) => root,
            other => panic!("expected root block, got: {other:?}"),
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
                            self,
                        );
                        var_gen.generate_for_ast(&keyword, &identifier, &expression);
                    }
                },
                ASTUnit::Expression(expr) => {
                    self.expr_gen.generate_from_ast(
                        &format!(
                            "{}",
                            if let Some(store_in) = store_return {
                                store_in
                            } else {
                                "expr_tmp"
                            }
                        ),
                        &expr,
                    );
                }
                ASTUnit::Statement(stmt) => {
                    let remaining_instruct = root
                        .iter()
                        .skip(idx + 1)
                        .map(|unit| Rc::clone(unit))
                        .collect::<Vec<Rc<ASTUnit>>>();

                    let stmt_gen = LLVMStatementGenerator::new(
                        self.context,
                        &self.builder,
                        &self.expr_gen,
                        self,
                        self.function,
                    );
                    if let Some(store_return) = store_return {
                        match stmt {
                            Statement::ImplicitReturn(ret) => match ret.as_ref() {
                                ASTUnit::Expression(expr) => {
                                    println!("store implicit return: {store_return}");
                                    let value = self.expr_gen.generate_from_ast(store_return, expr);
                                    if let Some(value) = value {
                                        self.ssa
                                            .borrow_mut()
                                            .insert(store_return.to_string(), value);
                                    }
                                }
                                ASTUnit::Block(block) => self.internal_generate_from_ast(
                                    Rc::new(ASTUnit::Block(block.clone())),
                                    Some(store_return),
                                ),
                                _ => unreachable!(),
                            },
                            stmt => stmt_gen.generate_from_ast(stmt, remaining_instruct),
                        }
                    } else {
                        stmt_gen.generate_from_ast(stmt, remaining_instruct);
                    }

                    break;
                }
                ASTUnit::Block(block) => {
                    self.internal_generate_from_ast(
                        Rc::new(ASTUnit::Block(block.iter().map(Rc::clone).collect())),
                        store_return,
                    );
                }
            }
        }
    }
}
