mod utils;
use utils::format;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod forms;
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
    } else {
        println!(
            "
    To view the help menu type 
    geo-math --help
            "
        )
    }
}

fn trapezoid_action(matches: &ArgMatches) {
    let lBase = String::from(matches.value_of("lBase").unwrap());
    let sBase = String::from(matches.value_of("sBase").unwrap());
    let height = String::from(matches.value_of("height").unwrap());

    let trapezoid = make_trapezoid(lBase, sBase, height);

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

fn make_trapezoid(lBase: String, sBase: String, height: String) -> Trapezoid {
    let formated_lBase_str = format::trim_str(lBase);
    let formated_sBase_str = format::trim_str(sBase);
    let formated_height_str = format::trim_str(height);

    let formated_lBase_int = format::str2int(formated_lBase_str);
    let formated_sBase_int = format::str2int(formated_sBase_str);
    let formated_height_int = format::str2int(formated_height_str);

    return Trapezoid {
        lBase: formated_lBase_int,
        sBase: formated_sBase_int,
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
