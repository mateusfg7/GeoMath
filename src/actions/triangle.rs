use crate::makers::triangle::*;
use clap::ArgMatches;

pub fn triangle_action(matches: &ArgMatches) {
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
