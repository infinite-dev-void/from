use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{custom_types, metas_holder::MetasHolder, types};

impl super::ElemParsing {
    pub fn try_build_bool<H: MetasHolder>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream> {
        match null {
            true => Self::try_build_bool_null(vec_var, idx_var, metas_holder, dflt_lang),

            false => Self::try_build_bool_not_null(vec_var, idx_var, metas_holder, dflt_lang),
        }
    }

    #[inline]
    pub fn try_build_bool_null<H: MetasHolder>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let type_mismatch_err;
        let processing;

        match metas_holder {
            Some(metas_holder) => {
                processing =
                    types::bool::Processing::try_build(&metas_holder, &idx_var, dflt_lang)?;

                type_mismatch_err = custom_types::TypeMismatchErr::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    "boolean",
                )?;
            }

            None => {
                processing = types::bool::Processing::new();

                type_mismatch_err = custom_types::TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: boolean, found: {}", found)},
                );
            }
        };

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#vec_var.push(::from::Null::<bool>::Null);},
        );

        Ok(Self::build_bool(
            processing,
            quote! {#vec_var.push(::from::Null::<bool>::Some(val));},
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    pub fn try_build_bool_not_null<H: MetasHolder>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let type_mismatch_err;
        let processing;
        let null_handling;

        match metas_holder {
            Some(metas_holder) => {
                processing =
                    types::bool::Processing::try_build(&metas_holder, &idx_var, dflt_lang)?;

                null_handling = custom_types::NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from("expected: boolean, found: null")}
                    },
                )?;

                type_mismatch_err = custom_types::TypeMismatchErr::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    "boolean",
                )?;
            }

            None => {
                processing = types::bool::Processing::new();

                null_handling = custom_types::NullHandling::new_dflt(
                    &idx_var,
                    quote! {String::from("expected: boolean, found: null")},
                );

                type_mismatch_err = custom_types::TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: boolean, found: {}", found)},
                );
            }
        };

        Ok(Self::build_bool(
            processing,
            quote! {#vec_var.push(val);},
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn build_bool(
        processing: types::bool::Processing,
        elem_push: TokenStream2,
        null_handling: custom_types::NullHandling,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Self {
        Self {
            none: bool_field_match_parse_temp(
                processing.valdg,
                &elem_push,
                null_handling.none,
                type_mismatch_err.none,
            ),

            lang: bool_field_match_parse_temp(
                processing.valdg_lang,
                &elem_push,
                null_handling.lang,
                type_mismatch_err.lang,
            ),

            stack_errs: bool_field_match_parse_temp(
                processing.valdg_stack_errs,
                &elem_push,
                null_handling.stack_errs,
                type_mismatch_err.stack_errs,
            ),

            stack_errs_lang: bool_field_match_parse_temp(
                processing.valdg_stack_errs_lang,
                &elem_push,
                null_handling.stack_errs_lang,
                type_mismatch_err.stack_errs_lang,
            ),
        }
    }
}

#[inline]
fn bool_field_match_parse_temp(
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
) -> TokenStream2 {
    quote! {
        match ::from::json::bool::parse(json, idx) {
            ::from::json::bool::ParseResult::Ok(val) => {
                #valdg
                #elem_push
            }

            ::from::json::bool::ParseResult::Null => {
                #null_handling
            }


            ::from::json::bool::ParseResult::TypeMismatch(found) => {
                #type_mismatch_err
            }

            ::from::json::bool::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        };
    }
}
