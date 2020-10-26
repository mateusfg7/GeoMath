use crate::forms::square::Square;
use crate::utils::format;

pub fn make_square(base: String, height: String) -> Square {
    let formatted_base_str = format::trim_str(base);
    let formatted_height_str = format::trim_str(height);

    let formatted_base_int = format::str2int(formatted_base_str);
    let formatted_height_int = format::str2int(formatted_height_str);

    return Square {
        base: formatted_base_int,
        height: formatted_height_int,
    };
}
