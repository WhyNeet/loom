use common::types::Type;
use inkwell::{
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType},
};
use parser::ast::{declaration::Declaration, unit::ASTUnit};

use super::{common::type_for, function::LLVMFunctionGenerator};

pub struct LLVMModuleGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> LLVMModuleGenerator<'ctx> {
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);

        Self { context, module }
    }

    pub fn generate_from_ast(&mut self, ast: &'ctx ASTUnit) {
        match ast {
            ASTUnit::Block(root) => {
                for unit in root {
                    self.__generate_from_ast(unit);
                }
            }
            _ => panic!("expected root block"),
        }
    }

    fn __generate_from_ast(&mut self, unit: &'ctx ASTUnit) {
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

                    LLVMFunctionGenerator::new(self.context, function)
                        .generate_from_ast(expression);
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
