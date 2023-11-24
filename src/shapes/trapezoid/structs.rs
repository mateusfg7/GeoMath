pub struct Trapezoid {
    pub s_base: f32,
    pub l_base: f32,
    pub height: f32,
}

impl Trapezoid {
    pub fn get_area(&self) -> f32 {
        ((self.l_base + self.s_base) * self.height) / 2.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let trapezoid = super::Trapezoid {
            s_base: 42.0,
            l_base: 42.0,
            height: 42.0,
        };

        assert_eq!(trapezoid.get_area(), 1764.0);
    }
}
