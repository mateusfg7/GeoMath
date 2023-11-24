use crate::{shapes::trapezoid::Trapezoid, tui};

pub fn view(s_base: f32, l_base: f32, height: f32) {
    let trapezoid = Trapezoid {
        height,
        l_base,
        s_base,
    };

    println!(
        "{}{}",
        tui::title("Area"),
        tui::content(trapezoid.get_area().to_string().as_str())
    )
}
