//! Utilities to flip booleans

/// Flip the given value
///
/// Usually used to flip a boolean value
pub trait Flip {
    #[must_use]
    /// Returns the flipped value
    fn flipped(&self) -> Self;

    /// Mutates the value to the flipped value
    fn flip(&mut self)
    where
        Self: Sized,
    {
        *self = self.flipped();
    }
}

impl Flip for bool {
    fn flipped(&self) -> Self {
        !self
    }
}

#[cfg(feature = "std")]
mod _std {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard, TryLockError,
    };

    use super::Flip;

    /// Error types for the [`FlipImmut`] trait
    #[derive(Debug, thiserror::Error)]
    pub enum Error<T> {
        /// The error returned by the std `try_lock` method
        StdLockError(#[from] TryLockError<T>),

        /// The [`parking_lot::Mutex::try_lock`] function returned [`None`]
        #[cfg(feature = "parking_lot")]
        LockError,
    }

    #[allow(clippy::module_name_repetitions)]
    /// Immutable version of the [`Flip`] trait, intended for use on static variables
    ///
    /// # Examples
    ///
    /// ```
    /// use quork::traits::flip::{FlipImmut, Flip};
    /// use std::sync::atomic::{AtomicBool, Ordering};
    ///
    /// static FOO: AtomicBool = AtomicBool::new(false);
    /// assert!(!FOO.load(Ordering::Relaxed));
    /// FOO.flip();
    /// assert!(FOO.load(Ordering::Relaxed));
    /// ```
    pub trait FlipImmut<'a, T: Flip + core::fmt::Debug>
    where
        Self: Sized,
        Self::Error: std::fmt::Debug,
    {
        /// The error type for the immutable flip trait
        type Error;

        /// Attempt to flip the value
        ///
        /// # Errors
        /// - When `parking_lot` feature is enabled, will return an error if [`parking_lot::Mutex::try_lock()`] returns an error
        fn try_flip(&'a self) -> Result<(), Self::Error>;

        /// Flip the value
        ///
        /// # Panics
        /// - Will panic if the [`FlipImmut::try_flip`] method returns an error
        fn flip(&'a self) {
            self.try_flip().unwrap();
        }

        /// Attempt to flip the value, without mutating the value
        ///
        /// # Errors
        /// - When `parking_lot` feature is enabled, will return an error if [`parking_lot::Mutex::try_lock()`] returns an error
        fn try_flipped(&'a self) -> Result<T, Self::Error>;

        /// Flip the value
        ///
        /// # Panics
        /// - Will panic if the [`FlipImmut::try_flipped`] method returns an error
        fn flipped(&'a self) -> T {
            self.try_flipped().unwrap()
        }
    }

    impl<'a> FlipImmut<'a, bool> for AtomicBool {
        type Error = Error<bool>;

        fn try_flip(&'a self) -> Result<(), Self::Error> {
            let val = self.load(Ordering::Relaxed);
            self.store(!val, Ordering::Relaxed);

            Ok(())
        }

        fn try_flipped(&'a self) -> Result<bool, Self::Error> {
            let val = self.load(Ordering::Relaxed);

            Ok(!val)
        }
    }

    impl<'a, T: Flip + core::fmt::Debug + 'a> FlipImmut<'a, T> for Mutex<T> {
        type Error = Error<MutexGuard<'a, T>>;

        fn try_flip(&'a self) -> Result<(), Self::Error> {
            self.try_lock()?.flip();

            Ok(())
        }

        fn try_flipped(&'a self) -> Result<T, Self::Error> {
            Ok(self.try_lock()?.flipped())
        }
    }

    #[cfg(feature = "parking_lot")]
    impl<'a, T: Flip + core::fmt::Debug + 'a> FlipImmut<'a, T> for parking_lot::Mutex<T> {
        type Error = Error<MutexGuard<'a, T>>;

        fn flip(&'a self) {
            self.lock().flip()
        }

        fn try_flip(&'a self) -> Result<(), Self::Error> {
            match self.try_lock() {
                Some(mut v) => Ok(v.flip()),
                None => Err(Error::LockError),
            }
        }

        fn try_flipped(&'a self) -> Result<T, Self::Error> {
            match self.try_lock() {
                Some(v) => Ok(v.flipped()),
                None => Err(Error::LockError),
            }
        }
    }
}

#[cfg(feature = "std")]
pub use _std::*;

#[cfg(test)]
mod tests {
    use core::sync::atomic::AtomicBool;
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn test_flip_bool() {
        let mut bool = true;

        assert!(bool);

        bool.flip();

        assert!(!bool);

        assert!(bool.flipped());
    }

    static BOOL: Mutex<bool> = Mutex::new(true);

    #[test]
    fn test_flip_mutex() {
        assert!(*BOOL.lock().unwrap());

        BOOL.flip();

        assert!(!*BOOL.lock().unwrap());

        assert!(BOOL.flipped());
    }

    #[cfg(feature = "parking_lot")]
    static PARKING_LOT: parking_lot::Mutex<bool> = parking_lot::Mutex::new(true);

    #[cfg(feature = "parking_lot")]
    #[test]
    fn test_flip_parking_mutex() {
        assert!(*PARKING_LOT.lock());

        PARKING_LOT.flip();

        assert!(!*PARKING_LOT.lock());

        assert!(PARKING_LOT.flipped())
    }

    static ATOMIC: AtomicBool = AtomicBool::new(true);

    impl Flip for AtomicBool {
        fn flipped(&self) -> Self {
            let mut bool = self.load(std::sync::atomic::Ordering::Relaxed);

            bool.flip();

            Self::new(bool)
        }
    }

    #[test]
    fn test_flip_atomic() {
        let bool = ATOMIC.load(std::sync::atomic::Ordering::Relaxed);

        assert!(bool);

        ATOMIC.flip();
    }
}
