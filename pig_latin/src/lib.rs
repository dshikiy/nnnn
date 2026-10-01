pub fn pig_latin(text: &str) -> String {
    let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U');

    match text.find(is_vowel) {
        Some(0) => format!("{}ay", text),
        Some(idx) => {
            let mut split_idx = idx;
            // Если перед гласной 'u' стоит 'q', и перед ними есть хотя бы одна согласная
            if split_idx >= 2 && text[..split_idx].ends_with('q') && text[split_idx..].starts_with('u') {
                split_idx += 1;
            }
            format!("{}{}ay", &text[split_idx..], &text[..split_idx])
        }
        None => format!("{}ay", text),
    }
}
