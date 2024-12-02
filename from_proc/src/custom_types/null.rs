#[derive(Debug)]
pub enum Null<T> {
    Some(T),
    Null,
}
/*
impl<T> Null<T> {
    #[inline]
    pub const fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::Null => false,
        }
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        match self {
            Self::Some(_) => false,
            Self::Null => true,
        }
    }
} */
