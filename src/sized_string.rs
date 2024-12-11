//! A sized, stack allocated string type.
//!
//! This is useful for when you need a string to be stack allocated, but you also need it to be sized (i.e not a reference to a [`str`]).
//!
//! Especially when using shared memory this can be useful as the actual string will be stored in shared memory, rather than just the pointer to the string.

use std::ops::Deref;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
/// A sized, stack allocated string type.
///
/// This is useful for when you need a string to be stack allocated, but you also need it to be sized (i.e not a reference to a [`str`]).
///
/// Especially when using shared memory this can be useful as the actual string will be stored in shared memory, rather than just the pointer to the string.
pub struct SizedString<const N: usize>([u8; N]);

impl<const N: usize> SizedString<N> {
    #[must_use]
    /// Construct a new [`SizedString`] from a byte array
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    #[must_use]
    /// Get the string as a [`str`]
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl<const N: usize> Deref for SizedString<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl<const N: usize> AsRef<str> for SizedString<N> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const N: usize, T> AsRef<T> for SizedString<N>
where
    str: std::convert::AsRef<T>,
{
    fn as_ref(&self) -> &T {
        let s: &str = self.as_ref();
        s.as_ref()
    }
}

mod string_trait_impls {
    use std::fmt::Debug;

    use super::SizedString;

    impl<const N: usize> Debug for SizedString<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("SizedString").field(&self.as_str()).finish()
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sized_string() {
        let s = quork_proc::sized_string!("hello world");

        let s_str: &str = s.as_ref();

        assert_eq!(s.len(), 11);
        assert_eq!(s_str, "hello world");
    }
}
