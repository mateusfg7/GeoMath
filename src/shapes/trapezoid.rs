struct Trapezoid {
    s_base: f32,
    l_base: f32,
    height: f32,
}

impl Trapezoid {
    fn get_area(&self) -> f32 {
        ((&self.l_base + &self.s_base) * &self.height) / 2.0
    }
}

pub fn trapezoid_actions(s_base: f32, l_base: f32, height: f32, area: bool) {
    let trapezoid = Trapezoid {
        height,
        l_base,
        s_base,
    };

    if area {
        println!("Area: {}cm", trapezoid.get_area())
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
