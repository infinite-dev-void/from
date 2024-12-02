use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::VariableParseSegments;

pub struct FromJsonValue {
    var_segs: VariableParseSegments,
}

impl FromJsonValue {
    #[inline]
    pub fn new() -> Self {
        Self {
            var_segs: VariableParseSegments::new(),
        }
    }

    #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var: &TokenStream2) {
        self.var_segs.add_field_var_def_ref(field_var);
    }

    /* #[inline]
    pub fn add_field_var_def(&mut self, field_var: TokenStream2) {
        self.var_segs.add_field_var_def(field_var);
    }

    #[inline]
    pub fn add_field_parsing_arm_ref(&mut self, field_parsing_arm: &TokenStream2) {
        self.var_segs.add_field_parsing_arm_ref(field_parsing_arm);
    } */

    #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: TokenStream2) {
        self.var_segs.add_field_parsing_arm(field_parsing_arm);
    }

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &TokenStream2) {
        self.var_segs
            .add_missing_field_check_ref(missing_field_check);
    } */

    #[inline]
    pub fn add_missing_field_check(&mut self, missing_field_check: TokenStream2) {
        self.var_segs.add_missing_field_check(missing_field_check);
    }

    #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.var_segs.add_field_assignment_ref(field_assignment);
    }

    /* #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.var_segs.add_field_assignment(field_assignment);
    } */

    #[inline]
    pub fn into_token_stream2(self) -> TokenStream2 {
        let VariableParseSegments {
            field_var_defs,
            field_parsing_arms,
            missing_field_checks,
            field_assignments,
        } = self.var_segs;

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
}

pub struct FromJsonValueLang {
    var_segs: VariableParseSegments,
}

impl FromJsonValueLang {
    #[inline]
    pub fn new() -> Self {
        Self {
            var_segs: VariableParseSegments::new(),
        }
    }

    /* #[inline]
    pub fn add_field_var_def(&mut self, field_var: TokenStream2) {
        self.var_segs.add_field_var_def(field_var);
    } */

    #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var: &TokenStream2) {
        self.var_segs.add_field_var_def_ref(field_var);
    }

    #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: TokenStream2) {
        self.var_segs.add_field_parsing_arm(field_parsing_arm);
    }

    /* #[inline]
    pub fn add_field_parsing_arm_ref(&mut self, field_parsing_arm: &TokenStream2) {
        self.var_segs.add_field_parsing_arm_ref(field_parsing_arm);
    } */

    #[inline]
    pub fn add_missing_field_check(&mut self, missing_field_check: TokenStream2) {
        self.var_segs.add_missing_field_check(missing_field_check);
    }

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &TokenStream2) {
        self.var_segs
            .add_missing_field_check_ref(missing_field_check);
    }

    #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.var_segs.add_field_assignment(field_assignment);
    } */

    #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.var_segs.add_field_assignment_ref(field_assignment);
    }

    #[inline]
    pub fn into_token_stream2(self) -> TokenStream2 {
        let VariableParseSegments {
            field_var_defs,
            field_parsing_arms,
            missing_field_checks,
            field_assignments,
        } = self.var_segs;

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
}

pub struct FromJsonValueStackErrs {
    var_segs: VariableParseSegments,
}

impl FromJsonValueStackErrs {
    #[inline]
    pub fn new() -> Self {
        Self {
            var_segs: VariableParseSegments::new(),
        }
    }

    #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var: &TokenStream2) {
        self.var_segs.add_field_var_def_ref(field_var);
    }

    /* #[inline]
    pub fn add_field_var_def(&mut self, field_var: TokenStream2) {
        self.var_segs.add_field_var_def(field_var);
    }

    #[inline]
    pub fn add_field_parsing_arm_ref(&mut self, field_parsing_arm: &TokenStream2) {
        self.var_segs.add_field_parsing_arm_ref(field_parsing_arm);
    } */

    #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: TokenStream2) {
        self.var_segs.add_field_parsing_arm(field_parsing_arm);
    }

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &TokenStream2) {
        self.var_segs
            .add_missing_field_check_ref(missing_field_check);
    } */

    #[inline]
    pub fn add_missing_field_check(&mut self, missing_field_check: TokenStream2) {
        self.var_segs.add_missing_field_check(missing_field_check);
    }

    #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.var_segs.add_field_assignment_ref(field_assignment);
    }

    /* #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.var_segs.add_field_assignment(field_assignment);
    } */

    #[inline]
    pub fn into_token_stream2(self) -> TokenStream2 {
        let VariableParseSegments {
            field_var_defs,
            field_parsing_arms,
            missing_field_checks,
            field_assignments,
        } = self.var_segs;

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
}

pub struct FromJsonValueStackErrsLang {
    var_segs: VariableParseSegments,
}

impl FromJsonValueStackErrsLang {
    #[inline]
    pub fn new() -> Self {
        Self {
            var_segs: VariableParseSegments::new(),
        }
    }

    /* #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var: &TokenStream2) {
        self.var_segs.add_field_var_def_ref(field_var);
    } */

    #[inline]
    pub fn add_field_var_def(&mut self, field_var: TokenStream2) {
        self.var_segs.add_field_var_def(field_var);
    }

    /* #[inline]
    pub fn add_field_parsing_arm_ref(&mut self, field_parsing_arm: &TokenStream2) {
        self.var_segs.add_field_parsing_arm_ref(field_parsing_arm);
    } */

    #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: TokenStream2) {
        self.var_segs.add_field_parsing_arm(field_parsing_arm);
    }

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &TokenStream2) {
        self.var_segs
            .add_missing_field_check_ref(missing_field_check);
    } */

    #[inline]
    pub fn add_missing_field_check(&mut self, missing_field_check: TokenStream2) {
        self.var_segs.add_missing_field_check(missing_field_check);
    }

    /* #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.var_segs.add_field_assignment_ref(field_assignment);
    } */

    #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.var_segs.add_field_assignment(field_assignment);
    }

    #[inline]
    pub fn into_token_stream2(self) -> TokenStream2 {
        let VariableParseSegments {
            field_var_defs,
            field_parsing_arms,
            missing_field_checks,
            field_assignments,
        } = self.var_segs;

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
}
