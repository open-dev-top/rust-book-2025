fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
// RUST_BACKTRACE=1 cargo run --bin unrecoverable_errors_with_panic2