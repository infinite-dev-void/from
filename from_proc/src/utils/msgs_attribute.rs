use std::collections::HashMap;

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ext::IdentExt;

use crate::metas_holder::{value::FromExpr, MetaNameValues};

use super::Append;

#[track_caller]
#[inline(always)]
pub fn process<F>(
    meta_name_values: &MetaNameValues,
    prop_or_idx: &TokenStream2,
    dflt_lang: &str,
    backup_msg: F,
) -> Result<(TokenStream2, TokenStream2), TokenStream>
where
    F: Fn() -> TokenStream2,
{
    let mut langs_msgs = convert_to_hash_map(meta_name_values)?;

    let dflt_msg = match langs_msgs.remove(dflt_lang) {
        Some(msg) => quote! {String::from(#msg)},
        None => backup_msg(),
    };

    Ok(construct(langs_msgs, prop_or_idx, dflt_lang, dflt_msg))
}

#[track_caller]
#[inline]
fn convert_to_hash_map(metas: &MetaNameValues) -> Result<HashMap<String, String>, TokenStream> {
    let mut lang_msgs = HashMap::<String, String>::new();

    for nv in metas {
        match nv.path.get_ident() {
            Some(ident) => {
                let ident = ident.unraw().to_string();

                lang_msgs.insert(ident, String::from_expr(&nv.value)?);
            }

            None => continue,
        };
    }

    Ok(lang_msgs)
}

#[inline(always)]
pub fn construct(
    langs_msgs: HashMap<String, String>,
    prop_or_idx: &TokenStream2,
    dflt_lang: &str,
    dflt_msg: TokenStream2,
) -> (TokenStream2, TokenStream2) {
    let single_msg_err = quote! {
        ::from::ValidationErr {
            target: From::from(#prop_or_idx),
            path: path.clone(),
            msg: #dflt_msg,
        }
    };

    let multi_msgs_err = {
        match langs_msgs.len() {
            0 => single_msg_err.clone(),

            // > 1
            _ => {
                let mut arms = TokenStream2::new();
                for (lang, msg) in langs_msgs {
                    arms.append(quote! {
                        #lang => String::from(#msg),
                    });
                }

                arms.append(quote! { #dflt_lang | _ => #dflt_msg, });

                quote! {
                    ::from::ValidationErr {
                        target: From::from(#prop_or_idx),
                        path: path.clone(),
                        msg: match lang {#arms},
                    }
                }
            }
        }
    };

    (single_msg_err, multi_msgs_err)
}

#[inline]
pub fn construct_dflt(
    prop_or_idx: &TokenStream2,
    dflt_msg: TokenStream2,
) -> (TokenStream2, TokenStream2) {
    let msg = quote! {
        ::from::ValidationErr {
            target: From::from(#prop_or_idx),
            path: path.clone(),
            msg: String::from(#dflt_msg),
        }
    };

    (msg.clone(), msg)
}
