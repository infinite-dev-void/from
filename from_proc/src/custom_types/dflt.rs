use proc_macro2::TokenStream as TokenStream2;

pub enum Dflt<T> {
    Path(TokenStream2),
    Null,
    Value(T),
}
