pub fn invert_sentence(string: &str) -> String {
    let mut words = Vec::new();
    let mut whitespaces = Vec::new();

    let mut current = String::new();
    let mut is_space = None;

    for c in string.chars() {
        let char_is_space = c.is_whitespace();
        match is_space {
            Some(space_flag) if space_flag == char_is_space => {
                current.push(c);
            }
            Some(space_flag) => {
                if space_flag {
                    whitespaces.push(current);
                } else {
                    words.push(current);
                }
                current = c.to_string();
                is_space = Some(char_is_space);
            }
            None => {
                current.push(c);
                is_space = Some(char_is_space);
            }
        }
    }

    if let Some(space_flag) = is_space {
        if space_flag {
            whitespaces.push(current);
        } else {
            words.push(current);
        }
    }

    words.reverse();

    let starts_with_space = string.chars().next().map_or(false, |c| c.is_whitespace());
    let mut result = String::new();

    let mut word_idx = 0;
    let mut space_idx = 0;

    let total_tokens = words.len() + whitespaces.len();

    for i in 0..total_tokens {
        if (i % 2 == 0) == starts_with_space {
            if space_idx < whitespaces.len() {
                result.push_str(&whitespaces[space_idx]);
                space_idx += 1;
            }
        } else {
            if word_idx < words.len() {
                result.push_str(&words[word_idx]);
                word_idx += 1;
            }
        }
    }

    result
}
