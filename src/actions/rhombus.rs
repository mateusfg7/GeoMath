use crate::makers::rhombus::make_rhombus;
use clap::ArgMatches;

pub fn rhombus_action(matches: &ArgMatches) {
    let l_diagonal = String::from(matches.value_of("lDiagonal").unwrap());
    let s_diagonal = String::from(matches.value_of("sDiagonal").unwrap());

    let rhombus = make_rhombus(l_diagonal, s_diagonal);

    if matches.is_present("area") {
        println!("{}cm", rhombus.get_area())
    }
}
