pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_ascii_digit())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0)
    });

    words
        .into_iter()
        .map(|word| {
            word.chars()
                .filter(|c| !c.is_ascii_digit())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
