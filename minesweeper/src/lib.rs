pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let rows = minefield.len();
    let cols = minefield[0].len();
    let mut result = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row_str = String::with_capacity(cols);
        for c in 0..cols {
            if minefield[r].as_bytes()[c] == b'*' {
                row_str.push('*');
            } else {
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        
                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            if minefield[nr as usize].as_bytes()[nc as usize] == b'*' {
                                count += 1;
                            }
                        }
                    }
                }
                
                if count > 0 {
                    row_str.push(std::char::from_digit(count, 10).unwrap());
                } else {
                    row_str.push(' ');
                }
            }
        }
        result.push(row_str);
    }

    result
}
