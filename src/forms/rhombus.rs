pub struct Rhombus {
    pub s_diagonal: i32,
    pub l_diagonal: i32,
}

impl Rhombus {
    pub fn get_area(&self) -> i32 {
        (&self.s_diagonal * &self.l_diagonal) / 2
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn get_area() {
        let rhombus = super::Rhombus {
            s_diagonal: 42,
            l_diagonal: 42,
        };

        assert_eq!(rhombus.get_area(), 882);
    }
}
