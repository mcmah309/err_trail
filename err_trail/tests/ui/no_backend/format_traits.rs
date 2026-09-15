fn debug<T>(value: T) {
    err_trail::error!(?value);
}

fn display<T>(value: T) {
    err_trail::warn!(%value);
}

fn main() {}
