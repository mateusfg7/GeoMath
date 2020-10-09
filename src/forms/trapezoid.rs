pub struct Trapezoid {
    pub s_base: i32,
    pub l_base: i32,
    pub height: i32,
}

impl Trapezoid {
    pub fn get_area(&self) -> i32 {
        ((&self.l_base + &self.s_base) * &self.height) / 2
    }
}
