use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{metas_holder::MetasHolder, utils};

pub struct Check {
    pub check: TokenStream2,
    pub err: TokenStream2,
}

impl Check {
    fn new(check: TokenStream2, err: TokenStream2) -> Self {
        Self { check, err }
    }
}

pub struct MissingFieldCheck {
    pub none: Check,
    pub lang: Check,
    pub stack_errs: Check,
    pub stack_errs_lang: Check,
}

impl MissingFieldCheck {
    #[inline]
    pub fn try_build<H, F>(
        metas_holder: &H,
        quoted_field_ident: &TokenStream2,
        not_matching_indicator_ident: &Ident,
        dflt_lang: &str,
        backup_msg: F,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
        F: Fn() -> TokenStream2,
    {
        let meta_name_values = metas_holder.parse_nvs_from_list_or_empty("required_msgs")?;

        let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
            &meta_name_values,
            quoted_field_ident,
            dflt_lang,
            backup_msg,
        )?;

        Ok(Self {
            none: Check::new(
                quote! {
                    if #not_matching_indicator_ident {
                        return Err(From::from(#single_msg_err));
                    };
                },
                quote! {
                    #single_msg_err
                },
            ),

            lang: Check::new(
                quote! {
                    if #not_matching_indicator_ident {
                        return Err(From::from(#multi_msgs_err));
                    };
                },
                quote! {
                    #multi_msgs_err
                },
            ),

            stack_errs: Check::new(
                quote! {
                    if #not_matching_indicator_ident {
                        errs.push(#single_msg_err);
                    };
                },
                quote! {
                    #single_msg_err
                },
            ),

            stack_errs_lang: Check::new(
                quote! {
                    if #not_matching_indicator_ident {
                        errs.push(#multi_msgs_err);
                    };
                },
                quote! {
                    #multi_msgs_err
                },
            ),
        })
    }
}
