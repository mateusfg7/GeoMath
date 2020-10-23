pub struct Circle {
    pub raio: f64,
}

impl Circle {
    pub fn get_area(&self) -> f64 {
        let pi = std::f64::consts::PI;

        return &self.raio * pi.powi(2);
    }
}
