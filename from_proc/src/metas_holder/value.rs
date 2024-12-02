use proc_macro::TokenStream;
use syn::{Expr, ExprLit, Lit, Path};

use crate::{utils, Null};

pub trait FromExpr: Sized {
    #[track_caller]
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream>;
}

/* #[track_caller]
pub fn value_from_metas<'a, T: FromExpr>(
    metas: &'a super::Metas,
    name: &'static str,
) -> Result<Option<(T, &'a MetaNameValue)>, TokenStream> {
    for meta in metas {
        let nv = match meta {
                Meta::NameValue(nv) => nv,
                _=> continue
            };

            let nv_name = match nv.path.get_ident() {
                Some(nv_name) => nv_name,
                _=> continue,
            };

        if nv_name.ne(name) {
            continue;
        };

        return Ok(Some((T::from_expr(&nv.value)?, nv)));
    }

    Ok(None)
} */

//
//
//
//
//
//

#[inline(always)]
fn lit_expr_or<'a>(expr: &'a Expr, err: &'static str) -> Result<&'a ExprLit, TokenStream> {
    match expr {
        Expr::Lit(l) => Ok(l),

        _ => Err(utils::compile_err(&expr, err)),
    }
}

macro_rules! imp_int {
    (
        typ = $typ: ident,
        err = $err: expr,
    ) => {
        impl FromExpr for $typ {
            fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
                const ERR: &str = $err;

                match &lit_expr_or(expr, ERR)?.lit {
                    Lit::Int(i) => match i.base10_digits().parse::<Self>() {
                        Ok(num) => Ok(num),

                        Err(e) => Err(utils::compile_err(expr, &e.to_string())),
                    },

                    _ => Err(utils::compile_err(&expr, ERR)),
                }
            }
        }
    };
}

imp_int!(typ = i8, err = "expected: i8",);
imp_int!(typ = i16, err = "expected: i16",);
imp_int!(typ = i32, err = "expected: i32",);
imp_int!(typ = i64, err = "expected: i64",);
imp_int!(typ = i128, err = "expected: i128",);
imp_int!(typ = isize, err = "expected: isize",);
imp_int!(typ = u8, err = "expected: u8",);
imp_int!(typ = u16, err = "expected: u16",);
imp_int!(typ = u32, err = "expected: u32",);
imp_int!(typ = u64, err = "expected: u64",);
imp_int!(typ = u128, err = "expected: u128",);
imp_int!(typ = usize, err = "expected: usize",);

//
//
//
//
//
//
//
//
//
//
//

macro_rules! imp_null_int {
    (
        typ = $typ: ident,
        err = $err: expr,
    ) => {
        impl FromExpr for Null<$typ> {
            fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
                const ERR: &str = $err;

                match expr {
                    //
                    //
                    // literal
                    Expr::Lit(l) => {
                        //
                        match &l.lit {
                            Lit::Int(i) => match i.base10_digits().parse::<$typ>() {
                                Ok(num) => Ok(Null::Some(num)),

                                Err(e) => Err(utils::compile_err(expr, &e.to_string())),
                            },

                            _ => Err(utils::compile_err(&expr, ERR)),
                        }
                    }

                    //
                    //
                    //
                    // path
                    Expr::Path(p) => {
                        //
                        match p.path.get_ident() {
                            Some(ident) => {
                                if ident.eq("null") {
                                    Ok(Null::Null)
                                } else {
                                    Err(utils::compile_err(expr, ERR))
                                }
                            }
                            None => Err(utils::compile_err(expr, ERR)),
                        }
                    }

                    //
                    //
                    //
                    _ => Err(utils::compile_err(expr, ERR)),
                }
            }
        }
    };
}

imp_null_int!(typ = i8, err = "expected: null or i8",);
imp_null_int!(typ = i16, err = "expected: null or i16",);
imp_null_int!(typ = i32, err = "expected: null or i32",);
imp_null_int!(typ = i64, err = "expected: null or i64",);
imp_null_int!(typ = i128, err = "expected: null or i128",);
imp_null_int!(typ = isize, err = "expected: null or isize",);
imp_null_int!(typ = u8, err = "expected: null or u8",);
imp_null_int!(typ = u16, err = "expected: null or u16",);
imp_null_int!(typ = u32, err = "expected: null or u32",);
imp_null_int!(typ = u64, err = "expected: null or u64",);
imp_null_int!(typ = u128, err = "expected: null or u128",);
imp_null_int!(typ = usize, err = "expected: null or usize",);

//
//
//
//
//
//
//
//
//

impl FromExpr for String {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: &str";
        match &lit_expr_or(expr, ERR)?.lit {
            Lit::Str(s) => Ok(s.value()),

            _ => Err(utils::compile_err(&expr, ERR)),
        }
    }
}

impl FromExpr for Null<String> {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: null or &str";

        match expr {
            //
            //
            // literal
            Expr::Lit(l) => {
                //
                match &l.lit {
                    Lit::Str(s) => Ok(Null::Some(s.value())),

                    _ => Err(utils::compile_err(&expr, ERR)),
                }
            }

            //
            //
            //
            // path
            Expr::Path(p) => {
                //
                match p.path.get_ident() {
                    Some(ident) => {
                        if ident.eq("null") {
                            Ok(Null::Null)
                        } else {
                            Err(utils::compile_err(expr, ERR))
                        }
                    }
                    None => Err(utils::compile_err(expr, ERR)),
                }
            }

            //
            //
            //
            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

//
//
//
//
//
//
//

impl FromExpr for f32 {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: f32";

        match &lit_expr_or(expr, ERR)?.lit {
            Lit::Float(f) => {
                //
                match f.base10_digits().parse::<Self>() {
                    Ok(f) => Ok(f),

                    Err(e) => Err(utils::compile_err(expr, e.to_string())),
                }
            }

            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

impl FromExpr for f64 {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: f64";

        match &lit_expr_or(expr, ERR)?.lit {
            Lit::Float(f) => match f.base10_digits().parse::<Self>() {
                Ok(f) => Ok(f),
                Err(e) => Err(utils::compile_err(expr, e.to_string())),
            },

            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

impl FromExpr for Null<f32> {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: null or f32";

        match expr {
            //
            //
            // literal
            Expr::Lit(l) => {
                //
                match &l.lit {
                    Lit::Float(f) => match f.base10_digits().parse::<f32>() {
                        Ok(f) => Ok(Null::Some(f)),
                        Err(e) => Err(utils::compile_err(expr, e.to_string())),
                    },

                    _ => Err(utils::compile_err(&expr, ERR)),
                }
            }

            //
            //
            //
            // path
            Expr::Path(p) => {
                //
                match p.path.get_ident() {
                    Some(ident) => {
                        if ident.eq("null") {
                            Ok(Null::Null)
                        } else {
                            Err(utils::compile_err(expr, ERR))
                        }
                    }
                    None => Err(utils::compile_err(expr, ERR)),
                }
            }

            //
            //
            //
            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

impl FromExpr for Null<f64> {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: null or f64";

        match expr {
            //
            //
            // literal
            Expr::Lit(l) => {
                //
                match &l.lit {
                    Lit::Float(f) => match f.base10_digits().parse::<f64>() {
                        Ok(f) => Ok(Null::Some(f)),
                        Err(e) => Err(utils::compile_err(expr, e.to_string())),
                    },

                    _ => Err(utils::compile_err(&expr, ERR)),
                }
            }

            //
            //
            //
            // path
            Expr::Path(p) => {
                //
                match p.path.get_ident() {
                    Some(ident) => {
                        if ident.eq("null") {
                            Ok(Null::Null)
                        } else {
                            Err(utils::compile_err(expr, ERR))
                        }
                    }
                    None => Err(utils::compile_err(expr, ERR)),
                }
            }

            //
            //
            //
            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}
//
//
//
//
//
//
//
//
//
//

impl FromExpr for bool {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: boolean";
        match &lit_expr_or(expr, ERR)?.lit {
            Lit::Bool(b) => Ok(b.value),

            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

impl FromExpr for Null<bool> {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: null or boolean";

        match expr {
            //
            //
            // literal
            Expr::Lit(l) => {
                //
                match &l.lit {
                    Lit::Bool(b) => Ok(Null::Some(b.value)),

                    _ => Err(utils::compile_err(&expr, ERR)),
                }
            }

            //
            //
            //
            // path
            Expr::Path(p) => {
                //
                match p.path.get_ident() {
                    Some(ident) => {
                        if ident.eq("null") {
                            Ok(Null::Null)
                        } else {
                            Err(utils::compile_err(expr, ERR))
                        }
                    }
                    None => Err(utils::compile_err(expr, ERR)),
                }
            }

            //
            //
            //
            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
}

impl<T> FromExpr for Vec<T>
where
    T: FromExpr,
{
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: array";

        match expr {
            Expr::Array(arr) => {
                let mut vec = Vec::<T>::with_capacity(arr.elems.len());

                for elem in &arr.elems {
                    vec.push(T::from_expr(elem)?);
                }

                Ok(vec)
            }

            _ => return Err(utils::compile_err(&expr, ERR)),
        }
    }
}

impl FromExpr for Path {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: Path";

        match expr {
            Expr::Path(ep) => Ok(ep.path.clone()),

            _ => return Err(utils::compile_err(&expr, ERR)),
        }
    }
}

impl FromExpr for Null<Path> {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: Path";

        match expr {
            Expr::Path(ep) => match ep.path.get_ident() {
                Some(ident) if ident.eq("null") => Ok(Null::Null),

                _ => Ok(Null::Some(ep.path.clone())),
            },

            _ => return Err(utils::compile_err(&expr, ERR)),
        }
    }
}

//
//
//
//
//

/* impl FromExpr for Ident {
    fn from_expr(expr: &Expr) -> Result<Self, TokenStream> {
        const ERR: &str = "expected: identifier";
        match expr {
            Expr::Path(p) => match p.path.get_ident() {
                Some(p) => Ok(p.clone()),

                None => Err(utils::compile_err(
                    p,
                    "A path is considered an ident if:\n - the path has no leading colon,\n - the number of path segments is 1,\n - and the first path segment has no angle bracketed or parenthesized path arguments.",
                )),
            },

            _ => Err(utils::compile_err(expr, ERR)),
        }
    }
} */
