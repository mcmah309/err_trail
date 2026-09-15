fn main() {
    // A closure lacks both traits without version-specific derive suggestions.
    let value = || {};
    err_trail::error!(?value);
    err_trail::warn!(%value);
}
