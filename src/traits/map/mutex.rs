#[cfg(feature = "parking_lot")]
impl<'a, 'g, T> super::Map<'a, 'g, T, parking_lot::MutexGuard<'g, T>> for parking_lot::Mutex<T> {
    fn map<F, U>(&'a self, f: F) -> U
    where
        F: FnOnce(parking_lot::MutexGuard<'g, T>) -> U,
        'a: 'g,
    {
        f(self.lock())
    }
}

#[cfg(feature = "parking_lot")]
#[derive(Debug, thiserror::Error)]
#[error("Failed to lock the mutex")]
/// Failed to lock the mutex
pub struct LockError;

#[cfg(feature = "parking_lot")]
impl<'a, 'g, T> super::TryMap<'a, 'g, T, parking_lot::MutexGuard<'g, T>, LockError>
    for parking_lot::Mutex<T>
{
    fn try_map<F, U>(&'a self, f: F) -> Result<U, LockError>
    where
        F: FnOnce(parking_lot::MutexGuard<'g, T>) -> U,
        'a: 'g,
    {
        match self.try_lock() {
            Some(guard) => Ok(f(guard)),
            None => Err(LockError),
        }
    }
}

impl<'a, 'g, T>
    super::TryMap<
        'a,
        'g,
        T,
        std::sync::MutexGuard<'g, T>,
        std::sync::TryLockError<std::sync::MutexGuard<'g, T>>,
    > for std::sync::Mutex<T>
{
    fn try_map<F, U>(
        &'a self,
        f: F,
    ) -> Result<U, std::sync::TryLockError<std::sync::MutexGuard<'g, T>>>
    where
        F: FnOnce(std::sync::MutexGuard<'g, T>) -> U,
        'a: 'g,
    {
        match self.try_lock() {
            Ok(guard) => Ok(f(guard)),
            Err(e) => Err(e),
        }
    }
}
