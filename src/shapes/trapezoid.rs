use clap::Parser;

struct Trapezoid {
    s_base: f32,
    l_base: f32,
    height: f32,
}

impl Trapezoid {
    fn get_area(&self) -> f32 {
        ((&self.l_base + &self.s_base) * &self.height) / 2.0
    }
}

pub fn trapezoid_actions(s_base: f32, l_base: f32, height: f32) {
    let trapezoid = Trapezoid {
        height,
        l_base,
        s_base,
    };

    println!("Area: {}cm", trapezoid.get_area())
}

#[derive(Parser)]
#[command(about="Mathematical operations with trapezoids", long_about = None)]
pub struct Command {
    #[arg(
        short = 'l',
        long = "larger-base",
        help = "Sets the Larger Base of the trapezoid | e.g. --larger-base 5"
    )]
    pub l_base: f32,
    #[arg(
        short = 's',
        long = "smaller-base",
        help = "Sets the Smaller Base of the trapezoid | e.g. --smaller-base 5"
    )]
    pub s_base: f32,
    #[arg(
        short = 'e',
        long,
        help = "Sets the hEight of the trapezoid | e.g. -e 5"
    )]
    pub height: f32,
}

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let trapezoid = super::Trapezoid {
            s_base: 42.0,
            l_base: 42.0,
            height: 42.0,
        };

        assert_eq!(trapezoid.get_area(), 1764.0);
    }
}
