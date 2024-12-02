use std::{
    mem,
    ops::{Deref, DerefMut},
    pin::Pin,
};

#[derive(Debug, PartialEq)]
pub enum Null<T> {
    /// Some value of type `T`.
    Some(T),
    /// Null value.
    Null,
}

impl<T> Null<T> {
    /// Returns `true` if the Null is a [`Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::Null;
    /// let x: Null<u32> = Null::Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: Null<u32> = Null::Null;
    /// assert_eq!(x.is_some(), false);
    /// ```
    #[inline]
    pub const fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::Null => false,
        }
    }

    /// Returns `true` if the Null is a [`Null`] and the value inside of it matches a predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::Null;
    /// let x: Null<u32> = Null::Some(2);
    /// assert_eq!(x.is_some_and(|x| x > 1), true);
    ///
    /// let x: Null<u32> = Null::Some(0);
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    ///
    /// let x: Null<u32> = Null::Null;
    /// assert_eq!(x.is_some_and(|x| x > 1), false);
    /// ```

    #[must_use]
    #[inline]
    pub fn is_some_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Self::Null => false,
            Self::Some(x) => f(x),
        }
    }

    /// Returns `true` if the Null is a [`Null`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use from::Null;
    /// let x: Null<u32> = Null::Some(2);
    /// assert_eq!(x.is_null(), false);
    ///
    /// let x: Null<u32> = Null::Null;
    /// assert_eq!(x.is_null(), true);
    /// ```

    #[inline]
    pub const fn is_null(&self) -> bool {
        match self {
            Self::Some(_) => false,
            Self::Null => true,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> Null<&T> {
        match *self {
            Null::Some(ref x) => Null::Some(x),
            Null::Null => Null::Null,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Null<&mut T> {
        match *self {
            Null::Some(ref mut x) => Null::Some(x),
            Null::Null => Null::Null,
        }
    }

    #[inline]
    #[must_use]
    pub fn as_pin_ref(self: Pin<&Self>) -> Null<Pin<&T>> {
        match Pin::get_ref(self).as_ref() {
            // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
            // which is pinned.
            Null::Some(x) => unsafe { Null::Some(Pin::new_unchecked(x)) },
            Null::Null => Null::Null,
        }
    }

    #[inline]
    #[must_use]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Null<Pin<&mut T>> {
        // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe {
            match Pin::get_unchecked_mut(self).as_mut() {
                Null::Some(x) => Null::Some(Pin::new_unchecked(x)),
                Null::Null => Null::Null,
            }
        }
    }

    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Some(val) => val,
            Self::Null => panic!("{}", msg),
        }
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(val) => val,
            Self::Null => panic!("called `Self::unwrap()` on a `Null` value"),
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
            Self::Null => f(),
        }
    }

    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Self::Some(x) => x,
            Self::Null => T::default(),
        }
    }

    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_some());
        match self {
            Self::Some(val) => val,
            // SAFETY: the safety contract must be upheld by the caller.
            Self::Null => ::std::hint::unreachable_unchecked(),
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> Null<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Some(x) => Null::Some(f(x)),
            Self::Null => Null::Null,
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
            Self::Null => default,
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
            Self::Null => default(),
        }
    }

    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Self::Some(v) => Ok(v),
            Self::Null => Err(err),
        }
    }

    #[inline]
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Self::Some(v) => Ok(v),
            Self::Null => Err(err()),
        }
    }

    #[inline]
    pub fn as_deref(&self) -> Null<&T::Target>
    where
        T: Deref,
    {
        match self.as_ref() {
            Null::Some(t) => Null::Some(t.deref()),
            Null::Null => Null::Null,
        }
    }

    #[inline]
    pub fn as_deref_mut(&mut self) -> Null<&mut T::Target>
    where
        T: DerefMut,
    {
        match self.as_mut() {
            Null::Some(t) => Null::Some(t.deref_mut()),
            Null::Null => Null::Null,
        }
    }

    #[inline]
    pub fn and<U>(self, optb: Null<U>) -> Null<U> {
        match self {
            Self::Some(_) => optb,
            Self::Null => Null::Null,
        }
    }

    #[doc(alias = "flatmap")]
    #[inline]
    pub fn and_then<U, F>(self, f: F) -> Null<U>
    where
        F: FnOnce(T) -> Null<U>,
    {
        match self {
            Self::Some(x) => f(x),
            Self::Null => Null::Null,
        }
    }

    #[inline]
    pub fn filter<P>(self, predicate: P) -> Self
    where
        P: FnOnce(&T) -> bool,
    {
        if let Null::Some(x) = self {
            if predicate(&x) {
                return Null::Some(x);
            }
        }
        Null::Null
    }

    #[inline]
    pub fn or(self, optb: Null<T>) -> Null<T> {
        match self {
            x @ Null::Some(_) => x,
            Null::Null => optb,
        }
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> Null<T>
    where
        F: FnOnce() -> Null<T>,
    {
        match self {
            x @ Null::Some(_) => x,
            Null::Null => f(),
        }
    }

    #[inline]
    pub fn xor(self, optb: Null<T>) -> Null<T> {
        match (self, optb) {
            (a @ Null::Some(_), Null::Null) => a,
            (Null::Null, b @ Null::Some(_)) => b,
            _ => Null::Null,
        }
    }

    #[must_use = "if you intended to set a value, consider assignment instead"]
    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Null::Some(value);

        // SAFETY: the code above just filled the Null
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        if let Null::Null = *self {
            *self = Null::Some(value);
        }

        // SAFETY: a `Null` variant for `self` would have been replaced by a `Some`
        // variant in the code above.
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        if let Null::Null = self {
            *self = Null::Some(f());
        }

        // SAFETY: a `Null` variant for `self` would have been replaced by a `Some`
        // variant in the code above.
        unsafe { self.as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> Null<T> {
        mem::replace(self, Null::Some(value))
    }

    pub fn zip<U>(self, other: Null<U>) -> Null<(T, U)> {
        match (self, other) {
            (Null::Some(a), Null::Some(b)) => Null::Some((a, b)),
            _ => Null::Null,
        }
    }
}

impl<T, U> Null<(T, U)> {
    #[inline]
    pub fn unzip(self) -> (Null<T>, Null<U>) {
        match self {
            Null::Some((a, b)) => (Null::Some(a), Null::Some(b)),
            Null::Null => (Null::Null, Null::Null),
        }
    }
}

impl<T> Null<&T> {
    #[must_use = "`self` will be dropped if the result is not used"]
    pub const fn copied(self) -> Null<T>
    where
        T: Copy,
    {
        match self {
            Null::Some(&v) => Null::Some(v),
            Null::Null => Null::Null,
        }
    }

    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Null<T>
    where
        T: Clone,
    {
        match self {
            Null::Some(t) => Null::Some(t.clone()),
            Null::Null => Null::Null,
        }
    }
}

impl<T> Null<&mut T> {
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn copied(self) -> Null<T>
    where
        T: Copy,
    {
        match self {
            Null::Some(&mut t) => Null::Some(t),
            Null::Null => Null::Null,
        }
    }

    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Null<T>
    where
        T: Clone,
    {
        match self {
            Null::Some(t) => Null::Some(t.clone()),
            Null::Null => Null::Null,
        }
    }
}

impl<T, E> Null<Result<T, E>> {
    #[inline]
    pub fn transpose(self) -> Result<Null<T>, E> {
        match self {
            Null::Some(Ok(x)) => Ok(Null::Some(x)),
            Null::Some(Err(e)) => Err(e),
            Null::Null => Ok(Null::Null),
        }
    }
}

impl<T> Clone for Null<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Null::Some(x) => Null::Some(x.clone()),
            Null::Null => Null::Null,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Null::Some(to), Null::Some(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T> Default for Null<T> {
    #[inline]
    fn default() -> Null<T> {
        Null::Null
    }
}

impl<T> From<T> for Null<T> {
    fn from(val: T) -> Null<T> {
        Null::Some(val)
    }
}

impl<'a, T> From<&'a Null<T>> for Null<&'a T> {
    fn from(o: &'a Null<T>) -> Null<&'a T> {
        o.as_ref()
    }
}

impl<'a, T> From<&'a mut Null<T>> for Null<&'a mut T> {
    fn from(o: &'a mut Null<T>) -> Null<&'a mut T> {
        o.as_mut()
    }
}

impl<T> Null<Null<T>> {
    #[inline]
    pub fn flatten(self) -> Null<T> {
        match self {
            Null::Some(inner) => inner,
            Null::Null => Null::Null,
        }
    }
}
