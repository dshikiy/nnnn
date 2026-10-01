pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let offset = c as i32 - b'a' as i32;
                let new_offset = (offset + key as i32).rem_euclid(26);
                (b'a' + new_offset as u8) as char
            } else if c.is_ascii_uppercase() {
                let offset = c as i32 - b'A' as i32;
                let new_offset = (offset + key as i32).rem_euclid(26);
                (b'A' + new_offset as u8) as char
            } else {
                c
            }
        })
        .collect()
}
