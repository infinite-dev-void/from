/* use proc_macro::TokenStream;

use syn::{
    punctuated::Punctuated, AngleBracketedGenericArguments, GenericArgument, PathSegment, Token,
};

use crate::utils::compile_err;

use super::{angle_bracketed_or_err, type_or_err, type_path_or_err};

pub fn stringify_sgs(sgs: &Punctuated<PathSegment, Token![::]>) -> Result<String, TokenStream> {
    // impossible case
    // if sgs.len() == 0 {};
    let mut s = String::new();

    for i in 0..sgs.len() - 1 {
        s.push_str(&stringify_sg(&sgs[i])?);
        s.push_str("::");
    }

    s.push_str(&stringify_sg(&sgs[sgs.len() - 1])?);
    Ok(s)
}

#[inline]
pub fn stringify_sg(sg: &PathSegment) -> Result<String, TokenStream> {
    let mut s = sg.ident.to_string();

    if sg.arguments.is_none() {
        return Ok(s);
    };

    let args = angle_bracketed_or_err(&sg.arguments)?;

    s.push_str(&stringify_args(args)?);

    Ok(s)
}

pub fn stringify_args(args: &AngleBracketedGenericArguments) -> Result<String, TokenStream> {
    if args.args.len() == 0 {
        return Err(compile_err(&args, "invalid generic arguments"));
    };

    let mut s = String::from('<');

    for i in 0..args.args.len() - 1 {
        s.push_str(&stringify_arg(&args.args[i])?);
        s.push(',');
    }

    s.push_str(&stringify_arg(&args.args[args.args.len() - 1])?);

    s.push('>');

    Ok(s)
}

#[inline]
pub fn stringify_arg(arg: &GenericArgument) -> Result<String, TokenStream> {
    let path = &type_path_or_err(type_or_err(arg)?)?.path;

    let mut s = String::new();

    if path.leading_colon.is_some() {
        s.push_str("::");
    };

    s.push_str(&stringify_sgs(&path.segments)?);

    Ok(s)
}
 */
