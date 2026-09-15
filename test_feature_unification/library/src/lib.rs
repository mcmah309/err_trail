#![no_std]
#![deny(warnings)]

use core::cell::Cell;
use trail::{ErrContext, NoneContext};

pub fn emit_logs(evaluations: &Cell<u32>) {
    let error = "library error";
    trail::error!(error = ?error);
    trail::warn!(
        code = {
            evaluations.set(evaluations.get() + 1);
            42u64
        },
        "library warning"
    );
    let message = "library info";
    trail::info!("{message}");
    trail::debug!("library debug");
    trail::trace!("library trace");

    let _ = Result::<(), &str>::Err("library context").error(());
    let _ = Option::<()>::None.warn(|| {
        evaluations.set(evaluations.get() + 1);
        "library missing"
    });
}
