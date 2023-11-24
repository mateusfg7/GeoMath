pub struct Rhombus {
    pub s_diagonal: f32,
    pub l_diagonal: f32,
}

impl Rhombus {
    pub fn get_area(&self) -> f32 {
        (&self.s_diagonal * &self.l_diagonal) / 2.0
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn get_area() {
        let rhombus = super::Rhombus {
            s_diagonal: 42.0,
            l_diagonal: 42.0,
        };

        assert_eq!(rhombus.get_area(), 882.0);
    }
}
