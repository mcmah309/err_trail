fn main() {
    err_trail::info!(parent: None, "unsupported");
    err_trail::warn!(name: "event", "unsupported");
    err_trail::error!(target: "first", target: "second", "duplicate");
    err_trail::debug!(value = 1, target: "late", "misplaced");
}
