use utils::format;
use utils::interact;

mod utils;

fn main() {
    println!("Base:");
    let user_base = interact::read_input();

    println!("Height:");
    let user_height = interact::read_input();

    let formated_base_str = format::trim_str(user_base);
    let formated_height_str = format::trim_str(user_height);

    let base = format::str2int(formated_base_str);
    let height = format::str2int(formated_height_str);

    let area = base * height;

    println!("{}cm", area);
}
