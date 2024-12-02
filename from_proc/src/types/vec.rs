use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::{metas_holder::MetasHolder, utils, Tokens};

pub struct Processing {
    pub valdg: TokenStream2,                 // instant return - dflt message
    pub valdg_lang: TokenStream2,            // instant return - match lang
    pub valdg_stack_errs: TokenStream2,      // stack errs - dflt message
    pub valdg_stack_errs_lang: TokenStream2, // stack errs - match lang
}

impl Processing {
    #[inline]
    pub fn new() -> Self {
        Self {
            valdg: TokenStream2::new(),
            valdg_lang: TokenStream2::new(),
            valdg_stack_errs: TokenStream2::new(),
            valdg_stack_errs_lang: TokenStream2::new(),
        }
    }

    pub fn try_build<H: MetasHolder>(
        metas_holder: &H,
        ty: &TokenStream2,
        field_var_name: &Ident,
        quoted_field_ident: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        // FIXME: 'metas' span pointing to the 'from' attribute not
        // to the 'len' attribute
        if let Some(metas) = metas_holder.parse_list_if_found("len")? {
            let (value, _) = metas
                .parse_value_or_err::<usize>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!(
                        "array must contains {} {}",
                        value,
                        if value < 2 { "element" } else { "elements" },
                    );

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if #field_var_name.len() != #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if #field_var_name.len() != #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if #field_var_name.len() != #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if #field_var_name.len() != #value {
                    errs.push(#multi_msgs_err);
                }
            });

        // if len attribute is missing
        } else {
            let mut max = Option::<usize>::None;

            // FIXME: 'metas' span pointing to the 'from' attribute not
            // to the 'max_len' attribute
            if let Some(metas) = metas_holder.parse_list_if_found("max_len")? {
                let (value, _) = metas.parse_value_or_err::<usize>(
                    "value",
                    "'value' NameValue attribute is required",
                )?;

                let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

                let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                    &meta_name_values,
                    &quoted_field_ident,
                    dflt_lang,
                    || -> TokenStream2 {
                        let msg = format!(
                            "array must not contains more than {} {}",
                            value,
                            if value < 2 { "element" } else { "elements" },
                        );

                        quote! {String::from(#msg)}
                    },
                )?;

                valdg.push(quote! {
                    if #field_var_name.len() > #value {
                        return Err(From::from(#single_msg_err));
                    }
                });

                valdg_lang.push(quote! {
                    if #field_var_name.len() > #value {
                        return Err(From::from(#multi_msgs_err));
                    }
                });

                valdg_stack_errs.push(quote! {
                    if #field_var_name.len() > #value {
                        errs.push(#single_msg_err);
                    }
                });

                valdg_stack_errs_lang.push(quote! {
                    if #field_var_name.len() > #value {
                        errs.push(#multi_msgs_err);
                    }
                });

                max = Some(value);
            };

            // FIXME: 'metas' span pointing to the 'from' attribute not
            // to the 'min_len' attribute
            if let Some(metas) = metas_holder.parse_list_if_found("min_len")? {
                let (value, refr) = metas.parse_value_or_err::<usize>(
                    "value",
                    "'value' NameValue attribute is required",
                )?;

                if let Some(max) = max {
                    if value > max {
                        return Err(utils::compile_err(
                            &refr.value,
                            "minimum length must be greater than maximum length",
                        ));
                    };

                    if value == max {
                        return Err(utils::compile_err(
                            &refr.value,
                            "you must use 'len' attribute if you want to restrict the length",
                        ));
                    };
                };

                let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

                let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                    &meta_name_values,
                    &quoted_field_ident,
                    dflt_lang,
                    || -> TokenStream2 {
                        let msg = format!(
                            "array must not contains less than {} {}",
                            value,
                            if value < 2 { "element" } else { "elements" },
                        );

                        quote! {String::from(#msg)}
                    },
                )?;

                valdg.push(quote! {
                    if #field_var_name.len() < #value {
                        return Err(From::from(#single_msg_err));
                    }
                });

                valdg_lang.push(quote! {
                    if #field_var_name.len() < #value {
                        return Err(From::from(#multi_msgs_err));
                    }
                });

                valdg_stack_errs.push(quote! {
                    if #field_var_name.len() < #value {
                        errs.push(#single_msg_err);
                    }
                });

                valdg_stack_errs_lang.push(quote! {
                    if #field_var_name.len() < #value {
                        errs.push(#multi_msgs_err);
                    }
                });
            };
        };

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
            for path in paths {
                valdg.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<#ty>>::none(&#field_var_name, &path) {
                        return Err(From::from(e));
                    }
                });
                valdg_lang.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<#ty>>::lang(&#field_var_name, &path, lang) {
                        return Err(From::from(e));
                    }
                });
                valdg_stack_errs.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<#ty>>::stack_errs(&#field_var_name, &path) {
                        errs.append(&mut errs2);
                    }
                });
                valdg_stack_errs_lang.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<#ty>>::stack_errs_lang(&#field_var_name, &path, lang) {
                        errs.append(&mut errs2);
                    }
                });
            }
        };

        let r#else = quote! {else};
        let semi_colon = quote! {;};

        Ok(Self {
            valdg: valdg.join_with_suffix(&r#else, &semi_colon),

            valdg_lang: valdg_lang.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs: valdg_stack_errs.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs_lang: valdg_stack_errs_lang.join_with_suffix(&r#else, &semi_colon),
        })
    }
}

/* pub struct JsonParsingArmParams {
    pub quoted_field_ident: TokenStream2,
    pub before: TokenStream2,
    pub var_assignment: TokenStream2,
    pub processing: Processing,
    pub null_handling: custom_types::NullHandling,
    pub type_mismatch_err: custom_types::TypeMismatchErr,
} */
