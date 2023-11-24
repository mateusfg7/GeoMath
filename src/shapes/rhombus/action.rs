use crate::shapes::rhombus::Rhombus;

pub fn rhombus_actions(s_diagonal: f32, l_diagonal: f32) {
    let rhombus = Rhombus {
        l_diagonal,
        s_diagonal,
    };

    println!("{}cm", rhombus.get_area())
}
