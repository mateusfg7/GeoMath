use crate::makers::circle::make_circle;
use clap::ArgMatches;

pub fn circle_actions(matches: &ArgMatches) {
    let raio = String::from(matches.value_of("raio").unwrap());

    let circle = make_circle(raio);

    if matches.is_present("area") {
        println!("{}cm", circle.get_area())
    }
}
