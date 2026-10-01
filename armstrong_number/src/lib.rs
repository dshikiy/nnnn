pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let s = nb.to_string();
    let num_digits = s.len() as u32;

    let sum: u32 = s
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits))
        .sum();

    if sum == nb {
        Some(nb)
    } else {
        None
    }
}
