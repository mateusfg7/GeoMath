use crate::shapes::triangle::{SidesTriangle, SimpleTriangle};

pub fn triangle_actions(
    base: Option<f32>,
    height: Option<f32>,

    side_a: Option<f32>,
    side_b: Option<f32>,
    side_c: Option<f32>,
) {
    if let (Some(base), Some(height)) = (base, height) {
        let triangle = SimpleTriangle { base, height };

        println!("Area: {}cm", triangle.get_area());
    } else if let (Some(side_a), Some(side_b), Some(side_c)) = (side_a, side_b, side_c) {
        let triangle = SidesTriangle {
            side_a,
            side_b,
            side_c,
        };

        println!("Area: {}cm", triangle.get_area());
        println!("Perimeter: {}cm", triangle.get_semi_perimeter());
    }
}
