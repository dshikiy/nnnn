pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let val = c as f64;
    (c, val.exp(), val.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exp_str = a
        .split_whitespace()
        .map(|s| {
            let n: f64 = s.parse().unwrap();
            n.exp().to_string()
        })
        .collect::<Vec<_>>()
        .join(" ");

    (a, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b
        .iter()
        .map(|&x| (x as f64).abs().ln())
        .collect();

    (b, logs)
}
