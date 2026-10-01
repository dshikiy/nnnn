pub fn stars(n: u32) -> String {
    let count = 2_usize.pow(n);
    "*".repeat(count)
}
