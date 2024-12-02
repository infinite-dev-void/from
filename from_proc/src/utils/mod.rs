use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

pub mod msgs_attribute;

#[track_caller]
#[inline]
pub fn compile_err<M: std::fmt::Display, S: Spanned>(s: &S, err: M) -> TokenStream {
    let caller_location = std::panic::Location::caller();
    let mut msg = String::from("Error: ");

    msg.push_str(&err.to_string());
    msg.push_str("\n========== Source ==========");
    msg.push_str("\nFile: ");
    msg.push_str(caller_location.file());
    msg.push_str(" \nLine: ");
    msg.push_str(&caller_location.line().to_string());
    msg.push('\n');

    TokenStream::from(syn::Error::new(s.span(), msg).into_compile_error())
}

#[track_caller]
#[inline]
pub fn spanned_compile_err<M: std::fmt::Display>(s: Span, err: M) -> TokenStream {
    let caller_location = std::panic::Location::caller();
    let mut msg = String::from("Error: ");

    msg.push_str(&err.to_string());
    msg.push_str("\n========== Source ==========");
    msg.push_str("\nFile: ");
    msg.push_str(caller_location.file());
    msg.push_str(" \nLine: ");
    msg.push_str(&caller_location.line().to_string());
    msg.push('\n');

    TokenStream::from(syn::Error::new(s, msg).into_compile_error())
}

pub trait Append: Extend<TokenTree> {
    fn append(&mut self, ts: TokenStream2);
    fn append_ref(&mut self, ts: &TokenStream2);
}

impl Append for TokenStream2 {
    #[inline]
    fn append(&mut self, ts: TokenStream2) {
        self.extend(::core::iter::once(ts))
    }

    fn append_ref(&mut self, ts: &TokenStream2) {
        self.extend(::core::iter::once(ts.clone()))
    }
}

pub fn vec_into_token_stream2_array<T>(vec: Vec<T>) -> TokenStream2
where
    T: ToTokens,
{
    if vec.len() == 0 {
        return quote! {[]};
    };

    let mut elems_ts = TokenStream2::new();

    for elem in vec[0..vec.len() - 1].iter() {
        elem.to_tokens(&mut elems_ts);
        quote::__private::push_comma(&mut elems_ts);
    }

    vec[vec.len() - 1].to_tokens(&mut elems_ts);

    let mut arr = TokenStream2::new();

    quote::__private::push_group(&mut arr, quote::__private::Delimiter::Bracket, elems_ts);

    arr
}
