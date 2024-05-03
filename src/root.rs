//! Checks if the current process has root privileges.

#[derive(Debug, thiserror::Error)]
/// Errors when checking if process is root
pub enum Error {
    #[cfg(windows)]
    #[error("Windows related error: {0}")]
    /// The Windows Process elevation cannot be checked
    WindowsError(#[from] windows::core::Error),
}

#[allow(clippy::module_name_repetitions)]
/// Checks if the process has root privileges
///
/// # Errors
/// - On Windows, may return an error if the process elevation cannot be checked
/// - On Unix-like, no errors will ever be returned
pub fn is_root() -> Result<bool, Error> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            let root = crate::win::root::is_elevated()?;
        } else if #[cfg(unix)] {
            let root = crate::unix::root::is_root();
        }
    };

    Ok(root)
}

// TODO: Platform agnostic Uid struct?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_root() {
        // You probably shouldn't be running cargo commands as root
        // Just checks if the command runs, and is consistent across the inspiration for the implementation
        assert_eq!(is_root().unwrap(), is_root::is_root());
    }
}
