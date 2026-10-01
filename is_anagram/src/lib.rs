pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().flat_map(|c| c.to_lowercase()).collect();
    let mut chars2: Vec<char> = s2.chars().flat_map(|c| c.to_lowercase()).collect();

    chars1.sort_unstable();
    chars2.sort_unstable();

    chars1 == chars2
}
