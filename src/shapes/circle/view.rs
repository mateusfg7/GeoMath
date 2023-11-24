use crate::shapes::circle::Circle;
use crate::tui;

pub fn view(radius: f64) {
    let circle = Circle { radius };

    println!(
        "{}{}",
        tui::title("Area"),
        tui::content(circle.get_area().to_string().as_str())
    );
}
