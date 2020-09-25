mod utils;
use utils::format;

#[macro_use]
extern crate clap;
use clap::App;
use clap::ArgMatches;

mod forms;
use forms::square::Square;

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
        
    } else {
        println!(
            "
    To view the help menu type 
    geo-math --help
            "
        )
    }
}

fn triangle_action

fn square_action(matches: &ArgMatches) {
    let base = String::from(matches.value_of("base").unwrap());
    let heigth = String::from(matches.value_of("height").unwrap());

    let square = make_square(base, heigth);

    if matches.is_present("area") {
        println!("{}cm", square.get_area())
    }
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
