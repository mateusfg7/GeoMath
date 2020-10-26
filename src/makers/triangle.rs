use crate::forms::triangle::{SidesTriangle, SimpleTriangle};
use crate::utils::format;

pub fn make_simple_triangle(base: String, height: String) -> SimpleTriangle {
    let formatted_base_str = format::trim_str(base);
    let formatted_height_str = format::trim_str(height);

    let formatted_base_int = format::str2int(formatted_base_str);
    let formatted_height_int = format::str2int(formatted_height_str);

    return SimpleTriangle {
        base: formatted_base_int,
        height: formatted_height_int,
    };
}

pub fn make_sides_triangle(side_a: String, side_b: String, side_c: String) -> SidesTriangle {
    let formatted_side_a_str = format::trim_str(side_a);
    let formatted_side_b_str = format::trim_str(side_b);
    let formatted_side_c_str = format::trim_str(side_c);

    let formatted_side_a_float = format::str2float(formatted_side_a_str);
    let formatted_side_b_float = format::str2float(formatted_side_b_str);
    let formatted_side_c_float = format::str2float(formatted_side_c_str);

    return SidesTriangle {
        side_a: formatted_side_a_float,
        side_b: formatted_side_b_float,
        side_c: formatted_side_c_float,
    };
}
