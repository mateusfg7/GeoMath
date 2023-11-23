use clap::Parser;

struct Rhombus {
    s_diagonal: f32,
    l_diagonal: f32,
}

impl Rhombus {
    fn get_area(&self) -> f32 {
        (&self.s_diagonal * &self.l_diagonal) / 2.0
    }
}

pub fn rhombus_actions(s_diagonal: f32, l_diagonal: f32) {
    let rhombus = Rhombus {
        l_diagonal,
        s_diagonal,
    };

    println!("{}cm", rhombus.get_area())
}

#[derive(Parser)]
#[command(about="Mathematical operations with rhombus", long_about = None)]
pub struct Command {
    #[arg(
        short = 'l',
        long = "larger-diagonal",
        help = "Sets the Larger diagonal of the rhombus | e.g. --larger-diagonal 5"
    )]
    pub l_diagonal: f32,
    #[arg(
        short = 's',
        long = "smaller-diagonal",
        help = "Sets the Smaller diagonal of the rhombus | e.g. --smaller-diagonal 5"
    )]
    pub s_diagonal: f32,
}

#[cfg(test)]
mod test {

    #[test]
    fn get_area() {
        let rhombus = super::Rhombus {
            s_diagonal: 42.0,
            l_diagonal: 42.0,
        };

        assert_eq!(rhombus.get_area(), 882.0);
    }
}
