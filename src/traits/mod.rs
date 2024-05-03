//! Trait implementations

cfg_if::cfg_if! {
    // TODO: Get these working without std crate (as much as possible)
    if #[cfg(feature = "std")] {
        pub mod flip;
        pub mod lock;
        pub mod truncate;
    }
}

pub mod list;
pub mod truthy;

/// Trait prelude imports
pub mod prelude {
    cfg_if::cfg_if! {
        // TODO: Get these working without std crate (as much as possible)
        if #[cfg(feature = "std")] {
            pub use super::flip::*;
            pub use super::lock::*;
            pub use super::truncate::*;
        }
    }

    pub use super::list::*;
    pub use super::truthy::*;
}
