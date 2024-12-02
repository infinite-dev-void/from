use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{
    json::custom::{ErrHandling, MethodCall},
    metas_holder::MetasHolder,
    types::custom::Processing,
    NullHandling,
};

impl super::ElemParsing {
    pub fn try_build_custom<H: MetasHolder>(
        ty: TokenStream2,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream> {
        match null {
            true => Self::try_build_custom_null::<H>(ty, vec_var, idx_var, metas_holder),

            false => {
                Self::try_build_custom_not_null::<H>(ty, vec_var, idx_var, metas_holder, dflt_lang)
            }
        }
    }

    #[inline]
    pub fn try_build_custom_null<H: MetasHolder>(
        ty: TokenStream2,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
    ) -> Result<Self, TokenStream> {
        let processing;

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(&metas_holder, &ty)?;
            }

            None => {
                processing = Processing::new();
            }
        };

        let null_handling =
            NullHandling::from_one(quote! {#vec_var.push(::from::Null::<#ty>::Null);});

        Ok(Self::build_custom(
            null_handling,
            MethodCall::new(&ty),
            idx_var,
            processing,
            quote! {
                #vec_var.push(::from::Null::<#ty>::Some(val));
            },
            ErrHandling::new(),
        ))
    }

    #[inline]
    pub fn try_build_custom_not_null<H: MetasHolder>(
        ty: TokenStream2,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let processing;
        let null_handling;

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(&metas_holder, &ty)?;

                null_handling = NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from("expected: object, found: null")}
                    },
                )?;
            }

            None => {
                processing = Processing::new();

                null_handling = NullHandling::new_dflt(
                    &idx_var,
                    quote! {String::from("expected: object, found: null")},
                );
            }
        };

        Ok(Self::build_custom(
            null_handling,
            MethodCall::new(&ty),
            idx_var,
            processing,
            quote! {
                #vec_var.push(val);
            },
            ErrHandling::new(),
        ))
    }

    #[inline]
    fn build_custom(
        null_handling: NullHandling,
        method_call: MethodCall,
        idx_var: TokenStream2,
        processing: Processing,
        elem_push: TokenStream2,
        err_handling: ErrHandling,
    ) -> Self {
        Self {
            none: custom_field_parsing_match_temp(
                null_handling.none,
                method_call.none,
                &idx_var,
                processing.valdg,
                &elem_push,
                err_handling.none,
            ),

            lang: custom_field_parsing_match_temp(
                null_handling.lang,
                method_call.lang,
                &idx_var,
                processing.valdg_lang,
                &elem_push,
                err_handling.lang,
            ),

            stack_errs: custom_field_parsing_match_temp(
                null_handling.stack_errs,
                method_call.stack_errs,
                &idx_var,
                processing.valdg_stack_errs,
                &elem_push,
                err_handling.stack_errs,
            ),

            stack_errs_lang: custom_field_parsing_match_temp(
                null_handling.stack_errs_lang,
                method_call.stack_errs_lang,
                &idx_var,
                processing.valdg_stack_errs_lang,
                &elem_push,
                err_handling.stack_errs_lang,
            ),
        }
    }
}

#[inline]
fn custom_field_parsing_match_temp(
    null_handling: TokenStream2,
    method_call: TokenStream2,
    idx_var: &TokenStream2,
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    err_handling: TokenStream2,
) -> TokenStream2 {
    quote! {
        byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;
        if byte == b'n' {
            if let Err(e) = ::from::json::utils::skip_null(json, idx, "{") {
                return Err(From::from(e));
            };

            #null_handling
        } else {
            let mut path = path.clone();
            path.push(From::from(#idx_var));

            match #method_call {
                Ok(val) => {
                    #valdg
                    #elem_push
                    *idx += 1;
                },

                Err(e) => {
                    #err_handling
                }
            };
        };
    }
}
