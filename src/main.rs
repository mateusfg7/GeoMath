mod utils;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod forms;

mod makers;
use makers::circle::make_circle;
use makers::rhombus::make_rhombus;
use makers::square::make_square;
use makers::trapezoid::make_trapezoid;
use makers::triangle::{make_sides_triangle, make_simple_triangle};

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
