#![deprecated(
    since = "0.8.0",
    note = "Use the `map::Map` trait instead. This trait will be removed in the future."
)]

//! Map a Mutex Lock
//!
//! Can theoretically be anything but designed primarily for Mutex Locks

#[allow(clippy::module_name_repetitions)]
/// Map a mutex lock
pub trait LockMap<T: ?Sized> {
    /// Maps a mutex lock of T to a value of U
    fn map<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Self) -> U,
    {
        f(self)
    }
}

impl<'a, T> LockMap<T> for std::sync::MutexGuard<'a, T> {}

#[cfg(feature = "spin")]
impl<'a, T> LockMap<T> for spin::mutex::MutexGuard<'a, T> {}

#[cfg(feature = "lock_api")]
impl<'a, T, G: lock_api::RawMutex> LockMap<T> for lock_api::MutexGuard<'a, G, T> {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    static MUTEX: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

    #[test]
    fn test_lock_map() {
        MUTEX.lock().map(|mut m| m.flip()).unwrap();
    }
}
