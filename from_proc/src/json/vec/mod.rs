use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Attribute;

use crate::{custom_types, metas_holder::MetasHolder, types, Kind};

mod elem_parsing;
use elem_parsing::ElemParsing;

type Processing = types::vec::Processing;

impl super::FromJsonValueImpl {
    #[inline]
    pub fn add_vec_field(
        &mut self,
        ty: TokenStream2,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        dflt_lang: &str,
        option: bool,
        null: bool,
        of: Kind,
    ) -> Result<(), TokenStream> {
        let elem_parsing = ElemParsing::try_build(
            &format_ident!("vec"),
            quote! {i},
            attrs.parse_list_if_found("elem")?,
            dflt_lang,
            of,
        )?;

        let processing = Processing::try_build(
            &attrs,
            &ty,
            &field_ident.var_name,
            &field_ident.quoted,
            dflt_lang,
        )?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &attrs,
            &field_ident.quoted,
            &dflt_lang,
            "array",
        )?;

        match (option, null) {
            (true, true) => Ok(self.option_null_vec(
                ty,
                field_ident,
                elem_parsing,
                processing,
                type_mismatch_err,
            )),

            (true, false) => self.option_not_null_vec(
                ty,
                field_ident,
                attrs,
                dflt_lang,
                elem_parsing,
                processing,
                type_mismatch_err,
            ),

            (false, true) => self.rqd_null_vec(
                ty,
                field_ident,
                attrs,
                dflt_lang,
                elem_parsing,
                processing,
                type_mismatch_err,
            ),

            (false, false) => self.rqd_not_null_vec(
                ty,
                field_ident,
                attrs,
                dflt_lang,
                elem_parsing,
                processing,
                type_mismatch_err,
            ),
        }
    }

    #[inline]
    fn option_null_vec(
        &mut self,
        ty: TokenStream2,
        field_ident: custom_types::FieldIdent,
        elem_parsing: ElemParsing,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) {
        let custom_types::FieldIdent {
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
            var_name,
        } = field_ident;

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::OptionNull::<#ty>::Null;},
        );

        let field_parsing_arm = vec_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            TokenStream2::new(),
            quote! {#var_name = ::from::OptionNull::<#ty>::Some(Vec::new());},
            elem_parsing,
            processing,
            quote! {
                #var_name = ::from::OptionNull::<#ty>::Some(vec);
            },
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #var_name = ::from::OptionNull::<#ty>::None;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });
    }

    #[inline]
    fn option_not_null_vec(
        &mut self,
        ty: TokenStream2,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        elem_parsing: ElemParsing,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
            var_name,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: array, found: null")}
            },
        )?;

        let field_parsing_arm = vec_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            TokenStream2::new(),
            quote! {#var_name = Option::<#ty>::Some(Vec::new());},
            elem_parsing,
            processing,
            quote! {
                #var_name = Option::<#ty>::Some(vec);
            },
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #var_name = Option::<#ty>::None;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    #[inline]
    fn rqd_null_vec(
        &mut self,
        ty: TokenStream2,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        elem_parsing: ElemParsing,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
            var_name,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = custom_types::MissingFieldCheck::try_build(
            &attrs,
            &quoted_field_ident,
            &&not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("required field")}
            },
        )?;

        let before = quote! {#not_matching_indicator_ident = false;};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::<#ty>::Null;});

        let field_parsing_arm = vec_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            quote! {#var_name = ::from::Null::<#ty>::Some(Vec::new());},
            elem_parsing,
            processing,
            quote! {
                #var_name = ::from::Null::<#ty>::Some(vec);
            },
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name = ::from::Null::<#ty>::Null;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    #[inline]
    fn rqd_not_null_vec(
        &mut self,
        ty: TokenStream2,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        elem_parsing: ElemParsing,
        processing: Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
            var_name,
        } = field_ident;

        let not_matching_indicator_ident = format_ident!("not_matched_{}", field_ident);

        let missing_field_check = custom_types::MissingFieldCheck::try_build(
            &attrs,
            &quoted_field_ident,
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("required field")}
            },
        )?;

        let before = quote! {#not_matching_indicator_ident = false;};

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: array, found: null")}
            },
        )?;

        let field_parsing_arm = vec_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            quote! {#var_name = Vec::new();},
            elem_parsing,
            processing,
            quote! {
                #var_name = vec;
            },
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name = <#ty>::new();
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }
}

#[inline]
fn vec_field_parsing_arm(
    byte_quoted_field_ident: TokenStream2,
    quoted_field_ident: TokenStream2,
    before: TokenStream2,
    var_assignment_empty: TokenStream2,
    elem_parsing: ElemParsing,
    processing: Processing,
    var_assignment: TokenStream2,
    null_handling: custom_types::NullHandling,
    type_mismatch_err: custom_types::TypeMismatchErr,
) -> custom_types::FieldParsingArm {
    custom_types::FieldParsingArm {
        none: vec_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            &var_assignment_empty,
            elem_parsing.none,
            processing.valdg,
            &var_assignment,
            null_handling.none,
            type_mismatch_err.none,
        ),

        lang: vec_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            &var_assignment_empty,
            elem_parsing.lang,
            processing.valdg_lang,
            &var_assignment,
            null_handling.lang,
            type_mismatch_err.lang,
        ),

        stack_errs: vec_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            &var_assignment_empty,
            elem_parsing.stack_errs,
            processing.valdg_stack_errs,
            &var_assignment,
            null_handling.stack_errs,
            type_mismatch_err.stack_errs,
        ),

        stack_errs_lang: vec_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            &var_assignment_empty,
            elem_parsing.stack_errs_lang,
            processing.valdg_stack_errs_lang,
            &var_assignment,
            null_handling.stack_errs_lang,
            type_mismatch_err.stack_errs_lang,
        ),
    }
}

#[inline]
fn vec_field_parsing_arm_temp(
    byte_quoted_field_ident: &TokenStream2,
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    var_assignment_empty: &TokenStream2,
    elem_parsing: TokenStream2,
    valdg: TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
) -> TokenStream2 {
    quote! {
        #byte_quoted_field_ident => {
            #before

            match ::from::json::vec::check(json, idx) {
                ::from::json::vec::CheckResult::Ok => {

                    ::from::json::utils::skip_whitespaces(json, idx);
                    byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                    if byte == b']' {
                        *idx += 1;
                        #var_assignment_empty
                    } else {
                        let mut path = path.clone();
                        path.push(From::from(#quoted_field_ident));
                        // custom validators take '&Path' as a parameter
                        // not 'Path'
                        let path = &path;

                        let mut vec = Vec::new();
                        let mut i = 0usize;
                        loop {


                            #elem_parsing

                            ::from::json::utils::skip_whitespaces(json, idx);
                            byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                            match byte {
                                b',' => {
                                    i += 1;
                                    *idx+= 1;
                                    ::from::json::utils::skip_whitespaces(json, idx);
                                },

                                b']' =>{
                                    *idx +=1;
                                    break;
                                },

                                _=> return Err(From::from(::from::SyntaxErr::unexpected_token("',' or ']'", &[byte], idx))),
                            }

                        }

                        #var_assignment
                    };

                    #valdg

                },

                ::from::json::vec::CheckResult::Null => {
                    #null_handling
                },

                ::from::json::vec::CheckResult::TypeMismatch(found) => {
                    #type_mismatch_err
                },

                ::from::json::vec::CheckResult::SyntaxErr(err) => {
                    return Err(From::from(err));
                },
            };
        }
    }
}

// Different cases (lang, stack_errs) have not been considered yet.

/*
   "field1" => 'parsing: {
        #before

        match ::from::json::vec::check(json, idx) {
            ::from::json::vec::CheckResult::Ok => {},
            ::from::json::vec::CheckResult::Null => {},
            ::from::json::vec::CheckResult::TypeMismatch => {},
            ::from::json::vec::CheckResult::SyntaxErr => {},
        }

        ::from::json::utils::skip_whitespaces(json, idx);

        byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

        if byte == "]" {
            #var_name = Vec::new();

        } else {
            let mut vec = Vec::new();
            loop {
                // match parse
                // processing inside match;

                ::from::json::utils::skip_whitespaces(json, idx);
                byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;

                match byte {
                    b',' => {
                        *idx+= 1;
                        ::from::json::utils::skip_whitespaces(json, idx);
                    },

                    b']' =>{
                        *idx +=1;
                        break;
                    },

                    _=>{ unexpected}
                }

            }

            #processing
            #var_name = vec;
        }
   }
*/
