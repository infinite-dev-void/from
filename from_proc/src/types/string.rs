use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

//
//
//

use crate::{
    metas_holder::MetasHolder,
    utils::{self, Append},
    Tokens,
};

pub struct Processing {
    pub mutable: TokenStream2,
    pub sanitizing: TokenStream2,
    pub valdg: TokenStream2,                 // none
    pub valdg_lang: TokenStream2,            // lang
    pub valdg_stack_errs: TokenStream2,      // stack errs
    pub valdg_stack_errs_lang: TokenStream2, // stack errs - lang
}

impl Processing {
    #[inline]
    pub fn new() -> Self {
        Self {
            mutable: TokenStream2::new(),
            sanitizing: TokenStream2::new(),
            valdg: TokenStream2::new(),
            valdg_lang: TokenStream2::new(),
            valdg_stack_errs: TokenStream2::new(),
            valdg_stack_errs_lang: TokenStream2::new(),
        }
    }
    pub fn try_build<H: MetasHolder>(
        metas_holder: &H,
        quoted_field_ident: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let mut sanitizing = TokenStream2::new();

        if metas_holder.contains_ident("trim") {
            sanitizing.append(quote! {
                val = String::from(val.trim());
            });
        } else {
            match (
                metas_holder.contains_ident("trim_start"),
                metas_holder.contains_ident("trim_end"),
            ) {
                (true, true) => {
                    sanitizing.append(quote! {
                        val = String::from(val.trim());
                    });
                }

                (true, false) => {
                    sanitizing.append(quote! {
                        val = String::from(val.trim_start());
                    });
                }

                (false, true) => {
                    sanitizing.append(quote! {
                        val = String::from(val.trim_end());
                    });
                }

                _ => {}
            };
        };

        if metas_holder.contains_ident("sanitize_xss") {
            sanitizing.append(quote! {
                ::from::json::string::sanitize_xss(&mut val);
            });
        };

        //
        //
        //
        //
        //

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
                        "string must be {} {} long",
                        value,
                        if value > 1 { "bytes" } else { "byte" },
                    );

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val.len() != #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val.len() != #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val.len() != #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val.len() != #value {
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
                            "the string value must be no longer than {} {}",
                            value,
                            if value > 1 { "bytes" } else { "byte" },
                        );

                        quote! {String::from(#msg)}
                    },
                )?;

                valdg.push(quote! {
                    if val.len() > #value {
                        return Err(From::from(#single_msg_err));
                    }
                });

                valdg_lang.push(quote! {
                    if val.len() > #value {
                        return Err(From::from(#multi_msgs_err));
                    }
                });

                valdg_stack_errs.push(quote! {
                    if val.len() > #value {
                        errs.push(#single_msg_err);
                    }
                });

                valdg_stack_errs_lang.push(quote! {
                    if val.len() > #value {
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
                            "the string value must be at least {} {} long",
                            value,
                            if value > 1 { "bytes" } else { "byte" },
                        );

                        quote! {String::from(#msg)}
                    },
                )?;

                valdg.push(quote! {
                    if val.len() < #value {
                        return Err(From::from(#single_msg_err));
                    }
                });

                valdg_lang.push(quote! {
                    if val.len() < #value {
                        return Err(From::from(#multi_msgs_err));
                    }
                });

                valdg_stack_errs.push(quote! {
                    if val.len() < #value {
                        errs.push(#single_msg_err);
                    }
                });

                valdg_stack_errs_lang.push(quote! {
                    if val.len() < #value {
                        errs.push(#multi_msgs_err);
                    }
                });
            };
        };

        //
        //
        //
        //
        //
        //
        //

        // FIXME: 'metas' span pointing to the 'from' attribute not
        // to the 'r#enum' attribute
        if let Some(metas) = metas_holder.parse_list_if_found("enum")? {
            let (values, _) = metas.parse_value_or_err::<Vec<String>>(
                "values",
                "'values' NameValue attribute is required",
            )?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("value must be one of: {:?}", values,);

                    quote! {String::from(#msg)}
                },
            )?;

            let values = utils::vec_into_token_stream2_array(values);
            valdg.push(quote! {
                if ::from::utils::array_not_contains(#values, &val) {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if ::from::utils::array_not_contains(#values, &val) {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if ::from::utils::array_not_contains(#values, &val) {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if ::from::utils::array_not_contains(#values, &val) {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
            for path in paths {
                valdg.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<String>>::none(&val, &path) {
                        return Err(From::from(e));
                    }
                });
                valdg_lang.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<String>>::lang(&val, &path, lang) {
                        return Err(From::from(e));
                    }
                });
                valdg_stack_errs.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<String>>::stack_errs(&val, &path) {
                        errs.append(&mut errs2);
                    }
                });
                valdg_stack_errs_lang.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<String>>::stack_errs_lang(&val, &path, lang) {
                        errs.append(&mut errs2);
                    }
                });
            }
        };

        let r#else = quote! {else};
        let semi_colon = quote! {;};

        Ok(Self {
            mutable: if sanitizing.is_empty() {
                TokenStream2::new()
            } else {
                quote! {mut}
            },

            sanitizing,

            valdg: valdg.join_with_suffix(&r#else, &semi_colon),

            valdg_lang: valdg_lang.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs: valdg_stack_errs.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs_lang: valdg_stack_errs_lang.join_with_suffix(&r#else, &semi_colon),
        })
    }
}
/*
pub struct JsonParsingArmParams {
    pub quoted_field_ident: TokenStream2,
    pub before: TokenStream2,
    pub var_assignment: TokenStream2,
    pub processing: Processing,
    pub null_handling: custom_types::NullHandling,
    pub type_mismatch_err: custom_types::TypeMismatchErr,
}

pub struct JsonSegVars {
    pub field_var_def: TokenStream2,
    pub parsing_arm_params: JsonParsingArmParams,
    pub missing_field_check: Option<custom_types::MissingFieldCheck>,
    pub field_assignment: TokenStream2,
}

impl JsonSegVars {
    pub fn try_build<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<Self, TokenStream> {
        let processing = Processing::try_build(&metas_holder, &field_ident.quoted, dflt_lang)?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &metas_holder,
            &field_ident.quoted,
            &dflt_lang,
            "string",
        )?;

        match (option, null) {
            (true, true) => Ok(Self::build_option_null(
                field_ident,
                processing,
                type_mismatch_err,
            )),

            (true, false) => Self::try_build_option_not_null(
                field_ident,
                metas_holder,
                dflt_lang,
                processing,
                type_mismatch_err,
            ),

            (false, true) => match metas_holder.parse_value_if_found::<Null<String>>("default")? {
                Some((dflt, _)) => Ok(Self::build_dflt_null(
                    field_ident,
                    processing,
                    type_mismatch_err,
                    dflt,
                )),

                None => Self::try_build_rqd_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                ),
            },

            (false, false) => match metas_holder.parse_value_if_found::<String>("default")? {
                Some((dflt, _)) => Self::try_build_dflt_not_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                    dflt,
                ),

                None => Self::try_build_rqd_not_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                ),
            },
        }
    }

    #[inline]
    fn build_option_null(
        field_ident: custom_types::FieldIdent,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::OptionNull::Null;});

        Self {
            field_var_def: quote! {
                let mut #var_name = ::from::OptionNull::<String>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = ::from::OptionNull::Some(val);},
                processing,
                null_handling,
                type_mismatch_err,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        }
    }

    #[inline]
    fn try_build_option_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = Option::<String>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = Some(val);},
                processing,
                null_handling,
                type_mismatch_err,
            },
            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    #[inline]
    fn try_build_rqd_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = Some(custom_types::MissingFieldCheck::try_build(
            &metas_holder,
            &quoted_field_ident,
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("")}
            },
        )?);

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name = ::from::Null::<String>::Null;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = ::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<String>::Null;},
                ),
                type_mismatch_err,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    #[inline]
    fn build_dflt_null(
        field_ident: custom_types::FieldIdent,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: Null<String>,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let field_var_def = match dflt {
            Null::Some(s) => {
                quote! {
                    let mut #var_name = ::from::Null::Some(String::from(#s)),
                }
            }
            Null::Null => {
                quote! {
                    let mut #var_name = ::from::Null::<String>::Null;
                }
            }
        };

        Self {
            field_var_def,
            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name =::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::Null;},
                ),
                type_mismatch_err,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        }
    }

    #[inline]
    fn try_build_rqd_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = Some(custom_types::MissingFieldCheck::try_build(
            &metas_holder,
            &quoted_field_ident,
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("")}
            },
        )?);

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name = String::new();
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                type_mismatch_err,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    #[inline]
    fn try_build_dflt_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: String,
    ) -> Result<Self, TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = String::from(#dflt);
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                type_mismatch_err,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }
}
 */
