use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut map = HashMap::new();

    for c in s1.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        let count = map.entry(c).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }

    map.values().all(|&v| v == 0)
}
