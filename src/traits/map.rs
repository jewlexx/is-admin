//! Provides a generalized map function
//!
//! This is primarily designed for mapping Mutex types, but can realistically be used for anything in future.

mod mutex;

#[allow(clippy::module_name_repetitions)]
/// Map a mutex lock
pub trait Map<'a, 'g, T: ?Sized, G: 'g> {
    /// Maps a value of T to a value of U
    fn map<F, U>(&'a self, f: F) -> U
    where
        F: FnOnce(G) -> U + 'g,
        'a: 'g;
}

#[allow(clippy::module_name_repetitions)]
/// Attempts to map a mutex lock
pub trait TryMap<'a, 'g, T: ?Sized, G: 'g, E = G> {
    /// Maps a value of T to a value of U
    ///
    /// # Errors
    /// - Locking the mutex failed
    fn try_map<F, U>(&'a self, f: F) -> Result<U, E>
    where
        F: FnOnce(G) -> U + 'g,
        'a: 'g;
}

// impl<'a, 'g, T: ?Sized, G: 'g, S> TryMap<'a, 'g, T, G> for S
// where
//     S: Map<'a, 'g, T, G>,
// {
//     fn try_map<F, U>(&'a self, f: F) -> Result<U, G>
//     where
//         F: FnOnce(G) -> U + 'g,
//         'a: 'g,
//     {
//         Ok(self.map(f))
//     }
// }
