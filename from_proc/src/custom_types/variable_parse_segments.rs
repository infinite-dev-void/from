use proc_macro2::TokenStream as TokenStream2;

use super::Tokens;
pub struct VariableParseSegments {
    pub required_field_errs: Tokens,
    pub field_var_defs: TokenStream2,
    pub field_parsing_arms: TokenStream2,
    pub missing_field_checks: TokenStream2,
    pub field_assignments: TokenStream2,
}

impl VariableParseSegments {
    #[inline]
    pub fn new() -> Self {
        Self {
            required_field_errs: Tokens::new(),
            field_var_defs: TokenStream2::new(),
            field_parsing_arms: TokenStream2::new(),
            missing_field_checks: TokenStream2::new(),
            field_assignments: TokenStream2::new(),
        }
    }

    #[inline]
    pub fn add_required_field_err(&mut self, required_field_err: TokenStream2) {
        self.required_field_errs.push(required_field_err);
    }

    #[inline]
    pub fn add_field_var_def_ref(&mut self, field_var: &TokenStream2) {
        self.field_var_defs
            .extend(::core::iter::once(field_var.clone()));
    }

    #[inline]
    pub fn add_field_var_def(&mut self, field_var: TokenStream2) {
        self.field_var_defs.extend(::core::iter::once(field_var));
    }

    /* #[inline]
    pub fn add_field_parsing_arm_ref(&mut self, field_parsing_arm: &TokenStream2) {
        self.field_parsing_arms
            .extend(::core::iter::once(field_parsing_arm.clone()))
    } */

    #[inline]
    pub fn add_field_parsing_arm(&mut self, field_parsing_arm: TokenStream2) {
        self.field_parsing_arms
            .extend(::core::iter::once(field_parsing_arm))
    }

    /* #[inline]
    pub fn add_missing_field_check_ref(&mut self, missing_field_check: &TokenStream2) {
        self.missing_field_checks
            .extend(::core::iter::once(missing_field_check.clone()));
    } */

    #[inline]
    pub fn add_missing_field_check(&mut self, missing_field_check: TokenStream2) {
        self.missing_field_checks
            .extend(::core::iter::once(missing_field_check));
    }

    #[inline]
    pub fn add_field_assignment_ref(&mut self, field_assignment: &TokenStream2) {
        self.field_assignments
            .extend(::core::iter::once(field_assignment.clone()))
    }

    #[inline]
    pub fn add_field_assignment(&mut self, field_assignment: TokenStream2) {
        self.field_assignments
            .extend(::core::iter::once(field_assignment))
    }
}
