use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{metas_holder::MetasHolder, Tokens};
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
        ty: &TokenStream2,
        // quoted_field_ident: &TokenStream2,
        // dflt_lang: &str,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let mut valdg = Tokens::new();
        let mut valdg_lang = Tokens::new();
        let mut valdg_stack_errs = Tokens::new();
        let mut valdg_stack_errs_lang = Tokens::new();

        if let Some(paths) = metas_holder.parse_paths_from_list_if_found("validators")? {
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
pub struct JsonMethodCall {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl JsonMethodCall {
    pub fn new(ty: &TokenStream2) -> Self {
        Self {
            none: quote! {
                <#ty as ::from::json::FromJsonValue>::from_json_value(json, idx, &path)
            },

            lang: quote! {
                <#ty as ::from::json::FromJsonValue>::from_json_value_lang(json, idx, &path, lang)
            },

            stack_errs: quote! {
                <#ty as ::from::json::FromJsonValue>::from_json_value_stack_errs(json, idx, &path)
            },

            stack_errs_lang: quote! {
                <#ty as ::from::json::FromJsonValue>::from_json_value_stack_errs_lang(json, idx, &path, lang)
            },
        }
    }
}

pub struct JsonErrHandling {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl JsonErrHandling {
    pub fn new() -> Self {
        Self {
            none: quote! {
                return Err(e);
            },

            lang: quote! {
                return Err(e);
            },

            stack_errs: quote! {
                match e {
                    ::from::Errs::ValidationErrs(mut errs2) => {
                        errs.append(&mut errs2);
                    }

                    ::from::Errs::SyntaxErr(e) =>{
                        return Err(From::from(e));
                    }
                }
            },

            stack_errs_lang: quote! {
                match e {
                    ::from::Errs::ValidationErrs(mut errs2) => {
                        errs.append(&mut errs2);
                    }

                    ::from::Errs::SyntaxErr(e) =>{
                        return Err(From::from(e));
                    }
                }
            },
        }
    }
}

pub struct JsonParsingArmParams {
    pub quoted_field_ident: TokenStream2,
    pub before: TokenStream2,
    pub null_handling: custom_types::NullHandling,
    pub method_call: JsonMethodCall,
    pub var_assignment: TokenStream2,
    pub processing: Processing,
    pub err_handling: JsonErrHandling,
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
        ty: TokenStream2,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
        let processing = Processing::try_build(&metas_holder)?;

        let method_call = JsonMethodCall::new(&ty);
        let err_handling = JsonErrHandling::new();

        match (option, null) {
            (true, true) => Ok(Self::build_option_null(
                field_ident,
                ty,
                processing,
                method_call,
                err_handling,
            )),

            (true, false) => Self::try_build_option_not_null(
                field_ident,
                metas_holder,
                ty,
                dflt_lang,
                processing,
                method_call,
                err_handling,
            ),

            (false, true) => match metas_holder.parse_value_if_found::<Null<Path>>("default")? {
                Some((dflt, _)) => Ok(Self::build_dflt_null(
                    field_ident,
                    ty,
                    processing,
                    method_call,
                    err_handling,
                    dflt,
                )),

                None => Self::try_build_rqd_null(
                    field_ident,
                    metas_holder,
                    ty,
                    dflt_lang,
                    processing,
                    method_call,
                    err_handling,
                ),
            },

            (false, false) => match metas_holder.parse_value_if_found::<Path>("default")? {
                Some((dflt, _)) => Self::try_build_dflt_not_null(
                    field_ident,
                    metas_holder,
                    ty,
                    dflt_lang,
                    processing,
                    method_call,
                    err_handling,
                    dflt,
                ),

                None => Self::try_build_rqd_not_null(
                    field_ident,
                    metas_holder,
                    ty,
                    dflt_lang,
                    processing,
                    method_call,
                    err_handling,
                ),
            },
        }
    }

    #[inline]
    fn build_option_null(
        field_ident: custom_types::FieldIdent,
        ty: TokenStream2,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::OptionNull::<#ty>::Null;},
        );

        Self {
            field_var_def: quote! {
                let mut #var_name = ::from::OptionNull::<#ty>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = ::from::OptionNull::<#ty>::Some(val);},
                processing,
                null_handling,
                method_call,
                err_handling,
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
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
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
                let msg = unsafe {
                    format!(
                        "expected: {}, found: null",
                        ty.to_string().split("::").last().unwrap_unchecked()
                    )
                };
                quote! {String::from(#msg)}
            },
        )?;

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
                method_call,
                err_handling,
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
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
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
                method_call,
                err_handling,
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
        ty: TokenStream2,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
        dflt: Null<Path>,
    ) -> Self {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
        } = field_ident;

        let field_var_def = match dflt {
            Null::Some(p) => {
                quote! {
                    let mut #var_name = #p,
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
                method_call,
                err_handling,
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
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
    ) -> Result<Self, TokenStream>
    where
        H: MetasHolder,
    {
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
                let msg = unsafe {
                    format!(
                        "expected: {}, found: null",
                        ty.to_string().split("::").last().unwrap_unchecked()
                    )
                };

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #not_matching_indicator_ident = true;
                let mut #var_name = Option::<#ty>::None;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: quote! {#not_matching_indicator_ident = false;},
                var_assignment: quote! {#var_name = Some(val);},
                processing,
                null_handling,
                method_call,
                err_handling,
            },

            missing_field_check,

            field_assignment: quote! {
                #field_ident: unsafe { #var_name.unwrap_unchecked() },
            },
        })
    }

    #[inline]
    fn try_build_dflt_not_null<H: MetasHolder>(
        field_ident: custom_types::FieldIdent,
        metas_holder: H,
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: JsonMethodCall,
        err_handling: JsonErrHandling,
        dflt: Path,
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
                let msg = unsafe {
                    format!(
                        "expected: {}, found: null",
                        ty.to_string().split("::").last().unwrap_unchecked()
                    )
                };

                quote! {String::from(#msg)}
            },
        )?;

        Ok(Self {
            field_var_def: quote! {
                let mut #var_name = #dflt;
            },

            parsing_arm_params: JsonParsingArmParams {
                quoted_field_ident,
                before: TokenStream2::new(),
                var_assignment: quote! {#var_name = val;},
                processing,
                null_handling,
                method_call,
                err_handling,
            },

            missing_field_check: None,
            field_assignment: quote! {
                #field_ident: #var_name,
            },
        })
    }
} */
