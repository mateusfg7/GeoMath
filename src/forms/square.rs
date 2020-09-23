pub struct Square {
    pub base: i32,
    pub height: i32,
}

impl Square {
    pub fn get_area(&self) -> i32 {
        self.base * self.height
    }
}
