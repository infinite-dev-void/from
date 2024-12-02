use proc_macro2::TokenStream as TokenStream2;

pub struct FieldParsingArm {
    pub none: TokenStream2,
    pub lang: TokenStream2,
    pub stack_errs: TokenStream2,
    pub stack_errs_lang: TokenStream2,
}
