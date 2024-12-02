use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Path};

use crate::{custom_types, metas_holder::MetasHolder, types, Null};

use super::FromJsonValueImpl;

type Processing = types::custom::Processing;

pub struct MethodCall {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl MethodCall {
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

pub struct ErrHandling {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}

impl ErrHandling {
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

impl FromJsonValueImpl {
    pub fn add_custom_field(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        ty: TokenStream2,
        dflt_lang: &str,
        option: bool,
        null: bool,
    ) -> Result<(), TokenStream> {
        let processing = Processing::try_build(&attrs, &ty)?;

        let method_call = MethodCall::new(&ty);
        let err_handling = ErrHandling::new();

        match (option, null) {
            (true, true) => {
                Ok(self.option_null_custom(field_ident, ty, processing, method_call, err_handling))
            }

            (true, false) => self.option_not_null_custom(
                field_ident,
                attrs,
                ty,
                dflt_lang,
                processing,
                method_call,
                err_handling,
            ),

            (false, true) => self.rqd_null_custom(
                field_ident,
                attrs,
                ty,
                dflt_lang,
                processing,
                method_call,
                err_handling,
            ),

            (false, false) => self.rqd_not_null_custom(
                field_ident,
                attrs,
                ty,
                dflt_lang,
                processing,
                method_call,
                err_handling,
            ),
        }
    }

    #[inline]
    fn option_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        ty: TokenStream2,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let before = TokenStream2::new();
        let var_assignment = quote! {#var_name = ::from::OptionNull::<#ty>::Some(val);};

        let null_handling = custom_types::NullHandling::from_one(
            quote! {#var_name = ::from::OptionNull::<#ty>::Null;},
        );

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
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
    fn option_not_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
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
                quote! {String::from("expected: object, found: null")}
            },
        )?;

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
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
    fn rqd_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
    ) -> Result<(), TokenStream> {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<Null<Path>>("default")? {
            return Ok(self.dflt_null_custom(
                field_ident,
                ty,
                processing,
                method_call,
                err_handling,
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

        let before = quote! {#not_matching_indicator_ident = false;};

        let var_assignment = quote! {#var_name = ::from::Null::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::<#ty>::Null;});

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
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
    fn dflt_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        ty: TokenStream2,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
        dflt: Null<Path>,
    ) {
        let custom_types::FieldIdent {
            var_name,
            ident: field_ident,
            quoted: quoted_field_ident,
            byte_quoted: byte_quoted_field_ident,
        } = field_ident;

        let field_var_def = match dflt {
            Null::Some(p) => {
                quote! {
                    let mut #var_name = #p(),
                }
            }
            Null::Null => {
                quote! {
                    let mut #var_name = ::from::Null::<#ty>::Null;
                }
            }
        };

        let before = TokenStream2::new();

        let var_assignment = quote! {#var_name =::from::Null::Some(val);};

        let null_handling =
            custom_types::NullHandling::from_one(quote! {#var_name = ::from::Null::<#ty>::Null;});

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
        );

        self.add_field_var_def(field_var_def);

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });
    }

    #[inline]
    fn rqd_not_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        ty: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
    ) -> Result<(), TokenStream> {
        if let Some((dflt, _)) = attrs.parse_value_if_found::<Path>("default")? {
            return self.dflt_not_null_custom(
                field_ident,
                attrs,
                ty,
                dflt_lang,
                processing,
                method_call,
                err_handling,
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
        let var_assignment = quote! {#var_name = Some(val);};

        let null_handling = custom_types::NullHandling::try_build(
            &attrs,
            &quoted_field_ident,
            &dflt_lang,
            || -> TokenStream2 {
                quote! {String::from("expected: object, found: null")}
            },
        )?;

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
        );

        self.add_field_var_def(quote! {
            let mut #not_matching_indicator_ident = true;
            let mut #var_name = Option::<#ty>::None;
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_missing_field_check(missing_field_check);

        self.add_field_assignment(quote! {
            #field_ident: unsafe { #var_name.unwrap_unchecked() },
        });

        Ok(())
    }

    #[inline]
    fn dflt_not_null_custom(
        &mut self,
        field_ident: custom_types::FieldIdent,
        attrs: Vec<Attribute>,
        _: TokenStream2,
        dflt_lang: &str,
        processing: Processing,
        method_call: MethodCall,
        err_handling: ErrHandling,
        dflt: Path,
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
                quote! {String::from("expected: object, found: null")}
            },
        )?;

        let field_parsing_arm = gen_custom_field_parsing_arm(
            byte_quoted_field_ident,
            quoted_field_ident,
            before,
            null_handling,
            method_call,
            processing,
            var_assignment,
            err_handling,
        );

        self.add_field_var_def(quote! {
            let mut #var_name = #dflt();
        });

        self.add_field_parsing_arm(field_parsing_arm);

        self.add_field_assignment(quote! {
            #field_ident: #var_name,
        });

        Ok(())
    }
}

#[inline]
fn gen_custom_field_parsing_arm(
    byte_quoted_field_ident: TokenStream2,
    quoted_field_ident: TokenStream2,
    before: TokenStream2,
    null_handling: custom_types::NullHandling,
    method_call: MethodCall,
    processing: Processing,
    var_assignment: TokenStream2,
    err_handling: ErrHandling,
) -> custom_types::FieldParsingArm {
    custom_types::FieldParsingArm {
        none: custom_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            null_handling.none,
            method_call.none,
            processing.valdg,
            &var_assignment,
            err_handling.none,
        ),

        lang: custom_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            null_handling.lang,
            method_call.lang,
            processing.valdg_lang,
            &var_assignment,
            err_handling.lang,
        ),

        stack_errs: custom_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            null_handling.stack_errs,
            method_call.stack_errs,
            processing.valdg_stack_errs,
            &var_assignment,
            err_handling.stack_errs,
        ),

        stack_errs_lang: custom_field_parsing_arm_temp(
            &byte_quoted_field_ident,
            &quoted_field_ident,
            &before,
            null_handling.stack_errs_lang,
            method_call.stack_errs_lang,
            processing.valdg_stack_errs_lang,
            &var_assignment,
            err_handling.stack_errs_lang,
        ),
    }
}

#[inline]
fn custom_field_parsing_arm_temp(
    byte_quoted_field_ident: &TokenStream2,
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    null_handling: TokenStream2,
    method_call: TokenStream2,
    valdg: TokenStream2,
    var_assignment: &TokenStream2,
    err_handling: TokenStream2,
) -> TokenStream2 {
    quote! {
        #byte_quoted_field_ident => {
            byte = ::from::json::utils::get_or_unexpected_end(json, idx)?;
            #before
            if byte == b'n' {
                if let Err(e) = ::from::json::utils::skip_null(json, idx, "{") {
                    return Err(From::from(e));
                };

                #null_handling
            } else {
                let mut path = path.clone();
                path.push(From::from(#quoted_field_ident));

                match #method_call {
                    Ok(val) => {
                        #valdg
                        #var_assignment
                    },

                    Err(e) => {
                        #err_handling
                    }
                };
            };

        }
    }
}
