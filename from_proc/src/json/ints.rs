use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Attribute;

use crate::{
    custom_types,
    metas_holder::{value::FromExpr, MetasHolder},
    types::{self, ints::Ints},
    Null,
};

use super::FromJsonValueImpl;

impl FromJsonValueImpl {
    pub fn add_int_field<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<(), TokenStream>
    where
        Null<I>: FromExpr,
    {
        let processing = types::ints::Processing::try_build::<Vec<Attribute>, I>(
            &attrs,
            &field_ident.quoted,
            dflt_lang,
        )?;

        let type_mismatch_err = custom_types::TypeMismatchErr::try_build(
            &attrs,
            &field_ident.quoted,
            dflt_lang,
            I::expc(),
        )?;

        let too_large_err =
            types::ints::TooLargeErr::try_build(&attrs, &field_ident.quoted, dflt_lang)?;

        let too_small_err =
            types::ints::TooSmallErr::try_build(&attrs, &field_ident.quoted, dflt_lang)?;

        match (option, null) {
            (true, true) => Ok(self.option_null_int::<I>(
                field_ident,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
            )),

            (true, false) => self.option_not_null_int::<I>(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
            ),

            (false, true) => self.rqd_null_int::<I>(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
            ),

            (false, false) => self.rqd_not_null_int::<I>(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
            ),
        }
    }

    #[inline]
    fn option_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: _quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let ty = I::ty();

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name = ::from::OptionNull::Some(val);};

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::OptionNull::<#ty>::Null;},
        );

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
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
    fn option_not_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        dflt_lang: &str,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
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
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: {}, found: null", I::expc());
                quote! {String::from(#msg)}
            },
        )?;

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        );

        let ty = I::ty();

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
    fn rqd_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
    ) -> Result<(), TokenStream>
    where
        Null<I>: FromExpr,
    {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<Null<I>>("default")? {
            return Ok(self.dflt_null_int(
                field_ident,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
                dflt,
            ));
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
            &not_matching_indicator_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("required field")}
            },
        )?;

        let ty = I::ty();

        let before = quote! {#not_matching_indicator_ident = false;};

        let var_assignment = quote! {#var_name = ::from::Null::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::<#ty>::Null;});

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
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
    fn dflt_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
        dflt: Null<I>,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: _quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let ty = I::ty();

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

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name =::from::Null::<#ty>::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::<#ty>::Null;});

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        );

        self.add_field_var_def(field_var_def);

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });
    }

    #[inline]
    fn rqd_not_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
    ) -> Result<(), TokenStream> {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<I>("default")? {
            return self.dflt_not_null_int(
                field_ident,
                attrs,
                dflt_lang,
                processing,
                type_mismatch_err,
                too_large_err,
                too_small_err,
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
            &not_matching_indicator_ident,
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
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: {}, found: null", I::expc());

                quote! {String::from(#msg)}
            },
        )?;

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        );

        let ty = I::ty();

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name: #ty = 0;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }

    #[inline]
    fn dflt_not_null_int<I: Ints>(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,

        dflt_lang: &str,
        processing: types::ints::Processing,
        type_mismatch_err: custom_types::TypeMismatchErr,
        too_large_err: types::ints::TooLargeErr,
        too_small_err: types::ints::TooSmallErr,
        dflt: I,
    ) -> Result<(), TokenStream> {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name = val;};

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                let msg = format!("expected: {}, found: null", I::expc());

                quote! {String::from(#msg)}
            },
        )?;

        let field_parsing_arm = gen_int_field_parsing_arm::<I>(
            byte_quoted_field_ident,
            before,
            processing,
            var_assignment,
            null_handling,
            type_mismatch_err,
            too_large_err,
            too_small_err,
        );

        let ty = I::ty();

        self.add_field_var_def(quote! {
            let mut #var_name: #ty = #dflt;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }
}

#[inline]
fn gen_int_field_parsing_arm<I: Ints>(
    quoted_field_ident: TokenStream2,
    before: TokenStream2,
    processing: types::ints::Processing,
    var_assignment: TokenStream2,
    null_handling: custom_types::NullHandling,
    type_mismatch_err: custom_types::TypeMismatchErr,
    too_large_err: types::ints::TooLargeErr,
    too_small_err: types::ints::TooSmallErr,
) -> custom_types::FieldParsingArm {
    custom_types::FieldParsingArm {
        none: int_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            I::ty(),
            processing.valdg,
            &var_assignment,
            null_handling.none,
            type_mismatch_err.none,
            too_large_err.none,
            too_small_err.none,
        ),

        lang: int_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            I::ty(),
            processing.valdg_lang,
            &var_assignment,
            null_handling.lang,
            type_mismatch_err.lang,
            too_large_err.lang,
            too_small_err.lang,
        ),

        stack_errs: int_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            I::ty(),
            processing.valdg_stack_errs,
            &var_assignment,
            null_handling.stack_errs,
            type_mismatch_err.stack_errs,
            too_large_err.stack_errs,
            too_small_err.stack_errs,
        ),

        stack_errs_lang: int_field_parsing_arm_temp(
            &quoted_field_ident,
            &before,
            I::ty(),
            processing.valdg_stack_errs_lang,
            &var_assignment,
            null_handling.stack_errs_lang,
            type_mismatch_err.stack_errs_lang,
            too_large_err.stack_errs_lang,
            too_small_err.stack_errs_lang,
        ),
    }
}

#[inline]
fn int_field_parsing_arm_temp(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    parser_module: TokenStream2,
    valdg: TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: TokenStream2,
    type_mismatch_err: TokenStream2,
    too_large_err: TokenStream2,
    too_small_err: TokenStream2,
) -> TokenStream2 {
    quote! {#quoted_field_ident => {
        #before
        match ::from::json::#parser_module::parse(json, idx) {
            ::from::json::#parser_module::ParseResult::Ok(val) => {
                #valdg
                #var_assignment
            }

            ::from::json::#parser_module::ParseResult::Null => {
                #null_handling
            }


            ::from::json::#parser_module::ParseResult::TypeMismatch(found) => {
                #type_mismatch_err
            }

            ::from::json::#parser_module::ParseResult::TooLargeToFitInto(typ) => {
                #too_large_err
            }

            ::from::json::#parser_module::ParseResult::TooSmallToFitInto(typ) => {
                #too_small_err
            }

            ::from::json::#parser_module::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
        }
    }}
}
