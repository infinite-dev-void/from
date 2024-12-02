pub struct FromJsonValueImpl {
    from_json_value: methods::FromJsonValue,
    from_json_value_lang: methods::FromJsonValueLang,
    from_json_value_stack_errs: methods::FromJsonValueStackErrs,
    from_json_value_stack_errs_lang: methods::FromJsonValueStackErrsLang,
}

impl FromJsonValueImpl {
    #[inline]
    pub fn new() -> Self {
        Self {
            from_json_value: methods::FromJsonValue::new(),
            from_json_value_lang: methods::FromJsonValueLang::new(),
            from_json_value_stack_errs: methods::FromJsonValueStackErrs::new(),
            from_json_value_stack_errs_lang: methods::FromJsonValueStackErrsLang::new(),
        }
    }

    #[inline]
    pub fn add_field_var_def(&mut self, field_var_def: TokenStream2) {
        self.from_json_value.add_field_var_def_ref(&field_var_def);
        self.from_json_value_lang
            .add_field_var_def_ref(&field_var_def);
        self.from_json_value_stack_errs
            .add_field_var_def_ref(&field_var_def);
        self.from_json_value_stack_errs_lang
            .add_field_var_def(field_var_def);
    }

    /* #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var_def: &TokenStream2) {
        self.from_json_value.add_field_var_def_ref(field_var_def);
        self.from_json_value_lang
            .add_field_var_def_ref(field_var_def);
        self.from_json_value_stack_errs
            .add_field_var_def_ref(field_var_def);
        self.from_json_value_stack_errs_lang
            .add_field_var_def_ref(field_var_def);
    } */

    /* #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: &TokenStream2) {
        self.from_json_value
            .add_field_parsing_arm(&field_parsing_arm);
        self.from_json_value_lang
            .add_field_parsing_arm(&field_parsing_arm);
        self.from_json_value_stack_errs
            .add_field_parsing_arm(&field_parsing_arm);
        self.from_json_value_stack_errs_lang
            .add_field_parsing_arm(&field_parsing_arm);
    } */

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &custom::MissingFieldCheck) {
        self.from_json_value
            .add_missing_field_check_ref(&missing_field_check.none);
        self.from_json_value_lang
            .add_missing_field_check_ref(&missing_field_check.lang);
        self.from_json_value_stack_errs
            .add_missing_field_check_ref(&missing_field_check.stack_errs);
        self.from_json_value_stack_errs_lang
            .add_missing_field_check_ref(&missing_field_check.stack_errs_lang);
    } */

    #[inline]
    pub fn add_missing_field_check(
        &mut self,
        missing_field_check: custom_types::MissingFieldCheck,
    ) {
        self.from_json_value
            .add_missing_field_check(missing_field_check.none);
        self.from_json_value_lang
            .add_missing_field_check(missing_field_check.lang);
        self.from_json_value_stack_errs
            .add_missing_field_check(missing_field_check.stack_errs);
        self.from_json_value_stack_errs_lang
            .add_missing_field_check(missing_field_check.stack_errs_lang);
    }

    /* #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.from_json_value
            .add_field_assignment_ref(field_assignment);
        self.from_json_value_lang
            .add_field_assignment_ref(field_assignment);
        self.from_json_value_stack_errs
            .add_field_assignment_ref(field_assignment);
        self.from_json_value_stack_errs_lang
            .add_field_assignment_ref(field_assignment);
    } */

    #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.from_json_value
            .add_field_assignment_ref(&field_assignment);
        self.from_json_value_lang
            .add_field_assignment_ref(&field_assignment);
        self.from_json_value_stack_errs
            .add_field_assignment_ref(&field_assignment);
        self.from_json_value_stack_errs_lang
            .add_field_assignment(field_assignment);
    }

    #[inline]
    pub fn impl_for(self, ident: &Ident, generics: &Generics) -> TokenStream2 {
        let from_json_value = self.from_json_value.into_token_stream2();

        let from_json_value_lang = self.from_json_value_lang.into_token_stream2();

        let from_json_value_stack_errs = self.from_json_value_stack_errs.into_token_stream2();

        let from_json_value_stack_errs_lang =
            self.from_json_value_stack_errs_lang.into_token_stream2();

        quote! {
            impl #generics ::from::json::FromJsonValue for #ident #generics {
                #from_json_value

                #from_json_value_lang

                #from_json_value_stack_errs

                #from_json_value_stack_errs_lang
            }
        }
    }
}

// string
impl FromJsonValueImpl {
    /*  pub fn add_string_field_parsing_arm_ref(
            &mut self,
            parsing_params: types::string::JsonParsingArmParams,
        ) {
            self.from_json_value
                .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                    &parsing_params.quoted_field_ident,
                    &parsing_params.before,
                    &parsing_params.processing.mutable,
                    &parsing_params.processing.sanitizing,
                    &parsing_params.processing.valdg,
                    &parsing_params.var_assignment,
                    &parsing_params.null_handling.none,
                    &parsing_params.type_mismatch_err.none,
                ));

            self.from_json_value_lang
                .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                    &parsing_params.quoted_field_ident,
                    &parsing_params.before,
                    &parsing_params.processing.mutable,
                    &parsing_params.processing.sanitizing,
                    &parsing_params.processing.valdg_lang,
                    &parsing_params.var_assignment,
                    &parsing_params.null_handling.lang,
                    &parsing_params.type_mismatch_err.lang,
                ));

            self.from_json_value_stack_errs
                .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                    &parsing_params.quoted_field_ident,
                    &parsing_params.before,
                    &parsing_params.processing.mutable,
                    &parsing_params.processing.sanitizing,
                    &parsing_params.processing.valdg_stack_errs,
                    &parsing_params.var_assignment,
                    &parsing_params.null_handling.stack_errs,
                    &parsing_params.type_mismatch_err.stack_errs,
                ));

            self.from_json_value_stack_errs_lang.add_field_parsing_arm(
                string_field_parsing_arm_temp_ref(
                    &parsing_params.quoted_field_ident,
                    &parsing_params.before,
                    &parsing_params.processing.mutable,
                    &parsing_params.processing.sanitizing,
                    &parsing_params.processing.valdg_stack_errs_lang,
                    &parsing_params.var_assignment,
                    &parsing_params.null_handling.stack_errs_lang,
                    &parsing_params.type_mismatch_err.stack_errs_lang,
                ),
            );
        }
    */
    pub fn add_string_field_parsing_arm(
        &mut self,
        parsing_params: types::string::JsonParsingArmParams,
    ) {
        self.from_json_value
            .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.mutable,
                &parsing_params.processing.sanitizing,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.none,
                &parsing_params.type_mismatch_err.none,
            ));

        self.from_json_value_lang
            .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.mutable,
                &parsing_params.processing.sanitizing,
                &parsing_params.processing.valdg_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.lang,
                &parsing_params.type_mismatch_err.lang,
            ));

        self.from_json_value_stack_errs
            .add_field_parsing_arm(string_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.mutable,
                &parsing_params.processing.sanitizing,
                &parsing_params.processing.valdg_stack_errs,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs,
                &parsing_params.type_mismatch_err.stack_errs,
            ));

        // TODO: optimization: create another template that will consume
        // all TokenStream2 values
        self.from_json_value_stack_errs_lang.add_field_parsing_arm(
            string_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.mutable,
                &parsing_params.processing.sanitizing,
                &parsing_params.processing.valdg_stack_errs_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs_lang,
                &parsing_params.type_mismatch_err.stack_errs_lang,
            ),
        );
    }
}

#[inline]
fn string_field_parsing_arm_temp_ref(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    mutable: &TokenStream2,
    sanitizing: &TokenStream2,
    valdg: &TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: &TokenStream2,
    type_mismatch_err: &TokenStream2,
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

// ints
impl FromJsonValueImpl {
    pub fn add_int_field_parsing_arm<I: types::ints::Ints>(
        &mut self,
        parsing_params: types::ints::JsonParsingArmParams<I>,
    ) {
        self.from_json_value
            .add_field_parsing_arm(int_field_parsing_arm_temp(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.parser_module,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.none,
                &parsing_params.type_mismatch_err.none,
                &parsing_params.too_large_err.none,
                &parsing_params.too_small_err.none,
            ));

        self.from_json_value_lang
            .add_field_parsing_arm(int_field_parsing_arm_temp(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.parser_module,
                &parsing_params.processing.valdg_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.lang,
                &parsing_params.type_mismatch_err.lang,
                &parsing_params.too_large_err.lang,
                &parsing_params.too_small_err.lang,
            ));

        self.from_json_value_stack_errs
            .add_field_parsing_arm(int_field_parsing_arm_temp(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.parser_module,
                &parsing_params.processing.valdg_stack_errs,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs,
                &parsing_params.type_mismatch_err.stack_errs,
                &parsing_params.too_large_err.stack_errs,
                &parsing_params.too_small_err.stack_errs,
            ));

        // TODO: optimization: create another template that will consume
        // all TokenStream2 values
        self.from_json_value_stack_errs_lang
            .add_field_parsing_arm(int_field_parsing_arm_temp(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.parser_module,
                &parsing_params.processing.valdg_stack_errs_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs_lang,
                &parsing_params.type_mismatch_err.stack_errs_lang,
                &parsing_params.too_large_err.stack_errs_lang,
                &parsing_params.too_small_err.stack_errs_lang,
            ));
    }
}

#[inline]
fn int_field_parsing_arm_temp(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    parser_module: &TokenStream2,
    valdg: &TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: &TokenStream2,
    type_mismatch_err: &TokenStream2,
    too_large_err: &TokenStream2,
    too_small_err: &TokenStream2,
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

// bool
impl FromJsonValueImpl {
    pub fn add_bool_field_parsing_arm(
        &mut self,
        parsing_params: types::bool::JsonParsingArmParams,
    ) {
        self.from_json_value
            .add_field_parsing_arm(bool_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.none,
                &parsing_params.type_mismatch_err.none,
            ));

        self.from_json_value_lang
            .add_field_parsing_arm(bool_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.lang,
                &parsing_params.type_mismatch_err.lang,
            ));

        self.from_json_value_stack_errs
            .add_field_parsing_arm(bool_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_stack_errs,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs,
                &parsing_params.type_mismatch_err.stack_errs,
            ));

        // TODO: optimization: create another template that will consume
        // all TokenStream2 values
        self.from_json_value_stack_errs_lang.add_field_parsing_arm(
            bool_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_stack_errs_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs_lang,
                &parsing_params.type_mismatch_err.stack_errs_lang,
            ),
        );
    }
}

fn bool_field_parsing_arm_temp_ref(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    valdg: &TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: &TokenStream2,
    type_mismatch_err: &TokenStream2,
) -> TokenStream2 {
    quote! {
        #quoted_field_ident => {
            #before
            match ::from::json::bool::parse(json, idx) {
                ::from::json::bool::ParseResult::Ok(val) => {
                    #valdg
                    #var_assignment
                }

                ::from::json::bool::ParseResult::Null => {
                    #null_handling
                }


                ::from::json::bool::ParseResult::TypeMismatch(found) => {
                    #type_mismatch_err
                }

                ::from::json::bool::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
            }
        }
    }
}

// float
impl FromJsonValueImpl {
    pub fn add_float_field_parsing_arm<F>(
        &mut self,
        parsing_params: types::floats::JsonParsingArmParams,
    ) where
        F: types::floats::Floats,
    {
        let parser_module = F::ty();
        self.from_json_value
            .add_field_parsing_arm(float_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.none,
                &parsing_params.type_mismatch_err.none,
                &parser_module,
            ));

        self.from_json_value_lang
            .add_field_parsing_arm(float_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.lang,
                &parsing_params.type_mismatch_err.lang,
                &parser_module,
            ));

        self.from_json_value_stack_errs
            .add_field_parsing_arm(float_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_stack_errs,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs,
                &parsing_params.type_mismatch_err.stack_errs,
                &parser_module,
            ));

        // TODO: optimization: create another template that will consume
        // all TokenStream2 values
        self.from_json_value_stack_errs_lang.add_field_parsing_arm(
            float_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.processing.valdg_stack_errs_lang,
                &parsing_params.var_assignment,
                &parsing_params.null_handling.stack_errs_lang,
                &parsing_params.type_mismatch_err.stack_errs_lang,
                &parser_module,
            ),
        );
    }
}

fn float_field_parsing_arm_temp_ref(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    valdg: &TokenStream2,
    var_assignment: &TokenStream2,
    null_handling: &TokenStream2,
    type_mismatch_err: &TokenStream2,
    parser_module: &TokenStream2,
) -> TokenStream2 {
    quote! {
        #quoted_field_ident => {
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

                ::from::json::#parser_module::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
            }
        }
    }
}

// custom
impl FromJsonValueImpl {
    pub fn add_custom_field_parsing_arm(
        &mut self,
        parsing_params: types::custom::JsonParsingArmParams,
    ) {
        self.from_json_value
            .add_field_parsing_arm(custom_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.null_handling.none,
                &parsing_params.method_call.none,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.err_handling.none,
            ));

        self.from_json_value_lang
            .add_field_parsing_arm(custom_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.null_handling.lang,
                &parsing_params.method_call.lang,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.err_handling.lang,
            ));

        self.from_json_value_stack_errs
            .add_field_parsing_arm(custom_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.null_handling.stack_errs,
                &parsing_params.method_call.stack_errs,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.err_handling.stack_errs,
            ));

        // TODO: optimization: create another template that will consume
        // all TokenStream2 values
        self.from_json_value_stack_errs_lang.add_field_parsing_arm(
            custom_field_parsing_arm_temp_ref(
                &parsing_params.quoted_field_ident,
                &parsing_params.before,
                &parsing_params.null_handling.stack_errs_lang,
                &parsing_params.method_call.stack_errs_lang,
                &parsing_params.processing.valdg,
                &parsing_params.var_assignment,
                &parsing_params.err_handling.stack_errs_lang,
            ),
        );
    }
}

fn custom_field_parsing_arm_temp_ref(
    quoted_field_ident: &TokenStream2,
    before: &TokenStream2,
    null_handling: &TokenStream2,
    method_call: &TokenStream2,
    valdg: &TokenStream2,
    var_assignment: &TokenStream2,
    err_handling: &TokenStream2,
) -> TokenStream2 {
    quote! {
        #quoted_field_ident => {
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

/* match ::from::json::#parser_module::parse(json, idx) {
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

    ::from::json::#parser_module::ParseResult::SyntaxErr(e) => return Err(From::from(e)),
} */
