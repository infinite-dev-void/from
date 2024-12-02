use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, TokenStreamExt};
use syn::{Field, Ident};

pub struct FieldIdent {
    pub ident: Ident,
    pub quoted: TokenStream2,
    pub byte_quoted: TokenStream2,
    pub var_name: Ident,
}

impl FieldIdent {
    pub fn new(field: &Field) -> Self {
        // It has been verified that the structure's fields are named
        let ident = unsafe { field.ident.as_ref().unwrap_unchecked().clone() };

        Self {
            quoted: {
                let mut ts = TokenStream2::new();
                ts.append(Literal::string(&ident.to_string()));
                ts
            },
            byte_quoted: {
                let mut ts = TokenStream2::new();
                ts.append(Literal::byte_string(ident.to_string().as_bytes()));
                ts
            },
            var_name: format_ident!("__{}", ident),
            ident,
        }
    }
}
