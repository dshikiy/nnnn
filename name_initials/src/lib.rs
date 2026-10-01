pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let mut parts = name.split_whitespace();
            let first = parts.next().unwrap_or("");
            let first_char = first.chars().next().unwrap();

            if let Some(second) = parts.next() {
                let second_char = second.chars().next().unwrap();
                format!("{}. {}.", first_char, second_char)
            } else {
                format!("{}.", first_char)
            }
        })
        .collect()
}
