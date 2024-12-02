use proc_macro::TokenStream;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, Type as SynType, TypePath,
};

use crate::utils::compile_err;

#[inline]
pub fn type_path_or_err(ty: &SynType) -> Result<&TypePath, TokenStream> {
    if let SynType::Path(tp) = ty {
        if tp.qself.is_some() {
            return Err(compile_err(&tp, format!("qualified path is not supported")));
        };
        return Ok(tp);
    };

    Err(compile_err(&ty, "expected identifier"))
}

#[inline]
pub fn angle_bracketed_or_err(
    args: &PathArguments,
) -> Result<&AngleBracketedGenericArguments, TokenStream> {
    match args {
        PathArguments::AngleBracketed(args) => Ok(args),

        _ => Err(compile_err(
            &args,
            "only angle bracketed generic arguments is supported",
        )),
    }
}

#[inline]
pub fn type_or_err(arg: &GenericArgument) -> Result<&SynType, TokenStream> {
    if let GenericArgument::Type(ty) = arg {
        return Ok(&ty);
    };

    Err(compile_err(&arg, "only generic type argument is supported"))
}
