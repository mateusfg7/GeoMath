struct Rhombus {
    s_diagonal: f32,
    l_diagonal: f32,
}

impl Rhombus {
    fn get_area(&self) -> f32 {
        (&self.s_diagonal * &self.l_diagonal) / 2.0
    }
}

pub fn rhombus_actions(s_diagonal: f32, l_diagonal: f32, area: bool) {
    let rhombus = Rhombus {
        l_diagonal,
        s_diagonal,
    };

    if area {
        println!("{}cm", rhombus.get_area())
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
