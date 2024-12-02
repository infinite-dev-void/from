use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{
    metas_holder::MetasHolder,
    types::floats::{Floats, Processing},
    NullHandling, TypeMismatchErr,
};

impl super::ElemParsing {
    pub fn try_build_float<H, F>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
        F: Floats,
    {
        match null {
            true => Self::try_build_float_null::<H, F>(vec_var, idx_var, metas_holder, dflt_lang),

            false => {
                Self::try_build_float_not_null::<H, F>(vec_var, idx_var, metas_holder, dflt_lang)
            }
        }
    }

    #[inline]
    fn try_build_float_null<H, F>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
        F: Floats,
    {
        let type_mismatch_err;
        let processing;

        let expc = F::exp();

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H, F>(&metas_holder, &idx_var, dflt_lang)?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, expc)?;
            }

            None => {
                processing = Processing::new();

                let msg = format!("expected: {}, found: {{}}", expc);

                type_mismatch_err =
                    TypeMismatchErr::new_dflt(&idx_var, quote! {format!(#msg, found)});
            }
        };

        let ty = F::ty();

        let null_handling =
            NullHandling::from_one(quote! {#vec_var.push(::from::Null::<#ty>::Null);});

        let elem_push = quote! {#vec_var.push(::from::Null::<#ty>::Some(val));};

        Ok(Self::build_float(
            ty,
            processing,
            elem_push,
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn try_build_float_not_null<H, F>(
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
        F: Floats,
    {
        let type_mismatch_err;
        let processing;
        let null_handling;

        let expc = F::exp();
        let null_msg = format!("expected: {}, found: null", expc);

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H, F>(&metas_holder, &idx_var, dflt_lang)?;

                null_handling = NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from(#null_msg)}
                    },
                )?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, expc)?;
            }

            None => {
                processing = Processing::new();

                null_handling = NullHandling::new_dflt(&idx_var, quote! {String::from(#null_msg)});

                let msg = format!("expected: {}, found: {{}}", expc);

                type_mismatch_err =
                    TypeMismatchErr::new_dflt(&idx_var, quote! {format!(#msg, found)});
            }
        };

        Ok(Self::build_float(
            F::ty(),
            processing,
            quote! {#vec_var.push(val);},
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn build_float(
        parser_module: TokenStream2,
        processing: Processing,
        elem_push: TokenStream2,
        null_handling: NullHandling,
        type_mismatch_err: TypeMismatchErr,
    ) -> Self {
        Self {
            none: float_field_match_parse_temp(
                &parser_module,
                processing.valdg,
                &elem_push,
                null_handling.none,
                type_mismatch_err.none,
            ),

            lang: float_field_match_parse_temp(
                &parser_module,
                processing.valdg_lang,
                &elem_push,
                null_handling.lang,
                type_mismatch_err.lang,
            ),

            stack_errs: float_field_match_parse_temp(
                &parser_module,
                processing.valdg_stack_errs,
                &elem_push,
                null_handling.stack_errs,
                type_mismatch_err.stack_errs,
            ),

            stack_errs_lang: float_field_match_parse_temp(
                &parser_module,
                processing.valdg_stack_errs_lang,
                &elem_push,
                null_handling.stack_errs_lang,
                type_mismatch_err.stack_errs_lang,
            ),
        }
    }
}

#[inline]
fn float_field_match_parse_temp(
    parser_module: &TokenStream2,
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
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

            ::from::json::#parser_module::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        }
    }
}
