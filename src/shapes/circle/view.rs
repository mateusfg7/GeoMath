use crate::shapes::circle::Circle;

pub fn view(radius: f64) {
    let circle = Circle { radius };

    println!("{}cm", circle.get_area());
}
