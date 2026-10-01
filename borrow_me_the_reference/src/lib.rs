pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::with_capacity(s.len());
    let mut skip = 0;

    for c in s.chars() {
        if c == '+' {
            skip += 1;
        } else if skip > 0 {
            skip -= 1;
        } else {
            res.push(c);
        }
    }

    let mut final_res = String::with_capacity(res.len());
    for c in res.chars() {
        if c == '-' {
            final_res.pop();
        } else {
            final_res.push(c);
        }
    }

    *s = final_res;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(pos) = s.find('+') {
            let num1: i32 = s[..pos].parse().unwrap();
            let num2: i32 = s[pos + 1..].parse().unwrap();
            *s = (num1 + num2).to_string();
        } else if let Some(pos) = s.find('-') {
            let num1: i32 = s[..pos].parse().unwrap();
            let num2: i32 = s[pos + 1..].parse().unwrap();
            *s = (num1 - num2).to_string();
        }
    }
}
