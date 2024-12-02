use std::{
    mem,
    ops::{Deref, DerefMut},
    pin::Pin,
};

#[derive(Debug, PartialEq)]
pub enum OptionNull<T> {
    Some(T),
    Null,
    None,
}

impl<T> OptionNull<T> {
    /// Returns `true` if the OptionNull is a [`Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::OptionNull;
    /// let x: OptionNull<u32> = OptionNull::Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: OptionNull<u32> = OptionNull::Null;
    /// assert_eq!(x.is_some(), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::None;
    /// assert_eq!(x.is_some(), false);
    /// ```
    #[inline]
    pub const fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the OptionNull is a [`Null`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::OptionNull;
    /// let x: OptionNull<u32> = OptionNull::Some(2);
    /// assert_eq!(x.is_some_and(|x| x > 1), true);
    ///
    /// let x: OptionNull<u32> = OptionNull::Some(0);
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::Null;
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    /// ```

    #[must_use]
    #[inline]
    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Self::Some(x) => f(x),
            _ => false,
        }
    }

    /// Returns `true` if the OptionNull is a [`Null`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::OptionNull;
    /// let x: OptionNull<u32> = OptionNull::Some(2);
    /// assert_eq!(x.is_null(), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::None;
    /// assert_eq!(x.is_null(), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::Null;
    /// assert_eq!(x.is_null(), true);
    /// ```

    #[inline]
    pub const fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }

    /// Returns `true` if the OptionNull is a [`Null`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::OptionNull;
    /// let x: OptionNull<u32> = OptionNull::Some(2);
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::Null;
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x: OptionNull<u32> = OptionNull::None;
    /// assert_eq!(x.is_none(), true);
    /// ```

    #[inline]
    pub const fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> OptionNull<&T> {
        match *self {
            OptionNull::Some(ref x) => OptionNull::Some(x),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> OptionNull<&mut T> {
        match *self {
            OptionNull::Some(ref mut x) => OptionNull::Some(x),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    #[must_use]
    pub fn as_pin_ref(self: Pin<&Self>) -> OptionNull<Pin<&T>> {
        match Pin::get_ref(self).as_ref() {
            // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
            // which is pinned.
            OptionNull::Some(x) => unsafe { OptionNull::Some(Pin::new_unchecked(x)) },
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    #[must_use]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> OptionNull<Pin<&mut T>> {
        // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe {
            match Pin::get_unchecked_mut(self).as_mut() {
                OptionNull::Some(x) => OptionNull::Some(Pin::new_unchecked(x)),
                OptionNull::Null => OptionNull::Null,
                OptionNull::None => OptionNull::None,
            }
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Some(val) => val,
            _ => panic!("{}", msg),
        }
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(val) => val,
            Self::Null => panic!("called `Self::unwrap()` on a `Null` value"),
            Self::None => panic!("called `Self::unwrap()` on a `None` value"),
        }
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Some(x) => x,
            _ => default,
        }
    }

    #[inline]
    #[track_caller]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Self::Some(x) => x,
            _ => f(),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Self::Some(x) => x,
            _ => T::default(),
        }
    }

    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_some());
        match self {
            Self::Some(val) => val,
            // SAFETY: the safety contract must be upheld by the caller.
            _ => ::std::hint::unreachable_unchecked(),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> OptionNull<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(x) => OptionNull::Some(f(x)),
            Self::Null => OptionNull::Null,
            Self::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Self::Some(ref x) = self {
            f(x);
        }

        self
    }

    #[inline]
    #[must_use = "if you don't need the returned value, use `if let` instead"]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(t) => f(t),
            _ => default,
        }
    }

    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(t) => f(t),
            _ => default(),
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Self::Some(v) => Ok(v),
            _ => Err(err),
        }
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Self::Some(v) => Ok(v),
            _ => Err(err()),
        }
    }

    #[inline]
    pub fn as_deref(&self) -> OptionNull<&T::Target>
    where
        T: Deref,
    {
        match self.as_ref() {
            OptionNull::Some(t) => OptionNull::Some(t.deref()),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn as_deref_mut(&mut self) -> OptionNull<&mut T::Target>
    where
        T: DerefMut,
    {
        match self.as_mut() {
            OptionNull::Some(t) => OptionNull::Some(t.deref_mut()),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn and<U>(self, optb: OptionNull<U>) -> OptionNull<U> {
        match self {
            Self::Some(_) => optb,
            Self::Null => OptionNull::Null,
            Self::None => OptionNull::None,
        }
    }

    #[doc(alias = "flatmap")]
    #[inline]
    pub fn and_then<U, F>(self, f: F) -> OptionNull<U>
    where
        F: FnOnce(T) -> OptionNull<U>,
    {
        match self {
            Self::Some(x) => f(x),
            Self::Null => OptionNull::Null,
            Self::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            OptionNull::Some(x) => {
                if predicate(&x) {
                    return OptionNull::Some(x);
                } else {
                    OptionNull::None
                }
            }
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    pub fn or(self, optb: OptionNull<T>) -> OptionNull<T> {
        match self {
            x @ OptionNull::Some(_) => x,
            _ => optb,
        }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> OptionNull<T>
    where
        F: FnOnce() -> OptionNull<T>,
    {
        match self {
            x @ OptionNull::Some(_) => x,
            _ => f(),
        }
    }

    /* #[inline]
    pub fn xor(self, optb: OptionNull<T>) -> OptionNull<T> {
        match (self, optb) {
            (a @ OptionNull::Some(_), OptionNull::Null) => a,
            (OptionNull::Null, b @ OptionNull::Some(_)) => b,
            _ => OptionNull::Null,
        }
    } */

    #[must_use = "if you intended to set a value, consider assignment instead"]
    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = OptionNull::Some(value);

        // SAFETY: the code above just filled the Null
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        match self {
            OptionNull::Some(_) => {}

            _ => {
                *self = OptionNull::Some(value);
            }
        };

        // SAFETY: a `Null` variant for `self` would have been replaced by a `Some`
        // variant in the code above.
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            OptionNull::Some(_) => {}

            _ => {
                *self = OptionNull::Some(f());
            }
        };

        // SAFETY: a `Null` variant for `self` would have been replaced by a `Some`
        // variant in the code above.
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> OptionNull<T> {
        mem::replace(self, OptionNull::Some(value))
    }

    pub fn zip<U>(self, other: OptionNull<U>) -> OptionNull<(T, U)> {
        match (self, other) {
            (OptionNull::Some(a), OptionNull::Some(b)) => OptionNull::Some((a, b)),
            _ => OptionNull::Null,
        }
    }
}

impl<T, U> OptionNull<(T, U)> {
    #[inline]
    pub fn unzip(self) -> (OptionNull<T>, OptionNull<U>) {
        match self {
            OptionNull::Some((a, b)) => (OptionNull::Some(a), OptionNull::Some(b)),
            OptionNull::Null => (OptionNull::Null, OptionNull::Null),
            OptionNull::None => (OptionNull::None, OptionNull::None),
        }
    }
}

impl<T> OptionNull<&T> {
    #[must_use = "`self` will be dropped if the result is not used"]
    pub const fn copied(self) -> OptionNull<T>
    where
        T: Copy,
    {
        match self {
            OptionNull::Some(&v) => OptionNull::Some(v),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> OptionNull<T>
    where
        T: Clone,
    {
        match self {
            OptionNull::Some(t) => OptionNull::Some(t.clone()),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }
}

impl<T> OptionNull<&mut T> {
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn copied(self) -> OptionNull<T>
    where
        T: Copy,
    {
        match self {
            OptionNull::Some(&mut t) => OptionNull::Some(t),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> OptionNull<T>
    where
        T: Clone,
    {
        match self {
            OptionNull::Some(t) => OptionNull::Some(t.clone()),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }
}

impl<T, E> OptionNull<Result<T, E>> {
    #[inline]
    pub fn transpose(self) -> Result<OptionNull<T>, E> {
        match self {
            OptionNull::Some(Ok(x)) => Ok(OptionNull::Some(x)),
            OptionNull::Some(Err(e)) => Err(e),
            OptionNull::Null => Ok(OptionNull::Null),
            OptionNull::None => Ok(OptionNull::None),
        }
    }
}

impl<T> Clone for OptionNull<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            OptionNull::Some(x) => OptionNull::Some(x.clone()),
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (OptionNull::Some(to), OptionNull::Some(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T> Default for OptionNull<T> {
    #[inline]
    fn default() -> OptionNull<T> {
        OptionNull::None
    }
}

impl<T> From<T> for OptionNull<T> {
    fn from(val: T) -> OptionNull<T> {
        OptionNull::Some(val)
    }
}

impl<'a, T> From<&'a OptionNull<T>> for OptionNull<&'a T> {
    fn from(o: &'a OptionNull<T>) -> OptionNull<&'a T> {
        o.as_ref()
    }
}

impl<'a, T> From<&'a mut OptionNull<T>> for OptionNull<&'a mut T> {
    fn from(o: &'a mut OptionNull<T>) -> OptionNull<&'a mut T> {
        o.as_mut()
    }
}

impl<T> OptionNull<OptionNull<T>> {
    #[inline]
    pub fn flatten(self) -> OptionNull<T> {
        match self {
            OptionNull::Some(inner) => inner,
            OptionNull::Null => OptionNull::Null,
            OptionNull::None => OptionNull::None,
        }
    }
}
