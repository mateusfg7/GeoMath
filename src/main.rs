#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod actions;
mod forms;
mod makers;
mod utils;

use makers::circle::make_circle;
use makers::rhombus::make_rhombus;

use actions::square::square_action;
use actions::trapezoid::trapezoid_action;
use actions::triangle::triangle_action;

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
