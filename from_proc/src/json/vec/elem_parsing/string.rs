use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{metas_holder::MetasHolder, types::string::Processing, NullHandling, TypeMismatchErr};

impl super::ElemParsing {
    pub fn try_build_string<H>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        match null {
            true => Self::try_build_string_null::<H>(vec_var, idx_var, metas_holder, dflt_lang),

            false => {
                Self::try_build_string_not_null::<H>(vec_var, idx_var, metas_holder, dflt_lang)
            }
        }
    }

    #[inline]
    fn try_build_string_null<H>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let type_mismatch_err;
        let processing;

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(&metas_holder, &idx_var, dflt_lang)?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, "string")?;
            }

            None => {
                processing = Processing::new();

                type_mismatch_err = TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: string, found: {}", found)},
                );
            }
        };

        let null_handling =
            NullHandling::from_one(quote! {#vec_var.push(::from::Null::<String>::Null);});

        Ok(Self::build_string(
            processing,
            quote! {
                #vec_var.push(::from::Null::<String>::Some(val));
            },
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn try_build_string_not_null<H>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let type_mismatch_err;
        let processing;
        let null_handling;

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(&metas_holder, &idx_var, dflt_lang)?;

                null_handling = NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from("expected: string, found: null")}
                    },
                )?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, "string")?;
            }

            None => {
                processing = Processing::new();

                null_handling = NullHandling::new_dflt(
                    &idx_var,
                    quote! {String::from("expected: string, found: null")},
                );

                type_mismatch_err = TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: string, found: {}", found)},
                );
            }
        };

        Ok(Self::build_string(
            processing,
            quote! {#vec_var.push(val);},
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn build_string(
        processing: Processing,
        elem_push: TokenStream2,
        null_handling: NullHandling,
        type_mismatch_err: TypeMismatchErr,
    ) -> Self {
        Self {
            none: string_field_match_parse_temp(
                &processing.mutable,
                &processing.sanitizing,
                processing.valdg,
                &elem_push,
                null_handling.none,
                type_mismatch_err.none,
            ),

            lang: string_field_match_parse_temp(
                &processing.mutable,
                &processing.sanitizing,
                processing.valdg_lang,
                &elem_push,
                null_handling.lang,
                type_mismatch_err.lang,
            ),

            stack_errs: string_field_match_parse_temp(
                &processing.mutable,
                &processing.sanitizing,
                processing.valdg_stack_errs,
                &elem_push,
                null_handling.stack_errs,
                type_mismatch_err.stack_errs,
            ),

            stack_errs_lang: string_field_match_parse_temp(
                &processing.mutable,
                &processing.sanitizing,
                processing.valdg_stack_errs_lang,
                &elem_push,
                null_handling.stack_errs_lang,
                type_mismatch_err.stack_errs_lang,
            ),
        }
    }
}

#[inline]
pub fn string_field_match_parse_temp(
    mutable: &TokenStream2,
    sanitizing: &TokenStream2,
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
) -> TokenStream2 {
    quote! {
        match ::from::json::string::parse(json, idx) {
            ::from::json::string::ParseResult::Ok(#mutable val) => {
                #sanitizing
                #valdg
                #elem_push
            }

            ::from::json::string::ParseResult::Null => {
                #null_handling
            }


            ::from::json::string::ParseResult::TypeMismatch(found) => {
                #type_mismatch_err
            }

            ::from::json::string::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        }
    }
}
