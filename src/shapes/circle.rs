struct Circle {
    radius: f64,
}

impl Circle {
    fn get_area(&self) -> f64 {
        let pi = std::f64::consts::PI;

        return &self.radius * pi.powi(2);
    }
}

pub fn circle_action(area: bool, radius: f64) {
    let circle = Circle { radius };

    if area {
        println!("{}cm", circle.get_area());
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let circle = super::Circle { radius: 42.0 };

        assert_eq!(circle.get_area(), 414.523384845753);
    }
}
