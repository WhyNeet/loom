use inkwell::{
    builder::Builder,
    context::Context,
    types::{BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, FunctionValue},
    AddressSpace,
};
use parser::ast::{
    declaration::{Declaration, VariableDeclarationKeyword},
    expression::Expression,
    unit::ASTUnit,
};
use std::collections::HashMap;

use super::common::{generate_for_literal, VariableData};

pub struct LLVMFunctionGenerator<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    stack_frame: HashMap<String, VariableData<'ctx>>,
    ssa: HashMap<String, BasicValueEnum<'ctx>>,
}

impl<'ctx> LLVMFunctionGenerator<'ctx> {
    pub fn new(context: &'ctx Context, function: FunctionValue<'ctx>) -> Self {
        let entry = context.append_basic_block(function, "entry");

        let builder = context.create_builder();

        builder.position_at_end(entry);

        Self {
            context,
            builder,
            stack_frame: HashMap::new(),
            ssa: HashMap::new(),
        }
    }

    pub fn generate_from_ast(&mut self, ast: &'ctx ASTUnit) {
        self.interal_generate_from_ast(ast);
    }

    fn interal_generate_from_ast(&mut self, ast: &'ctx ASTUnit) {
        match ast {
            ASTUnit::Block(block) => {
                for unit in block {
                    self.interal_generate_from_ast(unit);
                }
            }
            ASTUnit::Declaration(decl) => match decl {
                Declaration::VariableDeclaration {
                    keyword,
                    identifier,
                    expression,
                } => {
                    let var_type = self.context.i32_type();

                    if *keyword == VariableDeclarationKeyword::Let {
                        // mutable variable declaration

                        let var = self
                            .builder
                            .build_alloca(var_type, identifier.as_str())
                            .unwrap();
                        self.stack_frame.insert(
                            identifier.to_string(),
                            VariableData::new(var, var_type.into()),
                        );
                        self.builder
                            .build_store(
                                var,
                                match &(**expression) {
                                    ASTUnit::Expression(expr) => match expr {
                                        Expression::Literal(literal) => {
                                            generate_for_literal(self.context, literal)
                                        }
                                        Expression::Identifier(ident) => {
                                            if let Some(&basic) = self.ssa.get(ident) {
                                                basic
                                            } else {
                                                let variable_data =
                                                    self.stack_frame.get(ident).unwrap();
                                                self.builder
                                                    .build_load(
                                                        variable_data.ty(),
                                                        variable_data.ptr(),
                                                        ident,
                                                    )
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
                            match &(**expression) {
                                ASTUnit::Expression(expr) => match expr {
                                    Expression::Literal(literal) => literal,
                                    other => panic!("unimplemented: {other:?}"),
                                },
                                other => panic!("exprected expression, got: {other:?}"),
                            },
                        );

                        self.ssa.insert(identifier.to_string(), value);
                    }
                }
                Declaration::FunctionDeclaration {
                    identifier,
                    parameters,
                    return_type,
                    expression,
                } => panic!("dont declare functions within functions pls"),
            },
            ASTUnit::Expression(expr) => {}
            ASTUnit::Statement(stmt) => {}
        }
    }
}
