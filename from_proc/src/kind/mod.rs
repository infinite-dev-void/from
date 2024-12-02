use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;

use quote::ToTokens;
use syn::{Ident, Type as SynType};

mod utils;
use utils::*;

mod stringify;

mod from_genr_sg;
mod from_sg;
mod from_sgs;

use crate::utils::compile_err;

pub struct Kind {
    pub option: bool,
    pub null: bool,
    pub ty: Type,
}

impl Kind {
    pub fn from_ty(ty: &SynType) -> Result<Self, TokenStream> {
        let path = &type_path_or_err(ty)?.path;

        let sgs = &path.segments;

        match sgs.len() {
            0 => Err(compile_err(
                &path,
                "path must contains one segment at least",
            )),

            1 => Self::from_sg(&path),

            _ => Self::from_sgs(&path),
        }
    }
}

pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    String,
    F32,
    F64,
    Bool,
    Vec(TokenStream2, Box<Kind>),
    Custom(TokenStream2),
}

impl Type {
    /* #[inline]
    fn custom(ty: &str) -> Self {
        Self::Custom(ty.to_owned())
    } */

    #[inline]
    fn from_ident(ident: &Ident) -> Self {
        if ident.eq("i8") {
            Type::I8
        } else if ident.eq("i16") {
            Type::I16
        } else if ident.eq("i32") {
            Type::I32
        } else if ident.eq("i64") {
            Type::I64
        } else if ident.eq("i128") {
            Type::I128
        } else if ident.eq("isize") {
            Type::ISize
        } else if ident.eq("u8") {
            Type::U8
        } else if ident.eq("u16") {
            Type::U16
        } else if ident.eq("u32") {
            Type::U32
        } else if ident.eq("u64") {
            Type::U64
        } else if ident.eq("u128") {
            Type::U128
        } else if ident.eq("usize") {
            Type::USize
        } else if ident.eq("f32") {
            Type::F32
        } else if ident.eq("f64") {
            Type::F64
        } else if ident.eq("String") {
            Type::String
        } else if ident.eq("bool") {
            Type::Bool
        } else {
            Type::Custom(ident.to_token_stream())
        }
    }
}
