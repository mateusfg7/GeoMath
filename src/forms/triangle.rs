pub struct SimpleTriangle {
    pub base: i32,
    pub height: i32,
}

impl SimpleTriangle {
    pub fn get_area(&self) -> i32 {
        (&self.base * &self.height) / 2
    }
}

pub struct SidesTriangle {
    pub side_a: f32,
    pub side_b: f32,
    pub side_c: f32,
}

impl SidesTriangle {
    pub fn get_semi_perimeter(&self) -> f32 {
        (&self.side_a + &self.side_b + &self.side_c) / 2.0
    }

    pub fn get_area(&self) -> f32 {
        let p: f32 = self.get_semi_perimeter();
        let sides_and_p: f32 = p * (p - &self.side_a) * (p - &self.side_b) * (p - &self.side_c);

        return sides_and_p.sqrt();
    }
}

#[cfg(test)]
mod simple_triangle {
    #[test]
    fn get_area() {
        let triangle = super::SimpleTriangle {
            base: 42,
            height: 42,
        };

        assert_eq!(triangle.get_area(), 882);
    }
}

#[cfg(test)]
mod sides_triangle {
    #[test]
    fn get_semi_perimeter() {
        let triangle = super::SidesTriangle {
            side_a: 42.0,
            side_b: 42.0,
            side_c: 42.0,
        };

        assert_eq!(triangle.get_semi_perimeter(), 63.0);
    }

    #[test]
    fn get_area() {
        let triangle = super::SidesTriangle {
            side_a: 42.0,
            side_b: 42.0,
            side_c: 42.0,
        };

        assert_eq!(triangle.get_area(), 763.8344);
    }
}
