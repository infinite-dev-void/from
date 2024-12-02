use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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

    pub fn try_build<H>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        if let Some(metas) = metas_holder.parse_list_if_found("must_be")? {
            let (value, _) = metas
                .parse_value_or_err::<bool>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("value must be {}", value);

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val != #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val != #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val != #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val != #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
            for path in paths {
                valdg.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<bool>>::none(&val, &path) {
                        return Err(From::from(e));
                    }
                });
                valdg_lang.push(quote! {
                    if let Err(e) = <#path as ::from::Validator<bool>>::lang(&val, &path, lang) {
                        return Err(From::from(e));
                    }
                });
                valdg_stack_errs.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<bool>>::stack_errs(&val, &path) {
                        errs.append(&mut errs2);
                    }
                });
                valdg_stack_errs_lang.push(quote! {
                    if let Err(mut errs2) = <#path as ::from::Validator<bool>>::stack_errs_lang(&val, &path, lang) {
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
    pub prop_or_idx: TokenStream2,
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
    pub fn try_build<H>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let processing = Processing::try_build(&metas_holder, &field_ident.quoted, dflt_lang)?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &metas_holder,
            &field_ident.quoted,
            &dflt_lang,
            "boolean",
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

            (false, true) => match metas_holder.parse_value_if_found::<Null<bool>>("default")? {
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

            (false, false) => match metas_holder.parse_value_if_found::<bool>("default")? {
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
            quoted: prop_or_idx,
        } = field_ident;

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::OptionNull::Null;});

        Self {
            field_var_def: quote! {
                let mut #var_name = ::from::OptionNull::<bool>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
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
    fn try_build_option_not_null<H>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &prop_or_idx,
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: boolean, found: null");
                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = Option::<bool>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
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
    fn try_build_rqd_null<H>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = Some(custom_types::MissingFieldCheck::try_build(
            &metas_holder,
            &prop_or_idx,
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("")}
            },
        )?);

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name = ::from::Null::<bool>::Null;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = ::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<bool>::Null;},
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
        dflt: Null<bool>,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let field_var_def = match dflt {
            Null::Some(s) => {
                quote! {
                    let mut #var_name = ::from::Null::Some(#s),
                }
            }
            Null::Null => {
                quote! {
                    let mut #var_name = ::from::Null::<bool>::Null;
                }
            }
        };

        Self {
            field_var_def,
            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name =::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<bool>::Null;},
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
    fn try_build_rqd_not_null<H>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = Some(custom_types::MissingFieldCheck::try_build(
            &metas_holder,
            &prop_or_idx,
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("")}
            },
        )?);

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &prop_or_idx,
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: boolean, found: null");

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name = false;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                type_mismatch_err,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: #var_name ,
            },
        })
    }

    #[inline]
    fn try_build_dflt_not_null<H>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: bool,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &metas_holder,
            &prop_or_idx,
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: boolean, found: null");

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = #dflt;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
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
