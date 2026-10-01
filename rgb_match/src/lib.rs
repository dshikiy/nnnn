#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let swap_val = |val| match val {
            v if v == first => second,
            v if v == second => first,
            v => v,
        };

        self.r = swap_val(self.r);
        self.g = swap_val(self.g);
        self.b = swap_val(self.b);
        self.a = swap_val(self.a);

        self
    }
}