pub struct Trapezoid {
    pub sBase: i32,
    pub lBase: i32,
    pub height: i32,
}

impl Trapezoid {
    pub fn get_area(&self) -> i32 {
        ((&self.lBase + &self.sBase) * &self.height) / 2
    }
}
