pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exps = a
        .split_whitespace()
        .map(|s| {
            let n: f64 = s.parse().unwrap();
            n.exp().to_string()
        })
        .collect::<Vec<String>>()
        .join(" ");
    
    (a, exps)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let logs = b
        .iter()
        .map(|&x| (x.abs() as f64).ln())
        .collect();
        
    (b, logs)
}
