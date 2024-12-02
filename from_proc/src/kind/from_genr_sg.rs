use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{PathSegment, Type as SynType};

use super::{angle_bracketed_or_err, compile_err, type_or_err, Type};

impl super::Kind {
    pub(super) fn from_genr_sg(sg: &PathSegment) -> Result<Self, TokenStream> {
        let args = angle_bracketed_or_err(&sg.arguments)?;

        if args.args.len() != 1 {
            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Custom(sg.to_token_stream()),
            });
        };

        let ty = type_or_err(&args.args[0])?;

        if sg.ident.eq("Vec") {
            let kind = Self::from_ty(ty)?;

            if kind.option {
                if kind.null {
                    return Err(compile_err(
                        &ty,
                        "cannot use `OptionNull` enum as a generic parameter of the Vec type",
                    ));
                };
                return Err(compile_err(
                    &ty,
                    "cannot use `Option` enum as a generic parameter of the Vec type",
                ));
            };

            return Ok(Self {
                option: false,
                null: false,
                ty: Type::Vec(quote! {#sg}, Box::from(kind)),
            });
        };

        if sg.ident.eq("Option") {
            return Self::option_from_syn_ty(ty);
        };

        if sg.ident.eq("Null") {
            return Self::null_from_syn_ty(ty);
        };

        if sg.ident.eq("OptionNull") {
            return Self::option_null_from_syn_ty(ty);
        };

        /* let mut cus = sg.ident.to_string();
        let path = &type_path_or_err(ty)?.path;

        cus.push('<');
        if path.leading_colon.is_some() {
            cus.push_str("::");
        };

        cus.push_str(&stringify_sgs(&path.segments)?);
        cus.push('>'); */

        Ok(Self {
            option: false,
            null: false,
            ty: Type::Custom(sg.to_token_stream()),
        })
    }

    pub(super) fn option_from_syn_ty(ty: &SynType) -> Result<Self, TokenStream> {
        let of = Self::from_ty(ty)?;

        if of.null && of.option {
            return Err(compile_err(
                &ty,
                "cannot use the OptionNull type as a generic parameter of the Option type",
            ));
        };

        if of.null {
            return Err(compile_err(
                &ty,
                "cannot use the Null type as a generic parameter of the Option type",
            ));
        };

        if of.option {
            return Err(compile_err(
                &ty,
                "cannot use the Option type as a generic parameter of the Option type",
            ));
        };

        Ok(Self {
            option: true,
            null: false,
            ty: of.ty,
        })
    }

    pub(super) fn null_from_syn_ty(ty: &SynType) -> Result<Self, TokenStream> {
        let of = Self::from_ty(ty)?;

        if of.null && of.option {
            return Err(compile_err(
                &ty,
                "cannot use the OptionNull type as a generic parameter of the Null type",
            ));
        };

        if of.null {
            return Err(compile_err(
                &ty,
                "cannot use the Null type as a generic parameter of the Null type",
            ));
        };

        if of.option {
            return Err(compile_err(
                &ty,
                "cannot use the Option type as a generic parameter of the Null type",
            ));
        };

        Ok(Self {
            option: false,
            null: true,
            ty: of.ty,
        })
    }

    pub(super) fn option_null_from_syn_ty(ty: &SynType) -> Result<Self, TokenStream> {
        let of = Self::from_ty(ty)?;

        if of.null && of.option {
            return Err(compile_err(
                &ty,
                "cannot use the OptionNull type as a generic parameter of the OptionNull type",
            ));
        };

        if of.null {
            return Err(compile_err(
                &ty,
                "cannot use the Null type as a generic parameter of the OptionNull type",
            ));
        };

        if of.option {
            return Err(compile_err(
                &ty,
                "cannot use the Option type as a generic parameter of the OptionNull type",
            ));
        };

        Ok(Self {
            option: true,
            null: true,
            ty: of.ty,
        })
    }
}
