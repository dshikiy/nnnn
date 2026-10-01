use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let first_char = name.chars().next().unwrap_or_default();
        Self {
            short_hand: format!("-{}", first_char),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let func = self.flags.get(input).ok_or("flag not found")?;
        let a = argv.get(0).unwrap_or(&"");
        let b = argv.get(1).unwrap_or(&"");

        func(a, b).map_err(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num_a = a.parse::<f64>()?;
    let num_b = b.parse::<f64>()?;
    Ok((num_a / num_b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num_a = a.parse::<f64>()?;
    let num_b = b.parse::<f64>()?;
    Ok((num_a % num_b).to_string())
}
