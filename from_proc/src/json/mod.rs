use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Generics, Ident};

use crate::{custom_types, VariableParseSegments};

// mod methods;
// mod from_json_value_impl;

mod string;

mod ints;

mod floats;

mod boolean;

mod custom;

mod vec;

pub struct FromJsonValueImpl {
    none: VariableParseSegments,
    lang: VariableParseSegments,
    stack_errs: VariableParseSegments,
    stack_errs_lang: VariableParseSegments,
}

impl FromJsonValueImpl {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            none: VariableParseSegments::new(),
            lang: VariableParseSegments::new(),
            stack_errs: VariableParseSegments::new(),
            stack_errs_lang: VariableParseSegments::new(),
        }
    }

    #[inline(always)]
    pub fn add_field_var_def(&mut self, field_var_def: TokenStream2) {
        self.none.add_field_var_def_ref(&field_var_def);

        self.lang.add_field_var_def_ref(&field_var_def);

        self.stack_errs.add_field_var_def_ref(&field_var_def);

        self.stack_errs_lang.add_field_var_def(field_var_def);
    }

    #[inline(always)]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: custom_types::FieldParsingArm) {
        self.none.add_field_parsing_arm(field_parsing_arm.none);
        self.lang.add_field_parsing_arm(field_parsing_arm.lang);
        self.stack_errs
            .add_field_parsing_arm(field_parsing_arm.stack_errs);
        self.stack_errs_lang
            .add_field_parsing_arm(field_parsing_arm.stack_errs_lang);
    }

    #[inline(always)]
    pub fn add_missing_field_check(
        &mut self,
        missing_field_check: custom_types::MissingFieldCheck,
    ) {
        self.none.add_missing_field_check(missing_field_check.none);
        self.lang.add_missing_field_check(missing_field_check.lang);
        self.stack_errs
            .add_missing_field_check(missing_field_check.stack_errs);
        self.stack_errs_lang
            .add_missing_field_check(missing_field_check.stack_errs_lang);
    }

    #[inline(always)]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.none.add_field_assignment_ref(&field_assignment);
        self.lang.add_field_assignment_ref(&field_assignment);
        self.stack_errs.add_field_assignment_ref(&field_assignment);
        self.stack_errs_lang.add_field_assignment(field_assignment);
    }

    #[inline]
    pub fn construct(self, ident: &Ident, generics: &Generics) -> TokenStream2 {
        let from_json_value_method = construct_from_json_value_method(self.none);

        let from_json_value_lang_method = construct_from_json_value_lang_method(self.lang);

        let from_json_value_stack_errs_method =
            construct_from_json_value_stack_errs_method(self.stack_errs);

        let from_json_value_stack_errs_lang_method =
            construct_from_json_value_stack_errs_lang_method(self.stack_errs_lang);

        quote! {
            impl #generics ::from::json::FromJsonValue for #ident #generics {
                #from_json_value_method

                #from_json_value_lang_method

                #from_json_value_stack_errs_method

                #from_json_value_stack_errs_lang_method
            }
        }
    }
}

#[inline]
fn construct_from_json_value_method(var_segs: VariableParseSegments) -> TokenStream2 {
    let VariableParseSegments {
        field_var_defs,
        field_parsing_arms,
        missing_field_checks,
        field_assignments,
    } = var_segs;

    quote! {
        fn from_json_value(json: &[u8], idx: &mut usize, path: &::from::Path) -> Result<Self, ::from::Err> {

            ::from::json::utils::skip_whitespaces(json, idx);
            ::from::json::utils::expect_and_skip(b'{', json, idx)?;
            ::from::json::utils::skip_whitespaces(json, idx);

            let mut byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

            #field_var_defs

            if byte.eq(&b'}') {
                #missing_field_checks

                return Ok(Self {
                    #field_assignments
                })
            };

            let mut prop;

            loop {

                prop = ::from::json::prop::parse(json, idx)?;

                ::from::json::utils::skip_whitespaces(json, idx);
                ::from::json::utils::expect_and_skip(b':', json, idx)?;
                ::from::json::utils::skip_whitespaces(json, idx);

                match prop {
                    #field_parsing_arms

                    _=> {
                        ::from::json::utils::skip_value(json, idx)?;
                    }
                };

                ::from::json::utils::skip_whitespaces(json, idx);

                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                if byte == b',' {
                    *idx += 1;
                    ::from::json::utils::skip_whitespaces(json, idx);
                    continue;
                };

                if byte == b'}' {
                    break;
                };

                return Err(::from::Err::SyntaxErr(::from::SyntaxErr::unexpected_token("',' or '}'", &[byte], idx)));
            }



            #missing_field_checks

            Ok(Self {
                #field_assignments
            })
        }
    }
}

#[inline]
fn construct_from_json_value_lang_method(var_segs: VariableParseSegments) -> TokenStream2 {
    let VariableParseSegments {
        field_var_defs,
        field_parsing_arms,
        missing_field_checks,
        field_assignments,
    } = var_segs;

    quote! {
        fn from_json_value_lang(json: &[u8], idx: &mut usize, path: &::from::Path, lang: &str) -> Result<Self, ::from::Err>{
            ::from::json::utils::skip_whitespaces(json, idx);
            ::from::json::utils::expect_and_skip(b'{', json, idx)?;
            ::from::json::utils::skip_whitespaces(json, idx);

            let mut byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

            #field_var_defs

            if byte.eq(&b'}') {
                #missing_field_checks

                return Ok(Self {
                    #field_assignments
                })
            };

            let mut prop;

            loop {

                prop = ::from::json::prop::parse(json, idx)?;

                ::from::json::utils::skip_whitespaces(json, idx);
                ::from::json::utils::expect_and_skip(b':', json, idx)?;
                ::from::json::utils::skip_whitespaces(json, idx);

                match prop {
                    #field_parsing_arms

                    _=> {
                        ::from::json::utils::skip_value(json, idx)?;
                    }
                };

                ::from::json::utils::skip_whitespaces(json, idx);

                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                if byte == b',' {
                    *idx += 1;
                    ::from::json::utils::skip_whitespaces(json, idx);
                    continue;
                };

                if byte == b'}' {
                    break;
                };

                return Err(::from::Err::SyntaxErr(::from::SyntaxErr::unexpected_token("',' or '}'", &[byte], idx)));
            }



            #missing_field_checks

            Ok(Self {
                #field_assignments
            })
        }
    }
}

#[inline]
fn construct_from_json_value_stack_errs_method(var_segs: VariableParseSegments) -> TokenStream2 {
    let VariableParseSegments {
        field_var_defs,
        field_parsing_arms,
        missing_field_checks,
        field_assignments,
    } = var_segs;

    quote! {
        fn from_json_value_stack_errs(json: &[u8], idx: &mut usize, path: &::from::Path) -> Result<Self, ::from::Errs>{
            ::from::json::utils::skip_whitespaces(json, idx);
            ::from::json::utils::expect_and_skip(b'{', json, idx)?;
            ::from::json::utils::skip_whitespaces(json, idx);

            let mut byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

            #field_var_defs

            let mut errs = Vec::<::from::ValidationErr>::new();

            if byte.eq(&b'}') {
                #missing_field_checks

                if errs.len() > 0 {
                    return Err(::from::Errs::ValidationErrs(errs));
                };

                return Ok(Self {
                    #field_assignments
                })
            };

            let mut prop;

            loop {

                prop = ::from::json::prop::parse(json, idx)?;

                ::from::json::utils::skip_whitespaces(json, idx);
                ::from::json::utils::expect_and_skip(b':', json, idx)?;
                ::from::json::utils::skip_whitespaces(json, idx);

                match prop {
                    #field_parsing_arms

                    _=> {
                        ::from::json::utils::skip_value(json, idx)?;
                    }
                };

                ::from::json::utils::skip_whitespaces(json, idx);

                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                if byte == b',' {
                    *idx += 1;
                    ::from::json::utils::skip_whitespaces(json, idx);
                    continue;
                };

                if byte == b'}' {
                    break;
                };

                return Err(::from::Errs::SyntaxErr(::from::SyntaxErr::unexpected_token("',' or '}'", &[byte], idx)));
            }


            #missing_field_checks

            if errs.len() > 0 {
                return Err(::from::Errs::ValidationErrs(errs));
            };

            Ok(Self {
                #field_assignments
            })
        }
    }
}

#[inline]
fn construct_from_json_value_stack_errs_lang_method(
    var_segs: VariableParseSegments,
) -> TokenStream2 {
    let VariableParseSegments {
        field_var_defs,
        field_parsing_arms,
        missing_field_checks,
        field_assignments,
    } = var_segs;

    quote! {
        fn from_json_value_stack_errs_lang(json: &[u8], idx: &mut usize, path: &::from::Path, lang: &str) -> Result<Self, ::from::Errs> {
            ::from::json::utils::skip_whitespaces(json, idx);
            ::from::json::utils::expect_and_skip(b'{', json, idx)?;
            ::from::json::utils::skip_whitespaces(json, idx);

            let mut byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

            #field_var_defs

            let mut errs = Vec::<::from::ValidationErr>::new();

            if byte.eq(&b'}') {
                #missing_field_checks

                if errs.len() > 0 {
                    return Err(::from::Errs::ValidationErrs(errs));
                };

                return Ok(Self {
                    #field_assignments
                })
            };

            let mut prop;

            loop {

                prop = ::from::json::prop::parse(json, idx)?;

                ::from::json::utils::skip_whitespaces(json, idx);
                ::from::json::utils::expect_and_skip(b':', json, idx)?;
                ::from::json::utils::skip_whitespaces(json, idx);

                match prop {
                    #field_parsing_arms

                    _=> {
                        ::from::json::utils::skip_value(json, idx)?;
                    }
                };

                ::from::json::utils::skip_whitespaces(json, idx);

                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                if byte == b',' {
                    *idx += 1;
                    ::from::json::utils::skip_whitespaces(json, idx);
                    continue;
                };

                if byte == b'}' {
                    break;
                };

                return Err(::from::Errs::SyntaxErr(::from::SyntaxErr::unexpected_token("',' or '}'", &[byte], idx)));
            }


            #missing_field_checks

            if errs.len() > 0 {
                return Err(::from::Errs::ValidationErrs(errs));
            };

            Ok(Self {
                #field_assignments
            })
        }
    }
}
