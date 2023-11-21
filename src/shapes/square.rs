use clap::Parser;

struct Square {
    base: i32,
    height: i32,
}

impl Square {
    fn get_area(&self) -> i32 {
        self.base * self.height
    }
}

pub fn square_action(area: bool, base: i32, height: i32) {
    let square = Square { base, height };

    if area {
        println!("{}cm", square.get_area());
    }
}

#[derive(Parser)]
#[command(about="Mathematical operations with squares", long_about = None)]
pub struct Command {
    #[arg(short, long, help = "Sets the Base of the square | e.g. -b 5")]
    pub base: i32,

    #[arg(short = 'e', long, help = "Sets the hEight of the square | e.g. -e 5")]
    pub height: i32,

    #[arg(short, long, help = "Get the Area of the square")]
    pub area: bool,
}

#[cfg(test)]
mod test {
    #[test]
    fn get_area() {
        let square = super::Square {
            base: 42,
            height: 42,
        };

        assert_eq!(square.get_area(), 1764);
    }
}
