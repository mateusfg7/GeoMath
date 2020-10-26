use crate::forms::rhombus::Rhombus;
use crate::utils::format;

pub fn make_rhombus(l_diagonal: String, s_diagonal: String) -> Rhombus {
    let formatted_larger_diagonal_str = format::trim_str(l_diagonal);
    let formatted_smaller_diagonal_str = format::trim_str(s_diagonal);

    let formatted_larger_diagonal_int = format::str2int(formatted_larger_diagonal_str);
    let formatted_smaller_diagonal_int = format::str2int(formatted_smaller_diagonal_str);

    return Rhombus {
        l_diagonal: formatted_larger_diagonal_int,
        s_diagonal: formatted_smaller_diagonal_int,
    };
}
