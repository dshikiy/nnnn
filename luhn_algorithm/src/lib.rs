pub fn is_luhn_formula(code: &str) -> bool {
    let clean_code = code.replace(' ', "");
    
    if clean_code.len() <= 1 {
        return false;
    }

    let mut sum = 0;
    let mut is_second = false;

    for c in clean_code.chars().rev() {
        if let Some(mut digit) = c.to_digit(10) {
            if is_second {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            sum += digit;
            is_second = !is_second;
        } else {
            return false;
        }
    }

    sum % 10 == 0
}
