struct Square {
    base: i32,
    height: i32,
}

impl Square {
    fn get_area(&self) -> i32 {
        self.base * self.height
    }
}

pub fn square_action(area: bool, base: i32, height: i32) {
    let square = Square { base, height };

    if area {
        println!("{}cm", square.get_area());
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
