#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std as std;
    } else {
        use core as std;
    }
}

pub mod prelude {
    //! `use quork::prelude::*` To include common helpful items

    cfg_if::cfg_if! {
        if #[cfg(feature = "traits")] {
            pub use crate::traits::prelude::*;
        }
    }

    #[cfg(feature = "macros")]
    pub use crate::macros::*;

    #[cfg(feature = "root")]
    pub use crate::root::is_root;
}

#[cfg(windows)]
pub mod win;

#[cfg(unix)]
pub mod unix;

#[cfg(feature = "macros")]
pub mod macros;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "network")]
pub mod network;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "root", feature = "std"))] {
        pub mod root;
    }
}

/// Truncation helpers for truncating strings when formatting
pub mod truncate;
