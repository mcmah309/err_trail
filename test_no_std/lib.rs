#![no_std]

use trail::ErrContext;

// Compiled for every backend combination. This is a library so the final
// binary can supply any allocator and logger required by its chosen backends.
#[allow(unused_variables)]
pub fn log() {
    let x: Result<u32, &str> = Err("error value");
    let _: Result<u32, &str> = x.error("context around");
    let _: Option<u32> = x.info(()).ok();

    let error = TestError(42);
    let count = 3u64;
    trail::error!(%error, count, "Failed on attempt {count}");
    trail::warn!(target: "embedded", error = ?error, "Retrying {}", count);
    trail::info!({ device.code = 42u64, "request.count" = count, });
    trail::debug!(?error,);
    trail::trace!(concat!("code=", "{}"), error.0);
}

#[derive(Debug)]
struct TestError(u32);

impl core::error::Error for TestError {}

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TestError: {}", self.0)
    }
}
