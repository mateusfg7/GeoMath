use crate::forms::circle::Circle;
use crate::utils::format;

pub fn make_circle(raio: String) -> Circle {
    let formatted_raio_str = format::trim_str(raio);
    let formatted_raio_float = format::str2big_float(formatted_raio_str);

    return Circle {
        raio: formatted_raio_float,
    };
}
