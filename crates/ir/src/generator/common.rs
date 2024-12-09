use common::types::Type;
use inkwell::{
    context::Context,
    types::BasicTypeEnum,
    values::{BasicValueEnum, PointerValue},
    AddressSpace,
};
use parser::ast::literal::Literal;

pub struct VariableData<'ctx> {
    ptr: PointerValue<'ctx>,
    ty: BasicTypeEnum<'ctx>,
}

impl<'ctx> VariableData<'ctx> {
    pub fn new(ptr: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>) -> Self {
        Self { ptr, ty }
    }

    pub fn ptr(&self) -> PointerValue<'ctx> {
        self.ptr
    }

    pub fn ty(&self) -> BasicTypeEnum<'ctx> {
        self.ty
    }
}

pub fn type_for<'ctx>(context: &'ctx Context, ty: &Type) -> BasicTypeEnum<'ctx> {
    match ty {
        Type::Bool => context.bool_type().into(),
        Type::Char => context.i32_type().into(),
        Type::Float32 => context.f32_type().into(),
        Type::Float64 => context.f64_type().into(),
        Type::Int16 => context.i16_type().into(),
        Type::Int32 => context.i32_type().into(),
        Type::Int64 => context.i64_type().into(),
        Type::Int8 => context.i8_type().into(),
        Type::String => context.ptr_type(AddressSpace::default()).into(),
        Type::UInt16 => context.i16_type().into(),
        Type::UInt32 => context.i32_type().into(),
        Type::UInt64 => context.i64_type().into(),
        Type::UInt8 => context.i8_type().into(),
        Type::Void => unreachable!(),
    }
}

pub fn generate_for_literal<'ctx>(
    context: &'ctx Context,
    literal: &'ctx Literal,
) -> BasicValueEnum<'ctx> {
    match literal {
        Literal::Bool(bool) => context.bool_type().const_int(*bool as u64, false).into(),
        Literal::Char(char) => context.i32_type().const_int(*char as u64, false).into(),
        Literal::Int8(i8) => context.i8_type().const_int((*i8 as u8) as u64, true).into(),
        Literal::UInt8(u8) => context
            .i8_type()
            .const_int((*u8 as u8) as u64, false)
            .into(),
        Literal::Int16(i16) => context
            .i16_type()
            .const_int((*i16 as u16) as u64, true)
            .into(),
        Literal::UInt16(u16) => context
            .i16_type()
            .const_int((*u16 as u16) as u64, false)
            .into(),
        Literal::Int32(i32) => context
            .i32_type()
            .const_int((*i32 as u32) as u64, true)
            .into(),
        Literal::UInt32(u32) => context
            .i32_type()
            .const_int((*u32 as u32) as u64, false)
            .into(),
        Literal::Int64(i64) => context.i64_type().const_int(*i64 as u64, true).into(),
        Literal::UInt64(u64) => context.i64_type().const_int(*u64 as u64, false).into(),
        Literal::Float32(f32) => context.f32_type().const_float(*f32 as f64).into(),
        Literal::Float64(f64) => context.f64_type().const_float(*f64).into(),
        Literal::String(string) => context.const_string(string.as_bytes(), true).into(),
    }
}
