use proc_macro2::TokenStream as TokenStream2;

use crate::utils::Append;

pub struct Tokens(Vec<TokenStream2>);

impl Tokens {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn push(&mut self, t: TokenStream2) {
        self.0.push(t);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn join_with_suffix(self, sep: &TokenStream2, suffix: &TokenStream2) -> TokenStream2 {
        let len = self.0.len();
        if len == 0 {
            return TokenStream2::new();
        };

        if len == 1 {
            return self.0[0].clone();
        };

        let mut joined = TokenStream2::new();

        let last = self.0.len() - 1;

        let mut iter = self.0.into_iter();

        for _ in 0..last {
            unsafe {
                joined.extend(::core::iter::once(iter.next().unwrap_unchecked()));
            };
            joined.extend(::core::iter::once(sep.clone()));
        }

        unsafe {
            joined.extend(::core::iter::once(iter.next().unwrap_unchecked()));
        };

        joined.append_ref(suffix);

        joined
    }

    pub fn join(self, sep: TokenStream2) -> TokenStream2 {
        let len = self.0.len();
        if len == 0 {
            return TokenStream2::new();
        };

        if len == 1 {
            return self.0[0].clone();
        };

        let mut joined = TokenStream2::new();

        let last = self.0.len() - 1;

        let mut iter = self.0.into_iter();

        for _ in 0..last {
            unsafe {
                joined.extend(::core::iter::once(iter.next().unwrap_unchecked()));
            };
            joined.extend(::core::iter::once(sep.clone()));
        }

        unsafe {
            joined.extend(::core::iter::once(iter.next().unwrap_unchecked()));
        };

        joined
    }
}
