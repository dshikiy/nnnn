pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 || message.is_empty() {
        return message.to_string();
    }

    let chars: Vec<char> = message.chars().collect();
    let remainder = chars.len() % i;
    
    let mut padded = chars.clone();
    if remainder != 0 {
        padded.resize(chars.len() + (i - remainder), ' ');
    }

    let num_rows = padded.len() / i;
    let mut result = String::with_capacity(padded.len());

    for col in 0..i {
        for row in 0..num_rows {
            result.push(padded[row * i + col]);
        }
    }

    result.trim_end().to_string()
}
