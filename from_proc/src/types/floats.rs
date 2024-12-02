use std::fmt::{Debug, Display};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use crate::{
    metas_holder::{value::FromExpr, MetasHolder},
    utils, Tokens,
};

pub trait Floats: FromExpr + ToTokens + Display + Debug {
    fn name_of_fracs_counter_util() -> TokenStream2;
    fn exp() -> &'static str;
    fn ty() -> TokenStream2;
}

impl Floats for f32 {
    fn name_of_fracs_counter_util() -> TokenStream2 {
        quote! {num_of_f32_fracs}
    }

    fn exp() -> &'static str {
        "f32"
    }

    fn ty() -> TokenStream2 {
        quote! {f32}
    }
}
impl Floats for f64 {
    fn name_of_fracs_counter_util() -> TokenStream2 {
        quote! {num_of_f64_fracs}
    }

    fn exp() -> &'static str {
        "f64"
    }

    fn ty() -> TokenStream2 {
        quote! {f64}
    }
}

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

    pub fn try_build<H: MetasHolder, F: Floats>(
        metas_holder: &H,
        quoted_field_ident: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        // for now this is not possible since float parser consider 'Nan' as
        // an invalid digit
        /*
        if !metas_holder.contains_ident("allow_nan") {
            let meta_name_values = metas_holder.parse_nvs_from_list_or_empty("nan_msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must not be a Nan");
                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val.is_nan() {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val.is_nan() {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val.is_nan() {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val.is_nan() {
                    errs.push(#multi_msgs_err);
                }
            });
        }; */

        if !metas_holder.contains_ident("allow_infinite") {
            let meta_name_values = metas_holder.parse_nvs_from_list_or_empty("infinite_msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must not be Infinite");
                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val.is_infinite() {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val.is_infinite() {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val.is_infinite() {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val.is_infinite() {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("max")? {
            let (value, _) = metas
                .parse_value_or_err::<F>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let msg;
            let sign;
            if metas.contains_ident("inclusive") {
                msg = format!("number must be less than {}", value);
                sign = quote! {>=};
            } else {
                msg = format!("number must be less than or equal to {}", value);
                sign = quote! {>};
            };

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val #sign #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val #sign #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val #sign #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val #sign #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("min")? {
            let (value, _) = metas
                .parse_value_or_err::<F>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let msg;
            let sign;
            if metas.contains_ident("inclusive") {
                msg = format!("number must be greater than {}", value);
                sign = quote! {<=};
            } else {
                msg = format!("number must be greater than or equal to {}", value);
                sign = quote! {<};
            };

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val #sign #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val #sign #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val #sign #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val #sign #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("enum")? {
            let (values, _) = metas.parse_value_or_err::<Vec<F>>(
                "values",
                "'values' NameValue attribute is required",
            )?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be one of: {:?}", values,);

                    quote! {String::from(#msg)}
                },
            )?;

            let values = utils::vec_into_token_stream2_array(values);
            valdg.push(quote! {
                if ::from::utils::array_not_contains(#values, val) {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if ::from::utils::array_not_contains(#values, val) {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if ::from::utils::array_not_contains(#values, val) {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if ::from::utils::array_not_contains(#values, val) {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("max_fracs")? {
            let (value, _) = metas
                .parse_value_or_err::<usize>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &quoted_field_ident,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("fraction digits must not be more than {}", value,);

                    quote! {String::from(#msg)}
                },
            )?;

            let fracs_counter = F::name_of_fracs_counter_util();

            valdg.push(quote! {
                if ::from::utils::#fracs_counter(val) > #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if ::from::utils::#fracs_counter(val) > #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if ::from::utils::num_of_f32_fracs(val) > #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if ::from::utils::num_of_f32_fracs(val) > #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
            let ty = F::ty();
            for path in paths {
                valdg.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<#ty>>::none(&val, &path) {
                        return Err(From::from(e));
                    }
                });
                valdg_lang.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<#ty>>::lang(&val, &path, lang) {
                        return Err(From::from(e));
                    }
                });
                valdg_stack_errs.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<#ty>>::stack_errs(&val, &path) {
                        errs.append(&mut errs2);
                    }
                });
                valdg_stack_errs_lang.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<#ty>>::stack_errs_lang(&val, &path, lang) {
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
    pub fn try_build<H: MetasHolder, F>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        F: Floats,
        Null<F>: FromExpr,
    {
        let processing =
            Processing::try_build::<H, F>(&metas_holder, &field_ident.quoted, dflt_lang)?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &metas_holder,
            &field_ident.quoted,
            &dflt_lang,
            F::exp(),
        )?;

        match (option, null) {
            (true, true) => Ok(Self::build_option_null::<F>(
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

            (false, true) => match metas_holder.parse_value_if_found::<Null<F>>("default")? {
                Some((dflt, _)) => Ok(Self::build_dflt_null(
                    field_ident,
                    processing,
                    type_mismatch_err,
                    dflt,
                )),

                None => Self::try_build_rqd_null::<H, F>(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                ),
            },

            (false, false) => match metas_holder.parse_value_if_found::<F>("default")? {
                Some((dflt, _)) => Self::try_build_dflt_not_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                    dflt,
                ),

                None => Self::try_build_rqd_not_null::<H, F>(
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
    fn build_option_null<F: Floats>(
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

        let ty = F::ty();

        Self {
            field_var_def: quote! {
                let mut #var_name = ::from::OptionNull::<#ty>::None;
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
    fn try_build_option_not_null<H: MetasHolder, F>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream>
    where
        F: Floats,
        Null<F>: FromExpr,
    {
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
                let msg = format!("expected: {}, found: null", F::exp());
                quote! {String::from(#msg)}
            },
        )?;

        let ty = F::ty();
        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = Option::<#ty>::None;
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
    fn try_build_rqd_null<H: MetasHolder, F: Floats>(
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

        let ty = F::ty();

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = false;
                let mut #var_name = ::from::Null::<#ty>::Null;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = ::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<#ty>::Null;},
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
    fn build_dflt_null<F>(
        field_ident: custom_types::FieldIdent,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: Null<F>,
    ) -> Self
    where
        F: Floats,
        Null<F>: FromExpr,
    {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let ty = F::ty();

        let field_var_def = match dflt {
            Null::Some(s) => {
                quote! {
                    let mut #var_name = ::from::Null::Some(#s),
                }
            }
            Null::Null => {
                quote! {
                    let mut #var_name = ::from::Null::<#ty>::Null;
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
                    quote! {#var_name = ::from::Null::<#ty>::Null;},
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
    fn try_build_rqd_not_null<H: MetasHolder, F: Floats>(
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
                let msg = format!("expected: {}, found: null", F::exp());

                quote! {String::from(#msg)}
            },
        )?;

        let ty = F::ty();

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = false;
                let mut #var_name: #ty = 0.0;
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
    fn try_build_dflt_not_null<H: MetasHolder, F: Floats>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: F,
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
                let msg = format!("expected: {}, found: null", F::exp());

                quote! {String::from(#msg)}
            },
        )?;

        let ty = F::ty();

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name: #ty = #dflt;
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
