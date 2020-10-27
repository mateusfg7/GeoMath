use crate::makers::square::make_square;
use clap::ArgMatches;

pub fn square_action(matches: &ArgMatches) {
    let base = String::from(matches.value_of("base").unwrap());
    let height = String::from(matches.value_of("height").unwrap());

    let square = make_square(base, height);

    if matches.is_present("area") {
        println!("{}cm", square.get_area())
    }
}
