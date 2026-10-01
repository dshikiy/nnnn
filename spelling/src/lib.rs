pub fn spell(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1..=19 => {
            let units = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
                "eighteen", "nineteen",
            ];
            units[(n - 1) as usize].to_string()
        }
        20..=99 => {
            let tens = [
                "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
            ];
            let ten = tens[((n / 10) - 2) as usize];
            let rem = n % 10;
            if rem == 0 {
                ten.to_string()
            } else {
                format!("{}-{}", ten, spell(rem))
            }
        }
        100..=999 => {
            let rem = n % 100;
            if rem == 0 {
                format!("{} hundred", spell(n / 100))
            } else {
                format!("{} hundred {}", spell(n / 100), spell(rem))
            }
        }
        1000..=999_999 => {
            let rem = n % 1000;
            if rem == 0 {
                format!("{} thousand", spell(n / 1000))
            } else {
                format!("{} thousand {}", spell(n / 1000), spell(rem))
            }
        }
        1_000_000 => String::from("one million"),
        _ => String::new(),
    }
}
