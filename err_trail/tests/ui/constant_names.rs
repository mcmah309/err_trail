fn main() {
    err_trail::info!({ FIELD } = 1);
    err_trail::info!(value = 1, { FIELD } = 2);
    err_trail::info!({ { FIELD } = 3 });
}
