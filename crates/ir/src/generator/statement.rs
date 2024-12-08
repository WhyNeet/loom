use inkwell::{builder::Builder, context::Context, values::BasicValue};
use parser::ast::{statement::Statement, unit::ASTUnit};

use super::expression::LLVMExpressionGenerator;

pub struct LLVMStatementGenerator<'ctx> {
    builder: &'ctx Builder<'ctx>,
    context: &'ctx Context,
    expression_gen: &'ctx LLVMExpressionGenerator<'ctx>,
}

impl<'ctx> LLVMStatementGenerator<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        expression_gen: &'ctx LLVMExpressionGenerator<'ctx>,
    ) -> Self {
        Self {
            builder,
            context,
            expression_gen,
        }
    }

    pub fn generate_from_ast(&self, stmt: &'ctx Statement) {
        match stmt {
            Statement::Return(ret) => {
                let ret_value = match &ret[0] {
                    ASTUnit::Expression(expr) => {
                        self.expression_gen.generate_from_ast("ret_res", &expr)
                    }
                    _ => todo!(),
                };

                self.builder
                    .build_return(ret_value.as_ref().map(|val| val as &dyn BasicValue))
                    .unwrap();
            }
            other => panic!("{other:?} is not implemented yet"),
        }
    }
}
