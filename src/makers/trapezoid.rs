use crate::forms::trapezoid::Trapezoid;
use crate::utils::format;

pub fn make_trapezoid(l_base: String, s_base: String, height: String) -> Trapezoid {
    let formatted_larger_base_str = format::trim_str(l_base);
    let formatted_smaller_base_str = format::trim_str(s_base);
    let formatted_height_str = format::trim_str(height);

    let formatted_larger_base_int = format::str2int(formatted_larger_base_str);
    let formatted_smaller_base_int = format::str2int(formatted_smaller_base_str);
    let formatted_height_int = format::str2int(formatted_height_str);

    return Trapezoid {
        l_base: formatted_larger_base_int,
        s_base: formatted_smaller_base_int,
        height: formatted_height_int,
    };
}
