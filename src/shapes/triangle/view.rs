use crate::{
    shapes::triangle::{SidesTriangle, SimpleTriangle},
    tui,
};

pub fn view(
    base: Option<f32>,
    height: Option<f32>,

    side_a: Option<f32>,
    side_b: Option<f32>,
    side_c: Option<f32>,
) {
    if let (Some(base), Some(height)) = (base, height) {
        let triangle = SimpleTriangle { base, height };

        println!(
            "{}{}",
            tui::title("Area"),
            tui::content(triangle.get_area().to_string().as_str())
        );
    } else if let (Some(side_a), Some(side_b), Some(side_c)) = (side_a, side_b, side_c) {
        let triangle = SidesTriangle {
            side_a,
            side_b,
            side_c,
        };

        println!(
            "{}{}",
            tui::title("Area"),
            tui::content(triangle.get_area().to_string().as_str())
        );
        print!("\n");
        println!(
            "{}{}",
            tui::title("Perimeter"),
            tui::content(triangle.get_semi_perimeter().to_string().as_str())
        );
    }
}
