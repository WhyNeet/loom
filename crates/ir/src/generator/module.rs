use std::{cell::RefCell, collections::HashMap, rc::Rc};

use common::types::Type;
use inkwell::{
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType},
    values::FunctionValue,
};
use parser::ast::{declaration::Declaration, unit::ASTUnit};

use super::{common::type_for, function::LLVMFunctionGenerator};

pub type FunctionStack<'ctx> = HashMap<String, FunctionValue<'ctx>>;

pub struct LLVMModuleGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    function_stack: Rc<RefCell<FunctionStack<'ctx>>>,
}

impl<'ctx> LLVMModuleGenerator<'ctx> {
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);

        Self {
            context,
            module,
            function_stack: Rc::new(RefCell::new(FunctionStack::new())),
        }
    }

    pub fn generate_from_ast(&self, ast: &'ctx ASTUnit) {
        match ast {
            ASTUnit::Block(root) => {
                for unit in root {
                    self.__generate_from_ast(unit);
                }
            }
            _ => panic!("expected root block"),
        }
    }

    fn __generate_from_ast(&self, unit: &'ctx ASTUnit) {
        match unit {
            ASTUnit::Declaration(decl) => match decl {
                Declaration::FunctionDeclaration {
                    identifier,
                    parameters,
                    return_type,
                    expression,
                } => {
                    let params = &parameters
                        .into_iter()
                        .map(|(_, ty)| type_for(self.context, ty).into())
                        .collect::<Vec<BasicMetadataTypeEnum<'ctx>>>();

                    let function = self.module.add_function(
                        identifier.as_str(),
                        match return_type {
                            Type::Void => None,
                            other => Some(other),
                        }
                        .map(|ty| type_for(self.context, ty).fn_type(&params, false))
                        .unwrap_or(self.context.void_type().fn_type(&params, false)),
                        None,
                    );

                    let fn_gen = LLVMFunctionGenerator::new(
                        self.context,
                        function,
                        Rc::clone(&self.function_stack),
                    );
                    unsafe { (&fn_gen as *const LLVMFunctionGenerator).as_ref().unwrap() }
                        .generate_from_ast(expression);

                    self.function_stack
                        .borrow_mut()
                        .insert(identifier.clone(), function);
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
