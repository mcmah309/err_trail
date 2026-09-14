#![no_std]
#![no_main]

use exit_no_std::exit;

#[unsafe(no_mangle)]
fn main() -> i32 {
    exit(0);
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
