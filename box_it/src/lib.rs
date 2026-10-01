pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|part| {
            if let Some(stripped) = part.strip_suffix('k') {
                let val: f64 = stripped.parse().unwrap_or(0.0);
                Box::new((val * 1000.0) as u32)
            } else {
                let val: u32 = part.parse().unwrap_or(0);
                Box::new(val)
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
