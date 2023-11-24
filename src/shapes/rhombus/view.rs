use crate::{shapes::rhombus::Rhombus, tui};

pub fn view(s_diagonal: f32, l_diagonal: f32) {
    let rhombus = Rhombus {
        l_diagonal,
        s_diagonal,
    };

    println!(
        "{}{}",
        tui::title("Area"),
        tui::content(rhombus.get_area().to_string().as_str())
    )
}
