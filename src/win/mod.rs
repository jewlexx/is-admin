use std::sync::{atomic::Ordering, Mutex};

use once_cell::sync::Lazy;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

use crate::IsTrue;

pub(crate) static COM_INIT: Mutex<Lazy<ComInit>> = Mutex::new(Lazy::new(ComInit::init));

#[derive(Debug, Clone)]
pub(crate) struct ComInit {
    initialized: bool,
}

impl ComInit {
    pub fn init() -> Self {
        Self {
            initialized: unsafe { Self::init_com() }.is_ok(),
        }
    }

    unsafe fn init_com() -> Result<(), windows::core::Error> {
        if COM_INIT.lock().map(|lock| !lock.initialized).is_true() {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;
            COM_INIT.store(true, Ordering::Relaxed);
        }
        Ok(())
    }
}

impl Drop for ComInit {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

pub mod network;
