pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let len = s.len() as u32;
    
    let sum: u64 = s
        .chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(len))
        .sum();
        
    sum == num as u64
}
