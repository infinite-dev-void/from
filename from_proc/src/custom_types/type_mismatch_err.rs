use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{metas_holder::MetasHolder, utils};
pub struct TypeMismatchErr {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl TypeMismatchErr {
    pub fn try_build<H>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
        exp: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let metas = metas_holder.parse_nvs_from_list_or_empty("type_mismatch_msgs")?;

        let (single_msg_err, multi_msgs_err) =
            utils::msgs_attribute::process(&metas, prop_or_idx, dflt_lang, || -> TokenStream2 {
                let msg = format!("expected: {}, found: {{}}", exp);
                quote! { format!(#msg, found) }
            })?;

        Ok(Self {
            none: quote! {
                return Err(From::from(#single_msg_err));
            },
            lang: quote! {
                return Err(From::from(#multi_msgs_err));
            },

            stack_errs: quote! {
                errs.push(#single_msg_err);
            },

            stack_errs_lang: quote! {
                errs.push(#multi_msgs_err);
            },
        })
    }

    #[inline]
    pub fn new_dflt(prop_or_idx: &TokenStream2, dflt_msg: TokenStream2) -> Self {
        let (single_msg_err, multi_msgs_err) =
            utils::msgs_attribute::construct_dflt(prop_or_idx, dflt_msg);

        Self {
            none: quote! {
                return Err(From::from(#single_msg_err));
            },
            lang: quote! {
                return Err(From::from(#multi_msgs_err));
            },

            stack_errs: quote! {
                errs.push(#single_msg_err);
            },

            stack_errs_lang: quote! {
                errs.push(#multi_msgs_err);
            },
        }
    }
}
