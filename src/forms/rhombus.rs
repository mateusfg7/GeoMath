pub struct Rhombus {
    pub s_diagonal: i32,
    pub l_diagonal: i32,
}

impl Rhombus {
    pub fn get_area(&self) -> i32 {
        (&self.s_diagonal * &self.l_diagonal) / 2
    }
}
