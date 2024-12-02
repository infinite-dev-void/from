use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Path as SynPath, PathArguments};

use super::{type_or_err, Type};

impl super::Kind {
    pub(super) fn from_sgs(path: &SynPath) -> Result<Self, TokenStream> {
        let sgs = &path.segments;

        if sgs.len() != 2 {
            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Custom(path.to_token_stream()),
            });
        };

        let mut sg = &sgs[0];

        if !sg.ident.eq("from") {
            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Custom(path.to_token_stream()),
            });
        };

        if !sg.arguments.is_none() {
            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Custom(path.to_token_stream()),
            });
        };

        sg = &sgs[1];

        let args = match &sg.arguments {
            PathArguments::AngleBracketed(args) => args,
            _ => {
                return Ok(Self {
                    option: false,
                    null: false,
                    ty: Type::Custom(path.to_token_stream()),
                })
            }
        };

        // TODO: handle args.colon2_token
        // I do not know if there is a case where args.colon2_token
        // is some

        if args.args.len() != 1 {
            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Custom(path.to_token_stream()),
            });
        };

        let arg_ty = type_or_err(&args.args[0])?;

        if sg.ident.eq("Null") {
            return Self::null_from_syn_ty(arg_ty);
        };

        if sg.ident.eq("OptionNull") {
            return Self::option_null_from_syn_ty(arg_ty);
        };

        /* let mut cus = String::new();

        if path.leading_colon.is_some() {
            cus.push_str("::");
        };

        cus.push_str("from::");
        cus.push_str(&sg.ident.to_string());
        cus.push_str(&stringify_args(args)?); */

        return Ok(Self {
            option: false,
            null: false,
            ty: Type::Custom(path.to_token_stream()),
        });
    }
}
