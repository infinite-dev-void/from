use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Attribute;

use crate::{custom_types, metas_holder::MetasHolder, types, Null};

use super::FromJsonValueImpl;

impl FromJsonValueImpl {
    pub fn add_string_field(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<(), TokenStream> {
        let processing =
            types::string::Processing::try_build(&attrs, &field_ident.quoted, dflt_lang)?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &attrs,
            &field_ident.quoted,
            &dflt_lang,
            "string",
        )?;

        match (option, null) {
            (true, true) => Ok(self.option_null_string(field_ident, processing, type_mismatch_err)),

            (true, false) => self.option_not_null_string(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
            ),

            (false, true) => {
                self.rqd_null_string(field_ident, attrs, dflt_lang, processing, type_mismatch_err)
            }

            (false, false) => self.rqd_not_null_string(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
            ),
        }
    }

    #[inline]
    fn option_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: _quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name = ::from::OptionNull::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::OptionNull::Null;});

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        //

        self.add_field_var_def(quote! {
            let mut #var_name = ::from::OptionNull::<String>::None;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });
    }

    //
    //
    //
    //
    fn option_not_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name = Some(val);};

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &&dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #var_name = Option::<String>::None;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    //
    //
    //
    //
    //
    fn rqd_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<Null<String>>("default")? {
            self.dflt_null_string(field_ident, processing, type_mismatch_err, dflt);
            return Ok(());
        };

        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
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

        let var_assignment = quote! {#var_name = ::from::Null::Some(val);};

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::Null::<String>::Null;},
        );

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name = ::from::Null::<String>::Null;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    fn dflt_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: Null<String>,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: _quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
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

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name =::from::Null::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::Null;});

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(field_var_def);

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });
    }

    #[inline]
    fn rqd_not_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
    ) -> Result<(), TokenStream> {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<String>("default")? {
            return self.dflt_not_null_string(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
                dflt,
            );
        };

        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
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

        let var_assignment = quote! {#var_name = val;};

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &&dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name = String::new();
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    fn dflt_not_null_string(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::string::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        dflt: String,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &&dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: string, found: null")}
            },
        )?;

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name = val;};

        let field_parsing_arm = gen_string_field_parsing_arm(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
        );

        self.add_field_var_def(quote! {
            let mut #var_name = String::from(#dflt);
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }
}

#[inline]
fn gen_string_field_parsing_arm(
    quoted_field_ident: TokenStream2,
    before: TokenStream2,
    processing: types::string::Processing,
    var_assignment: TokenStream2,
    null_handling: custom_types::NullHandling,
    type_mismatch_err: custom_types::TypeMismatchErr,
) -> custom_types::FieldParsingArm {
    custom_types::FieldParsingArm {
        none: string_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            &processing.mutable,
            &processing.sanitizing,
            processing.valdg,
            &var_assignment,
            null_handling.none,
            type_mismatch_err.none,
        ),

        lang: string_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            &processing.mutable,
            &processing.sanitizing,
            processing.valdg_lang,
            &var_assignment,
            null_handling.lang,
            type_mismatch_err.lang,
        ),

        stack_errs: string_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            &processing.mutable,
            &processing.sanitizing,
            processing.valdg_stack_errs,
            &var_assignment,
            null_handling.stack_errs,
            type_mismatch_err.stack_errs,
        ),

        stack_errs_lang: string_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            &processing.mutable,
            &processing.sanitizing,
            processing.valdg_stack_errs_lang,
            &var_assignment,
            null_handling.stack_errs_lang,
            type_mismatch_err.stack_errs_lang,
        ),
    }
}

#[inline]
pub fn string_field_parsing_arm_temp(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    mutable: &TokenStream2,
    sanitizing: &TokenStream2,
    valdg: TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
) -> TokenStream2 {
    quote! {#quoted_field_ident => {
        #before
        match ::from::json::string::parse(json, idx) {
            ::from::json::string::ParseResult::Ok(#mutable val) => {
                #sanitizing
                #valdg
                #var_assignment
            }

            ::from::json::string::ParseResult::Null => {
                #null_handling
            }


            ::from::json::string::ParseResult::TypeMismatch(found) => {
                #type_mismatch_err
            }

            ::from::json::string::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        }
    }}
}
