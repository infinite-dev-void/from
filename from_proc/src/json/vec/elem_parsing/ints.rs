use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{
    metas_holder::MetasHolder,
    types::ints::{Ints, Processing, TooLargeErr, TooSmallErr},
    NullHandling, TypeMismatchErr,
};

impl super::ElemParsing {
    pub fn try_build_int<H, I>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        I: Ints,
        H: MetasHolder,
    {
        match null {
            true => Self::try_build_int_null::<H, I>(vec_var, idx_var, metas_holder, dflt_lang),

            false => {
                Self::try_build_int_not_null::<H, I>(vec_var, idx_var, metas_holder, dflt_lang)
            }
        }
    }

    #[inline]
    fn try_build_int_null<H, I>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        I: Ints,
        H: MetasHolder,
    {
        let type_mismatch_err;
        let processing;
        let too_large_err;
        let too_small_err;

        let expc = I::expc();

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H, I>(&metas_holder, &idx_var, dflt_lang)?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, expc)?;

                too_large_err = TooLargeErr::try_build(&metas_holder, &idx_var, dflt_lang)?;

                too_small_err = TooSmallErr::try_build(&metas_holder, &idx_var, dflt_lang)?;
            }

            None => {
                processing = Processing::new();

                let msg = format!("expected: {}, found: {{}}", expc);
                type_mismatch_err =
                    TypeMismatchErr::new_dflt(&idx_var, quote! {format!(#msg, found)});

                too_large_err = TooLargeErr::new_dflt(&idx_var);

                too_small_err = TooSmallErr::new_dflt(&idx_var);
            }
        };

        let ty = I::ty();

        let null_handling =
            NullHandling::from_one(quote! {#vec_var.push(::from::Null::<#ty>::Null);});

        let elem_push = quote! {#vec_var.push(::from::Null::<#ty>::Some(val));};

        Ok(Self::build_int(
            ty,
            processing,
            elem_push,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        ))
    }

    #[inline]
    fn try_build_int_not_null<H, I>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        I: Ints,
        H: MetasHolder,
    {
        let type_mismatch_err;
        let processing;
        let null_handling;
        let too_large_err;
        let too_small_err;

        let expc = I::expc();

        let null_msg = format!("expected: {}, found: null", expc);

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H, I>(&metas_holder, &idx_var, dflt_lang)?;

                null_handling = NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from(#null_msg)}
                    },
                )?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, &expc)?;

                too_large_err = TooLargeErr::try_build(&metas_holder, &idx_var, dflt_lang)?;

                too_small_err = TooSmallErr::try_build(&metas_holder, &idx_var, dflt_lang)?;
            }

            None => {
                processing = Processing::new();

                null_handling = NullHandling::new_dflt(&idx_var, quote! {String::from(#null_msg)});

                let msg = format!("expected: {}, found: {{}}", expc);

                type_mismatch_err =
                    TypeMismatchErr::new_dflt(&idx_var, quote! {format!(#msg, found)});

                too_large_err = TooLargeErr::new_dflt(&idx_var);

                too_small_err = TooSmallErr::new_dflt(&idx_var);
            }
        };

        Ok(Self::build_int(
            I::ty(),
            processing,
            quote! {#vec_var.push(val);},
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        ))
    }

    #[inline]
    fn build_int(
        parser_module: TokenStream2,
        processing: Processing,
        elem_push: TokenStream2,
        null_handling: NullHandling,
        type_mismatch_err: TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
    ) -> Self {
        Self {
            none: int_field_match_parse_temp(
                &parser_module,
                processing.valdg,
                &elem_push,
                null_handling.none,
                type_mismatch_err.none,
                too_large_err.none,
                too_small_err.none,
            ),

            lang: int_field_match_parse_temp(
                &parser_module,
                processing.valdg_lang,
                &elem_push,
                null_handling.lang,
                type_mismatch_err.lang,
                too_large_err.lang,
                too_small_err.lang,
            ),

            stack_errs: int_field_match_parse_temp(
                &parser_module,
                processing.valdg_stack_errs,
                &elem_push,
                null_handling.stack_errs,
                type_mismatch_err.stack_errs,
                too_large_err.stack_errs,
                too_small_err.stack_errs,
            ),

            stack_errs_lang: int_field_match_parse_temp(
                &parser_module,
                processing.valdg_stack_errs_lang,
                &elem_push,
                null_handling.stack_errs_lang,
                type_mismatch_err.stack_errs_lang,
                too_large_err.stack_errs_lang,
                too_small_err.stack_errs_lang,
            ),
        }
    }
}

#[inline]
fn int_field_match_parse_temp(
    parser_module: &TokenStream2,
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
    too_large_err: TokenStream2,
    too_small_err: TokenStream2,
) -> TokenStream2 {
    quote! {
        match ::from::json::#parser_module::parse(json, idx) {
            ::from::json::#parser_module::ParseResult::Ok(val) => {
                #valdg
                #elem_push
            }

            ::from::json::#parser_module::ParseResult::Null => {
                #null_handling
            }


            ::from::json::#parser_module::ParseResult::TypeMismatch(found) => {
                #type_mismatch_err
            }

            ::from::json::#parser_module::ParseResult::TooLargeToFitInto(typ) => {
                #too_large_err
            }

            ::from::json::#parser_module::ParseResult::TooSmallToFitInto(typ) => {
                #too_small_err
            }

            ::from::json::#parser_module::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        };
    }
}
