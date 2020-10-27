use crate::makers::trapezoid::make_trapezoid;
use clap::ArgMatches;

pub fn trapezoid_action(matches: &ArgMatches) {
    let l_base = String::from(matches.value_of("lBase").unwrap());
    let s_base = String::from(matches.value_of("sBase").unwrap());
    let height = String::from(matches.value_of("height").unwrap());

    let trapezoid = make_trapezoid(l_base, s_base, height);

    if matches.is_present("area") {
        print!("{}cm", trapezoid.get_area())
    }
}
