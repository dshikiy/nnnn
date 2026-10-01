pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            for uppercase_char in c.to_uppercase() {
                result.push(uppercase_char);
            }
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        if c.is_uppercase() {
            for lowercase_char in c.to_lowercase() {
                result.push(lowercase_char);
            }
        } else if c.is_lowercase() {
            for uppercase_char in c.to_uppercase() {
                result.push(uppercase_char);
            }
        } else {
            result.push(c);
        }
    }

    result
}
