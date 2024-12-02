use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{metas_holder::MetasHolder, utils};
pub struct NullHandling {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl NullHandling {
    #[inline]
    pub fn try_build<H, F>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
        backup_msg: F,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
        F: Fn() -> TokenStream2,
    {
        let meta_name_values = metas_holder.parse_nvs_from_list_or_empty("not_null_msgs")?;

        let (single_msg_err, multi_msgs_err) =
            utils::msgs_attribute::process(&meta_name_values, prop_or_idx, dflt_lang, backup_msg)?;

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
    pub fn from_one(ts: TokenStream2) -> Self {
        Self {
            none: ts.clone(),
            lang: ts.clone(),
            stack_errs: ts.clone(),
            stack_errs_lang: ts,
        }
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
