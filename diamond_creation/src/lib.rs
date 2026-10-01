pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let mut diamond = Vec::with_capacity(2 * n + 1);

    for i in 0..=(2 * n) {
        let idx = if i <= n { i } else { 2 * n - i };
        let letter = (b'A' + idx as u8) as char;
        let outer = " ".repeat(n - idx);
        
        if idx == 0 {
            diamond.push(format!("{}A{}", outer, outer));
        } else {
            let inner = " ".repeat(2 * idx - 1);
            diamond.push(format!("{}{}{}{}{}", outer, letter, inner, letter, outer));
        }
    }

    diamond
}
