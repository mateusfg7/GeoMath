use crate::shapes::trapezoid::Trapezoid;

pub fn view(s_base: f32, l_base: f32, height: f32) {
    let trapezoid = Trapezoid {
        height,
        l_base,
        s_base,
    };

    println!("Area: {}cm", trapezoid.get_area())
}
