use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    kind::Kind, metas_holder::MetasHolder, types::vec::Processing, NullHandling, TypeMismatchErr,
};

impl super::ElemParsing {
    pub fn try_build_vec<H: MetasHolder>(
        ty: TokenStream2,
        of: Kind,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
        null: bool,
    ) -> Result<Self, TokenStream> {
        match null {
            true => {
                Self::try_build_vec_null::<H>(ty, of, vec_var, idx_var, metas_holder, dflt_lang)
            }

            false => {
                Self::try_build_vec_not_null::<H>(ty, of, vec_var, idx_var, metas_holder, dflt_lang)
            }
        }
    }

    #[inline]
    fn try_build_vec_null<H: MetasHolder>(
        ty: TokenStream2,
        of: Kind,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let type_mismatch_err;
        let processing;
        let elem_parsing;

        let sub_vec_var = format_ident!("{}1", vec_var);

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(
                    &metas_holder,
                    &ty,
                    &sub_vec_var,
                    &idx_var,
                    dflt_lang,
                )?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, "array")?;

                elem_parsing = Self::try_build(
                    &sub_vec_var,
                    quote! {i},
                    metas_holder.parse_list_if_found("elem")?,
                    dflt_lang,
                    of,
                )?;
            }

            None => {
                processing = Processing::new();

                type_mismatch_err = TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: array, found: {}", found)},
                );

                elem_parsing = Self::try_build::<H>(&sub_vec_var, quote! {i}, None, dflt_lang, of)?;
            }
        };

        let null_handling =
            NullHandling::from_one(quote! {#vec_var.push(::from::Null::<#ty>::Null);});

        let elem_push = quote! {
            #vec_var.push(::from::Null::<#ty>::Some(#sub_vec_var));
        };

        Ok(Self::build_vec(
            //quote! {#vec_var.push(::from::Null::<#ty>::Some(Vec::new()))},
            idx_var,
            sub_vec_var,
            elem_parsing,
            processing,
            elem_push,
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn try_build_vec_not_null<H: MetasHolder>(
        ty: TokenStream2, // ty
        of: Kind,
        vec_var: &Ident,
        idx_var: TokenStream2,
        metas_holder: Option<H>,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let type_mismatch_err;
        let processing;
        let elem_parsing;
        let null_handling;

        let sub_vec_var = format_ident!("{}1", vec_var);

        match metas_holder {
            Some(metas_holder) => {
                processing = Processing::try_build::<H>(
                    &metas_holder,
                    &ty,
                    &sub_vec_var,
                    &idx_var,
                    dflt_lang,
                )?;

                type_mismatch_err =
                    TypeMismatchErr::try_build(&metas_holder, &idx_var, &dflt_lang, "array")?;

                elem_parsing = Self::try_build(
                    &sub_vec_var,
                    quote! {i},
                    metas_holder.parse_list_if_found("elem")?,
                    dflt_lang,
                    of,
                )?;

                null_handling = NullHandling::try_build(
                    &metas_holder,
                    &idx_var,
                    &dflt_lang,
                    || -> TokenStream2 {
                        quote! {String::from("expected: array, found: null")}
                    },
                )?;
            }

            None => {
                processing = Processing::new();

                type_mismatch_err = TypeMismatchErr::new_dflt(
                    &idx_var,
                    quote! {format!("expected: array, found: {}", found)},
                );

                elem_parsing = Self::try_build::<H>(&sub_vec_var, quote! {i}, None, dflt_lang, of)?;

                null_handling = NullHandling::new_dflt(
                    &idx_var,
                    quote! {String::from("expected: array, found: null")},
                );
            }
        };

        let elem_push = quote! {
            #vec_var.push(#sub_vec_var);
        };

        Ok(Self::build_vec(
            //quote! {#vec_var.push(Vec::new())},
            idx_var,
            sub_vec_var,
            elem_parsing,
            processing,
            elem_push,
            null_handling,
            type_mismatch_err,
        ))
    }

    #[inline]
    fn build_vec(
        //elem_push_empty: TokenStream2,
        idx_var: TokenStream2,
        sub_vec_var: Ident,
        elem_parsing: Self,
        processing: Processing,
        elem_push: TokenStream2,
        null_handling: NullHandling,
        type_mismatch_err: TypeMismatchErr,
    ) -> Self {
        Self {
            none: vec_field_match_parse_temp(
                //&elem_push_empty,
                &idx_var,
                &sub_vec_var,
                elem_parsing.none,
                processing.valdg,
                &elem_push,
                null_handling.none,
                type_mismatch_err.none,
            ),

            lang: vec_field_match_parse_temp(
                //&elem_push_empty,
                &idx_var,
                &sub_vec_var,
                elem_parsing.lang,
                processing.valdg_lang,
                &elem_push,
                null_handling.lang,
                type_mismatch_err.lang,
            ),

            stack_errs: vec_field_match_parse_temp(
                //&elem_push_empty,
                &idx_var,
                &sub_vec_var,
                elem_parsing.stack_errs,
                processing.valdg_stack_errs,
                &elem_push,
                null_handling.stack_errs,
                type_mismatch_err.stack_errs,
            ),

            stack_errs_lang: vec_field_match_parse_temp(
                // &elem_push_empty,
                &idx_var,
                &sub_vec_var,
                elem_parsing.stack_errs_lang,
                processing.valdg_stack_errs_lang,
                &elem_push,
                null_handling.stack_errs_lang,
                type_mismatch_err.stack_errs_lang,
            ),
        }
    }
}

#[inline]
fn vec_field_match_parse_temp(
    // elem_push_empty: &TokenStream2,
    idx_var: &TokenStream2,
    sub_vec_var: &Ident,
    elem_parsing: TokenStream2,
    valdg: TokenStream2,
    elem_push: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
) -> TokenStream2 {
    quote! {
        match ::from::json::vec::check(json, idx) {
            ::from::json::vec::CheckResult::Ok => {

                ::from::json::utils::skip_whitespaces(json, idx);
                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                let mut #sub_vec_var = Vec::new();

                if byte == b']' {
                    *idx += 1;
                } else {
                    let mut path = path.clone();
                    path.push(From::from(#idx_var));
                    // custom validators take '&Path' as a parameter
                    // not 'Path'
                    let path = &path;


                    let mut i = 0usize;
                    loop {
                        #elem_parsing

                        ::from::json::utils::skip_whitespaces(json, idx);
                        byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                        match byte {
                            b',' => {
                                i += 1;
                                *idx+= 1;
                                ::from::json::utils::skip_whitespaces(json, idx);
                            },

                            b']' =>{
                                *idx +=1;
                                break;
                            },

                            _=> return Err(From::from(::from::SyntaxErr::unexpected_token("',' or ']'", &[byte], idx))),
                        }

                    }

                };

                #valdg
                #elem_push

            },

            ::from::json::vec::CheckResult::Null => {
                #null_handling
            },

            ::from::json::vec::CheckResult::TypeMismatch(found) => {
                #type_mismatch_err
            },

            ::from::json::vec::CheckResult::SyntaxErr(err) => {
                return Err(From::from(err));
            },
        };
    }
}
