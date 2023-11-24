use crate::{shapes::square::Square, tui};

pub fn view(base: i32, height: i32) {
    let square = Square { base, height };

    println!(
        "{}{}",
        tui::title("Area"),
        tui::content(square.get_area().to_string().as_str())
    );
}
