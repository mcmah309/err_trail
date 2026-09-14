fn main() {
    err_trail::info!(value = ?);
    err_trail::info!(value = 1 other = 2);
    err_trail::info!(%compute());
    err_trail::info!({ "quoted" });
    err_trail::info!();
}
