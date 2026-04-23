use super::NativeWiimote;
use std::sync::atomic::{AtomicBool, Ordering};

static WARNING_PRINTED: AtomicBool = AtomicBool::new(false);

pub fn wiimotes_scan(_wiimotes: &mut Vec<NullNativeWiimote>) {
    if !WARNING_PRINTED.swap(true, Ordering::Relaxed) {
        eprintln!("wiimote-rs does not support this platform. You will not be able to connect Wii remotes.");
    }
}

pub const fn wiimotes_scan_cleanup() {}

pub struct NullNativeWiimote;

impl NativeWiimote for NullNativeWiimote {
    fn read(&mut self, _buffer: &mut [u8]) -> Option<usize> {
        unreachable!()
    }

    fn read_timeout(&mut self, _buffer: &mut [u8], _timeout_millis: usize) -> Option<usize> {
        unreachable!()
    }

    fn write(&mut self, _buffer: &[u8]) -> Option<usize> {
        unreachable!()
    }

    fn identifier(&self) -> String {
        unreachable!()
    }
}

impl Drop for NullNativeWiimote {
    fn drop(&mut self) {
        unreachable!()
    }
}
