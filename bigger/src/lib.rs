use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values()
        .cloned()
        .filter(|&v| v > 0)
        .max()
        .unwrap_or(0)
}
