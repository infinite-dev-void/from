use proc_macro2::TokenStream as TokenStream2;

use crate::{custom, types};

use super::MissingFieldCheck;
pub struct StringParsingArgs {
    pub field_name_str_ts: TokenStream2,
    pub var_assignment: TokenStream2,
    pub processing: types::string::Processing,
    pub null_handling: custom::NullHandling,
    pub type_mismatch_err: custom::TypeMismatchErr,
}

pub struct IntParsingArgs<I: crate::types::ints::Ints> {
    field_name_str_ts: TokenStream2,
    var_assignment: TokenStream2,
    parser_module: TokenStream2,
    processing: types::ints::Processing<I>,
    null_handling: custom::NullHandling,
    type_mismatch_err: custom::TypeMismatchErr,
    too_large_err: types::ints::TooLargeErr,
    too_small_err: types::ints::TooSmallErr,
}

pub struct Impls<V, PS, PI, PF, PB, PC, PV, C, A>
where
    V: Fn(TokenStream2),       // V -> Var
    PS: Fn(StringParsingArgs), // PS -> Parsing String
    PI: Fn(),                  // PI -> Parsing Integer
    PF: Fn(),                  // PF -> Parsing Float
    PB: Fn(),                  // PF -> Parsing Boolean
    PC: Fn(),                  // PC -> Parsing Custom
    PV: Fn(),                  // PV -> Parsing Vector
    C: Fn(MissingFieldCheck),  // C -> Checks
    A: Fn(TokenStream2),       // A -> Assignment
{
    pub add_field_var: V,
    pub add_string_field_parsing_arm: PS,
    pub add_int_field_parsing_arm: PI,
    pub add_float_field_parsing_arm: PF,
    pub add_bool_field_parsing_arm: PB,
    pub add_custom_field_parsing_arm: PC,
    pub add_vec_field_parsing_arm: PV,
    pub add_missing_field_check: C,
    pub add_field_assignment: A,
}
