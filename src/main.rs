mod utils;
use utils::format;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod forms;
use forms::circle::Circle;
use forms::rhombus::Rhombus;
use forms::trapezoid::Trapezoid;
use forms::triangle::SidesTriangle;
use forms::triangle::SimpleTriangle;

mod makers;
use makers::square::make_square;
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
    }
    if let Some(matches) = matches.subcommand_matches("circle") {
        circle_actions(matches);
    } else {
        println!(
            "
    To view the help menu type
    geo-math --help
            "
        )
    }
}

fn circle_actions(matches: &ArgMatches) {
    let raio = String::from(matches.value_of("raio").unwrap());

    let circle = make_circle(raio);

    if matches.is_present("area") {
        println!("{}cm", circle.get_area())
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
    let height = String::from(matches.value_of("height").unwrap());

    let square = make_square(base, height);

    if matches.is_present("area") {
        println!("{}cm", square.get_area())
    }
}

fn make_circle(raio: String) -> Circle {
    let formatted_raio_str = format::trim_str(raio);
    let formatted_raio_float = format::str2big_float(formatted_raio_str);

    return Circle {
        raio: formatted_raio_float,
    };
}

fn make_rhombus(l_diagonal: String, s_diagonal: String) -> Rhombus {
    let formatted_larger_diagonal_str = format::trim_str(l_diagonal);
    let formatted_smaller_diagonal_str = format::trim_str(s_diagonal);

    let formatted_larger_diagonal_int = format::str2int(formatted_larger_diagonal_str);
    let formatted_smaller_diagonal_int = format::str2int(formatted_smaller_diagonal_str);

    return Rhombus {
        l_diagonal: formatted_larger_diagonal_int,
        s_diagonal: formatted_smaller_diagonal_int,
    };
}

fn make_trapezoid(l_base: String, s_base: String, height: String) -> Trapezoid {
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

fn make_sides_triangle(side_a: String, side_b: String, side_c: String) -> SidesTriangle {
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

fn make_simple_triangle(base: String, height: String) -> SimpleTriangle {
    let formatted_base_str = format::trim_str(base);
    let formatted_height_str = format::trim_str(height);

    let formatted_base_int = format::str2int(formatted_base_str);
    let formatted_height_int = format::str2int(formatted_height_str);

    return SimpleTriangle {
        base: formatted_base_int,
        height: formatted_height_int,
    };
}
