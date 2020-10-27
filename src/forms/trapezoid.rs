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

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let trapezoid = super::Trapezoid {
            s_base: 42,
            l_base: 42,
            height: 42,
        };

        assert_eq!(trapezoid.get_area(), 1764);
    }
}
