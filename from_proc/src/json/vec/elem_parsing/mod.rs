use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

use crate::{
    kind::{Kind, Type},
    metas_holder::MetasHolder,
};

mod boolean;
mod custom;
mod floats;
mod ints;
mod string;
mod vec;

pub struct ElemParsing {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl ElemParsing {
    #[inline]
    pub fn try_build<H: MetasHolder>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        of: Kind,
    ) -> Result<Self, TokenStream> {
        match of.ty {
            Type::I8 => {
                Self::try_build_int::<H, i8>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::I16 => {
                Self::try_build_int::<H, i16>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::I32 => {
                Self::try_build_int::<H, i32>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::I64 => {
                Self::try_build_int::<H, i64>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::I128 => {
                Self::try_build_int::<H, i128>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::ISize => {
                Self::try_build_int::<H, isize>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::U8 => {
                Self::try_build_int::<H, u8>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::U16 => {
                Self::try_build_int::<H, u16>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::U32 => {
                Self::try_build_int::<H, u32>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::U64 => {
                Self::try_build_int::<H, u64>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::U128 => {
                Self::try_build_int::<H, u128>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::USize => {
                Self::try_build_int::<H, usize>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::String => {
                Self::try_build_string::<H>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::F32 => {
                Self::try_build_float::<H, f32>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::F64 => {
                Self::try_build_float::<H, f64>(vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }

            Type::Bool => Self::try_build_bool(vec_var, idx_var, metas_holder, dflt_lang, of.null),

            Type::Vec(ty, sub_of) => Self::try_build_vec(
                ty,
                *sub_of,
                vec_var,
                idx_var,
                metas_holder,
                dflt_lang,
                of.null,
            ),

            Type::Custom(ty) => {
                Self::try_build_custom(ty, vec_var, idx_var, metas_holder, dflt_lang, of.null)
            }
        }
    }
}
