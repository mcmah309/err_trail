struct NoFormatting;

fn main() {
    let value = NoFormatting;
    err_trail::error!(?value);
    err_trail::warn!(%value);
}
