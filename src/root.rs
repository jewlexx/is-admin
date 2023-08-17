//! Checks if the current user has root privileges.

#[derive(Debug, thiserror::Error)]
/// Errors when checking if process is root
pub enum RootError {
    #[cfg(windows)]
    /// The Windows Process elevation cannot be checked
    #[error("Windows related error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

/// Checks if the process has root privileges
///
/// # Errors
/// - On Windows, may return an error if the process elevation cannot be checked
/// - On Unix-like, no errors will ever be returned
pub fn is_root() -> Result<bool, RootError> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            let root = crate::win::root::is_elevated()?;
        } else if #[cfg(unix)] {
            let root = crate::unix::root::is_root();
        }
    };

    Ok(root)
}

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
