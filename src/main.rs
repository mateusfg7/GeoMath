#[macro_use]
extern crate clap;
use clap::App;

mod actions;
mod forms;
mod makers;
mod utils;

use actions::circle::circle_actions;
use actions::rhombus::rhombus_action;
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
