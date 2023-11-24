use crate::shapes::square::Square;

pub fn view(base: i32, height: i32) {
    let square = Square { base, height };

    println!("{}cm", square.get_area());
}
