use proc_macro::TokenStream;

use syn::{ext::IdentExt, punctuated::Punctuated, Attribute, Meta, MetaNameValue, Path, Token};

use crate::utils;

pub type Metas = Punctuated<Meta, Token![,]>;
pub type MetaNameValues = Punctuated<MetaNameValue, Token![,]>;
pub type MetaPaths = Punctuated<Path, Token![,]>;

pub mod value;

pub trait MetasHolder {
    // list
    fn parse_list_if_found(&self, ident: &str) -> Result<Option<Metas>, TokenStream>;

    fn parse_nvs_from_list_or_empty(&self, ident: &str) -> Result<MetaNameValues, TokenStream>;

    fn parse_paths_from_list_if_found(&self, ident: &str)
        -> Result<Option<MetaPaths>, TokenStream>;

    //
    // name_value
    fn parse_value_if_found<'a, T>(
        &'a self,
        name: &str,
    ) -> Result<Option<(T, &'a MetaNameValue)>, TokenStream>
    where
        T: value::FromExpr;

    fn parse_value_or_err<'a, T>(
        &'a self,
        name: &str,
        err: &'static str,
    ) -> Result<(T, &'a MetaNameValue), TokenStream>
    where
        T: value::FromExpr;

    //
    // path
    fn contains_ident(&self, ident: &str) -> bool;
}

impl MetasHolder for Vec<Attribute> {
    fn parse_list_if_found(&self, ident: &str) -> Result<Option<Metas>, TokenStream> {
        for attr in self {
            let list = match &attr.meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(Some(
                        list.parse_args_with(Metas::parse_terminated)
                            .map_err(|e| utils::spanned_compile_err(e.span(), e))?,
                    ));
                }

                _ => continue,
            }
        }

        Ok(None)
    }

    fn parse_nvs_from_list_or_empty(&self, ident: &str) -> Result<MetaNameValues, TokenStream> {
        for attr in self {
            let list = match &attr.meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(list
                        .parse_args_with(MetaNameValues::parse_terminated)
                        .map_err(|e| utils::spanned_compile_err(e.span(), e))?);
                }

                _ => continue,
            }
        }

        Ok(MetaNameValues::new())
    }

    fn parse_paths_from_list_if_found(
        &self,
        ident: &str,
    ) -> Result<Option<MetaPaths>, TokenStream> {
        for attr in self {
            let list = match &attr.meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(Some(
                        list.parse_args_with(MetaPaths::parse_terminated)
                            .map_err(|e| utils::spanned_compile_err(e.span(), e))?,
                    ));
                }

                _ => continue,
            }
        }

        Ok(None)
    }

    fn parse_value_if_found<'a, T>(
        &'a self,
        name: &str,
    ) -> Result<Option<(T, &'a MetaNameValue)>, TokenStream>
    where
        T: value::FromExpr,
    {
        for attr in self {
            let nv = match &attr.meta {
                Meta::NameValue(nv) => nv,
                _ => continue,
            };

            let nv_name = match nv.path.get_ident() {
                Some(nv_name) => nv_name,
                _ => continue,
            };

            if nv_name.ne(name) {
                continue;
            };

            return Ok(Some((T::from_expr(&nv.value)?, nv)));
        }

        Ok(None)
    }

    fn parse_value_or_err<'a, T>(
        &'a self,
        name: &str,
        err: &'static str,
    ) -> Result<(T, &'a MetaNameValue), TokenStream>
    where
        T: value::FromExpr,
    {
        for attr in self {
            let nv = match &attr.meta {
                Meta::NameValue(nv) => nv,
                _ => continue,
            };

            let nv_name = match nv.path.get_ident() {
                Some(nv_name) => nv_name,
                _ => continue,
            };

            if nv_name.ne(name) {
                continue;
            };

            return Ok((T::from_expr(&nv.value)?, nv));
        }

        // FIXME: must put something here but Vec<Attribute>
        // cannot be spanned. using this for now is okey
        // since this is not called anywhere.
        Err(utils::compile_err(&String::from(""), err))
    }

    // path
    fn contains_ident(&self, ident: &str) -> bool {
        for attr in self {
            match &attr.meta {
                Meta::Path(path) => match path.get_ident() {
                    Some(i) if i.unraw().eq(ident) => return true,

                    _ => continue,
                },

                _ => continue,
            }
        }

        false
    }
}

impl MetasHolder for Metas {
    fn parse_list_if_found(&self, ident: &str) -> Result<Option<Metas>, TokenStream> {
        for meta in self {
            let list = match meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(Some(
                        list.parse_args_with(Metas::parse_terminated)
                            .map_err(|e| utils::spanned_compile_err(e.span(), e))?,
                    ));
                }

                _ => continue,
            }
        }

        Ok(None)
    }

    fn parse_nvs_from_list_or_empty(&self, ident: &str) -> Result<MetaNameValues, TokenStream> {
        for meta in self {
            let list = match meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(list
                        .parse_args_with(MetaNameValues::parse_terminated)
                        .map_err(|e| utils::spanned_compile_err(e.span(), e))?);
                }

                _ => continue,
            }
        }

        Ok(MetaNameValues::new())
    }

    fn parse_paths_from_list_if_found(
        &self,
        ident: &str,
    ) -> Result<Option<MetaPaths>, TokenStream> {
        for meta in self {
            let list = match meta {
                Meta::List(list) => list,
                _ => continue,
            };

            match list.path.get_ident() {
                Some(i) if i.unraw().eq(ident) => {
                    return Ok(Some(
                        list.parse_args_with(MetaPaths::parse_terminated)
                            .map_err(|e| utils::spanned_compile_err(e.span(), e))?,
                    ));
                }

                _ => continue,
            }
        }

        Ok(None)
    }

    fn parse_value_if_found<'a, T>(
        &'a self,
        name: &str,
    ) -> Result<Option<(T, &'a MetaNameValue)>, TokenStream>
    where
        T: value::FromExpr,
    {
        for meta in self {
            let nv = match meta {
                Meta::NameValue(nv) => nv,
                _ => continue,
            };

            let nv_name = match nv.path.get_ident() {
                Some(nv_name) => nv_name,
                _ => continue,
            };

            if nv_name.ne(name) {
                continue;
            };

            return Ok(Some((T::from_expr(&nv.value)?, nv)));
        }

        Ok(None)
    }

    fn parse_value_or_err<'a, T>(
        &'a self,
        name: &str,
        err: &'static str,
    ) -> Result<(T, &'a MetaNameValue), TokenStream>
    where
        T: value::FromExpr,
    {
        for meta in self {
            let nv = match meta {
                Meta::NameValue(nv) => nv,
                _ => continue,
            };

            let nv_name = match nv.path.get_ident() {
                Some(nv_name) => nv_name,
                _ => continue,
            };

            if nv_name.ne(name) {
                continue;
            };

            return Ok((T::from_expr(&nv.value)?, nv));
        }

        Err(utils::compile_err(self, err))
    }

    // path
    fn contains_ident(&self, ident: &str) -> bool {
        for meta in self {
            match meta {
                Meta::Path(path) => match path.get_ident() {
                    Some(i) if i.unraw().eq(ident) => return true,

                    _ => continue,
                },

                _ => continue,
            }
        }

        false
    }
}
