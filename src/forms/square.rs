pub struct Square {
    pub base: i32,
    pub height: i32,
}

impl Square {
    pub fn get_area(&self) -> i32 {
        self.base * self.height
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let square = super::Square {
            base: 42,
            height: 42,
        };

        assert_eq!(square.get_area(), 1764);
    }
}
