use proc_macro::TokenStream;
use syn::Path as SynPath;

use super::{compile_err, Type};

impl super::Kind {
    pub(super) fn from_sg(path: &SynPath) -> Result<Self, TokenStream> {
        if path.leading_colon.is_some() {
            return Err(compile_err(&path, "invalid type"));
        };

        let sg = &path.segments[0];

        if !sg.arguments.is_none() {
            return Self::from_genr_sg(sg);
        };

        Ok(Self {
            option: false,
            null: false,
            ty: Type::from_ident(&sg.ident),
        })
    }
}
