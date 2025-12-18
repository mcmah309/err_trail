#![no_std]
#![no_main]

use err_trail::ErrContext;
use exit_no_std::exit;

#[unsafe(no_mangle)]
fn main() -> i32 {
    exit(0);
}

// Purposely not called since then we would have to set up the logger. Just making sure it compiles.
#[allow(dead_code)]
fn log() {
    let x: Result<u32, &str> = Err("error value");
    let _: Result<u32, &str> = x.error("context around");
    let _: Option<u32> = x.info(()).ok();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1);
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {
    exit(2);
}

#[derive(Debug)]
pub struct TestError(u32);

impl TestError {
    pub fn new(code: u32) -> Self {
        Self(code)
    }
}

impl core::error::Error for TestError {}

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "TestError: {}", self.0)
    }
}
