use crate::shapes::circle::Circle;

pub fn circle_action(radius: f64) {
    let circle = Circle { radius };

    println!("{}cm", circle.get_area());
}
