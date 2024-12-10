//! Windows specific functionality.

use core::sync::atomic::{AtomicBool, Ordering};

use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "root")]
pub mod root;

pub(crate) static COM_INIT: ComInit = ComInit {
    initialized: AtomicBool::new(false),
};

#[derive(Debug)]
pub(crate) struct ComInit {
    initialized: AtomicBool,
}

impl ComInit {
    pub(crate) unsafe fn init() {
        if !COM_INIT.initialized.load(Ordering::Relaxed) {
            COM_INIT.initialized.store(
                CoInitializeEx(None, COINIT_MULTITHREADED).is_ok(),
                Ordering::Relaxed,
            );
        }
    }
}

// As COM_INIT is a static variable this will be dropped at the end of the program.
impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}
