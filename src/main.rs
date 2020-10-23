mod utils;
use utils::format;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod forms;
use forms::rhombus::Rhombus;
use forms::square::Square;
use forms::trapezoid::Trapezoid;
use forms::triangle::SidesTriangle;
use forms::triangle::SimpleTriangle;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("doc") {
        println!("https://github.com/mateusfg7/GeoMath")
    }

    if let Some(matches) = matches.subcommand_matches("square") {
        square_action(matches);
    }
    if let Some(matches) = matches.subcommand_matches("rectangle") {
        square_action(matches);
    }
    if let Some(matches) = matches.subcommand_matches("parallelogram") {
        square_action(matches);
    }
    if let Some(matches) = matches.subcommand_matches("triangle") {
        triangle_action(matches);
    }
    if let Some(matches) = matches.subcommand_matches("trapezoid") {
        trapezoid_action(matches);
    }
    if let Some(matches) = matches.subcommand_matches("rhombus") {
        rhombus_action(matches);
    } else {
        println!(
            "
    To view the help menu type 
    geo-math --help
            "
        )
    }
}

fn rhombus_action(matches: &ArgMatches) {
    let l_diagonal = String::from(matches.value_of("lDiagonal").unwrap());
    let s_diagonal = String::from(matches.value_of("sDiagonal").unwrap());

    let rhombus = make_rhombus(l_diagonal, s_diagonal);

    if matches.is_present("area") {
        println!("{}cm", rhombus.get_area())
    }
}

fn trapezoid_action(matches: &ArgMatches) {
    let l_base = String::from(matches.value_of("lBase").unwrap());
    let s_base = String::from(matches.value_of("sBase").unwrap());
    let height = String::from(matches.value_of("height").unwrap());

    let trapezoid = make_trapezoid(l_base, s_base, height);

    if matches.is_present("area") {
        print!("{}cm", trapezoid.get_area())
    }
}

fn triangle_action(matches: &ArgMatches) {
    if matches.is_present("base") && matches.is_present("height") {
        let base = String::from(matches.value_of("base").unwrap());
        let height = String::from(matches.value_of("height").unwrap());

        let triangle = make_simple_triangle(base, height);

        if matches.is_present("area") {
            println!("{}cm", triangle.get_area())
        }
    } else {
        let side_a = String::from(matches.value_of("side-a").unwrap());
        let side_b = String::from(matches.value_of("side-b").unwrap());
        let side_c = String::from(matches.value_of("side-c").unwrap());

        let triangle = make_sides_triangle(side_a, side_b, side_c);

        if matches.is_present("area") {
            println!("{}cm", triangle.get_area())
        }

        if matches.is_present("perimeter") {
            println!("{}cm", triangle.get_semi_perimeter())
        }
    }
}

fn square_action(matches: &ArgMatches) {
    let base = String::from(matches.value_of("base").unwrap());
    let heigth = String::from(matches.value_of("height").unwrap());

    let square = make_square(base, heigth);

    if matches.is_present("area") {
        println!("{}cm", square.get_area())
    }
}

fn make_rhombus(l_diagonal: String, s_diagonal: String) -> Rhombus {
    let formated_larger_diagonal_str = format::trim_str(l_diagonal);
    let formated_smaller_diagonal_str = format::trim_str(s_diagonal);

    let formated_larger_diagonal_int = format::str2int(formated_larger_diagonal_str);
    let formated_smaller_diagonal_int = format::str2int(formated_smaller_diagonal_str);

    return Rhombus {
        l_diagonal: formated_larger_diagonal_int,
        s_diagonal: formated_smaller_diagonal_int,
    };
}

fn make_trapezoid(l_base: String, s_base: String, height: String) -> Trapezoid {
    let formated_larger_base_str = format::trim_str(l_base);
    let formated_smaller_base_str = format::trim_str(s_base);
    let formated_height_str = format::trim_str(height);

    let formated_larger_base_int = format::str2int(formated_larger_base_str);
    let formated_smaller_base_int = format::str2int(formated_smaller_base_str);
    let formated_height_int = format::str2int(formated_height_str);

    return Trapezoid {
        l_base: formated_larger_base_int,
        s_base: formated_smaller_base_int,
        height: formated_height_int,
    };
}

fn make_sides_triangle(side_a: String, side_b: String, side_c: String) -> SidesTriangle {
    let formated_side_a_str = format::trim_str(side_a);
    let formated_side_b_str = format::trim_str(side_b);
    let formated_side_c_str = format::trim_str(side_c);

    let formated_side_a_float = format::str2float(formated_side_a_str);
    let formated_side_b_float = format::str2float(formated_side_b_str);
    let formated_side_c_float = format::str2float(formated_side_c_str);

    return SidesTriangle {
        side_a: formated_side_a_float,
        side_b: formated_side_b_float,
        side_c: formated_side_c_float,
    };
}

fn make_simple_triangle(base: String, height: String) -> SimpleTriangle {
    let formated_base_str = format::trim_str(base);
    let formated_height_str = format::trim_str(height);

    let formated_base_int = format::str2int(formated_base_str);
    let formated_height_int = format::str2int(formated_height_str);

    return SimpleTriangle {
        base: formated_base_int,
        height: formated_height_int,
    };
}

fn make_square(base: String, height: String) -> Square {
    let formated_base_str = format::trim_str(base);
    let formated_height_str = format::trim_str(height);

    let formated_base_int = format::str2int(formated_base_str);
    let formated_height_int = format::str2int(formated_height_str);

    return Square {
        base: formated_base_int,
        height: formated_height_int,
    };
}
