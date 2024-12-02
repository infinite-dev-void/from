use std::fmt::{Debug, Display};

use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use crate::{
    metas_holder::{value::FromExpr, MetasHolder},
    utils, Tokens,
};

pub trait Ints: FromExpr + Display + ToTokens + Debug {
    fn expc() -> &'static str;
    fn ty() -> TokenStream2;
}
impl Ints for i8 {
    #[inline(always)]
    fn expc() -> &'static str {
        "i8"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {i8}
    }
}
impl Ints for i16 {
    #[inline(always)]
    fn expc() -> &'static str {
        "i16"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {i16}
    }
}
impl Ints for i32 {
    #[inline(always)]
    fn expc() -> &'static str {
        "i32"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {i32}
    }
}
impl Ints for i64 {
    #[inline(always)]
    fn expc() -> &'static str {
        "i64"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {i64}
    }
}
impl Ints for i128 {
    #[inline(always)]
    fn expc() -> &'static str {
        "i128"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {i128}
    }
}
impl Ints for isize {
    #[inline(always)]
    fn expc() -> &'static str {
        "isize"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {isize}
    }
}
impl Ints for u8 {
    #[inline(always)]
    fn expc() -> &'static str {
        "u8"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {u8}
    }
}
impl Ints for u16 {
    #[inline(always)]
    fn expc() -> &'static str {
        "u16"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {u16}
    }
}
impl Ints for u32 {
    #[inline(always)]
    fn expc() -> &'static str {
        "u32"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {u32}
    }
}
impl Ints for u64 {
    #[inline(always)]
    fn expc() -> &'static str {
        "u64"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {u64}
    }
}
impl Ints for u128 {
    #[inline(always)]
    fn expc() -> &'static str {
        "u128"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {u128}
    }
}
impl Ints for usize {
    #[inline(always)]
    fn expc() -> &'static str {
        "usize"
    }
    #[inline(always)]
    fn ty() -> TokenStream2 {
        quote! {usize}
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
    pub fn try_build<H: MetasHolder, I: Ints>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        if let Some(metas) = metas_holder.parse_list_if_found("max")? {
            let (value, _) = metas
                .parse_value_or_err::<I>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be less than or equal to {}", value);

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val > #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val > #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val > #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val > #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("min")? {
            let (value, _) = metas
                .parse_value_or_err::<I>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be greater than or equal to {}", value);

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val < #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val < #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val < #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val < #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("enum")? {
            let (values, _) = metas.parse_value_or_err::<Vec<I>>(
                "values",
                "'values' NameValue attribute is required",
            )?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
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

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
            let ty = I::ty();
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
/* pub struct Processing<I: Ints> {
    pub valdg: TokenStream2,                 // instant return - dflt message
    pub valdg_lang: TokenStream2,            // instant return - match lang
    pub valdg_stack_errs: TokenStream2,      // stack errs - dflt message
    pub valdg_stack_errs_lang: TokenStream2, // stack errs - match lang
    phantom: PhantomData<I>,
}

impl<I> Processing<I>
where
    I: Ints,
{
    pub fn try_build<H: MetasHolder>(
        metas_holder: &H,
        dflt_lang: &str,
        prop_or_idx: &TokenStream2,
    ) -> Result<Self, TokenStream> {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        if let Some(metas) = metas_holder.parse_list_if_found("max")? {
            let (value, _) = metas
                .parse_value_or_err::<I>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be less than or equal to {}", value);

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val > #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val > #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val > #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val > #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("min")? {
            let (value, _) = metas
                .parse_value_or_err::<I>("value", "'value' NameValue attribute is required")?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be greater than or equal to {}", value);

                    quote! {String::from(#msg)}
                },
            )?;

            valdg.push(quote! {
                if val < #value {
                    return Err(From::from(#single_msg_err));
                }
            });

            valdg_lang.push(quote! {
                if val < #value {
                    return Err(From::from(#multi_msgs_err));
                }
            });

            valdg_stack_errs.push(quote! {
                if val < #value {
                    errs.push(#single_msg_err);
                }
            });

            valdg_stack_errs_lang.push(quote! {
                if val < #value {
                    errs.push(#multi_msgs_err);
                }
            });
        };

        if let Some(metas) = metas_holder.parse_list_if_found("enum")? {
            let (values, _) = metas.parse_value_or_err::<Vec<I>>(
                "values",
                "'values' NameValue attribute is required",
            )?;

            let meta_name_values = metas.parse_nvs_from_list_or_empty("msgs")?;

            let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::process(
                &meta_name_values,
                &prop_or_idx,
                dflt_lang,
                || -> TokenStream2 {
                    let msg = format!("number must be one of: {:?}", values,);

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

        let r#else = quote! {else};
        let semi_colon = quote! {;};

        Ok(Self {
            valdg: valdg.join_with_suffix(&r#else, &semi_colon),

            valdg_lang: valdg_lang.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs: valdg_stack_errs.join_with_suffix(&r#else, &semi_colon),

            valdg_stack_errs_lang: valdg_stack_errs_lang.join_with_suffix(&r#else, &semi_colon),
            phantom: PhantomData,
        })
    }
} */

pub struct TooLargeErr {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl TooLargeErr {
    pub fn try_build<H: MetasHolder>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let metas = metas_holder.parse_nvs_from_list_or_empty("too_large_msgs")?;

        let (single_msg_err, multi_msgs_err) =
            utils::msgs_attribute::process(&metas, prop_or_idx, dflt_lang, || -> TokenStream2 {
                quote! { format!("number is too large to fit in '{}' type", typ) }
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
    pub fn new_dflt(prop_or_idx: &TokenStream2) -> Self {
        let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::construct_dflt(
            prop_or_idx,
            quote! { format!("number is too large to fit in '{}' type", typ) },
        );

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

pub struct TooSmallErr {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl TooSmallErr {
    pub fn try_build<H: MetasHolder>(
        metas_holder: &H,
        prop_or_idx: &TokenStream2,
        dflt_lang: &str,
    ) -> Result<Self, TokenStream> {
        let metas = metas_holder.parse_nvs_from_list_or_empty("too_small_msgs")?;

        let (single_msg_err, multi_msgs_err) =
            utils::msgs_attribute::process(&metas, prop_or_idx, dflt_lang, || -> TokenStream2 {
                quote! { format!("number is too small to fit in '{}' type", typ) }
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
    pub fn new_dflt(prop_or_idx: &TokenStream2) -> Self {
        let (single_msg_err, multi_msgs_err) = utils::msgs_attribute::construct_dflt(
            prop_or_idx,
            quote! { format!("number is too small to fit in '{}' type", typ) },
        );

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
/*
pub struct JsonParsingArmParams<I: Ints> {
    pub prop_or_idx: TokenStream2,
    pub before: TokenStream2,
    pub var_assignment: TokenStream2,
    pub parser_module: TokenStream2,
    pub processing: Processing<I>,
    pub null_handling: custom_types::NullHandling,
    pub type_mismatch_err: custom_types::TypeMismatchErr,
    pub too_large_err: TooLargeErr,
    pub too_small_err: TooSmallErr,
}

pub struct JsonSegVars<I>
where
    I: Ints,
    Null<I>: FromExpr,
{
    pub field_var_def: TokenStream2,
    pub parsing_arm_params: JsonParsingArmParams<I>,
    pub missing_field_check: Option<custom_types::MissingFieldCheck>,
    pub field_assignment: TokenStream2,
}

impl<I> JsonSegVars<I>
where
    I: Ints,
    Null<I>: FromExpr,
{
    pub fn try_build<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        option: bool,
        null: bool,
        ty: TokenStream2,
        parser_module: TokenStream2,
        exp: &str,
    ) -> Result<Self, TokenStream> {
        let processing = Processing::try_build(&metas_holder, dflt_lang, &field_ident.quoted)?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &metas_holder,
            &field_ident.quoted,
            dflt_lang,
            exp,
        )?;

        let too_large_err = TooLargeErr::try_build(&metas_holder, &field_ident.quoted, dflt_lang)?;

        let too_small_err = TooSmallErr::try_build(&metas_holder, &field_ident.quoted, dflt_lang)?;

        match (option, null) {
            (true, true) => Ok(Self::build_option_null(
                field_ident,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                ty,
                parser_module,
            )),

            (true, false) => Self::try_build_option_not_null(
                field_ident,
                metas_holder,
                dflt_lang,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                ty,
                parser_module,
                exp,
            ),

            (false, true) => match metas_holder.parse_value_if_found::<Null<I>>("default")? {
                Some((dflt, _)) => Ok(Self::build_dflt_null(
                    field_ident,
                    processing,
                    type_mismatch_err,
                    too_large_err,
                    too_small_err,
                    ty,
                    parser_module,
                    dflt,
                )),

                None => Self::try_build_rqd_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                    too_large_err,
                    too_small_err,
                    ty,
                    parser_module,
                ),
            },

            (false, false) => match metas_holder.parse_value_if_found::<I>("default")? {
                Some((dflt, _)) => Self::try_build_dflt_not_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                    too_large_err,
                    too_small_err,
                    ty,
                    parser_module,
                    dflt,
                    exp,
                ),

                None => Self::try_build_rqd_not_null(
                    field_ident,
                    metas_holder,
                    dflt_lang,
                    processing,
                    type_mismatch_err,
                    too_large_err,
                    too_small_err,
                    ty,
                    parser_module,
                    exp,
                ),
            },
        }
    }

    fn build_option_null(
        field_ident: custom_types::FieldIdent,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::OptionNull::<#ty>::Null;},
        );

        Self {
            field_var_def: quote! {
                let mut #var_name = ::from::OptionNull::<#ty>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = ::from::OptionNull::Some(val);},
                processing,
                null_handling,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        }
    }

    fn try_build_option_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
        exp: &str,
    ) -> Result<Self, TokenStream> {
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
                let msg = format!("expected: {}, found: null", exp);
                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = Option::<#ty>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = Some(val);},
                processing,
                null_handling,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },
            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    fn try_build_rqd_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
    ) -> Result<Self, TokenStream> {
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
                let mut #var_name = ::from::Null::<#ty>::Null;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = ::from::Null::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<#ty>::Null;},
                ),
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    fn build_dflt_null(
        field_ident: custom_types::FieldIdent,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
        dflt: Null<I>,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: prop_or_idx,
        } = field_ident;

        let field_var_def = match dflt {
            Null::Some(s) => {
                quote! {
                    let mut #var_name = ::from::Null::<#ty>::Some(#s),
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
                prop_or_idx,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name =::from::Null::<#ty>::Some(val);},
                processing,
                null_handling: custom_types::NullHandling::from_one(
                    quote! {#var_name = ::from::Null::<#ty>::Null;},
                ),
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        }
    }

    fn try_build_rqd_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
        exp: &str,
    ) -> Result<Self, TokenStream> {
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
                let msg = format!("expected: {}, found: null", exp);

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name: #ty = 0;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }

    fn try_build_dflt_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        dflt_lang: &str,
        processing: Processing<I>,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: TooLargeErr,
        too_small_err: TooSmallErr,
        ty: TokenStream2,
        parser_module: TokenStream2,
        dflt: I,
        exp: &str,
    ) -> Result<Self, TokenStream> {
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
                let msg = format!("expected: {}, found: null", exp);

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = #dflt #ty;
            },

            parsing_arm_params: JsonParsingArmParams {
                prop_or_idx,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                parser_module,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }
}
 */
