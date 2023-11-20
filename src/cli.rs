use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="GeoMath", author, version, about="Geometric calcs", long_about = None, arg_required_else_help=true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about="Mathematical operations with squares", long_about = None)]
    Square {
        #[arg(short, long, help = "Sets the Base of the square | e.g. -b 5")]
        base: i32,

        #[arg(short = 'e', long, help = "Sets the hEight of the square | e.g. -e 5")]
        height: i32,

        #[arg(short, long, help = "Get the Area of the square")]
        area: bool,
    },
    #[command(about="Mathematical operations with circles", long_about = None)]
    Circle {
        #[arg(short, long, help = "Sets the radius of the circle | e.g. -r 5")]
        radius: f64,

        #[arg(short, long, help = "Get the Area of the circle")]
        area: bool,
    },
    #[command(about="Mathematical operations with triangles", long_about = None)]
    Triangle {
        #[arg(short, long, help = "Sets the Base of the triangle | e.g. -b 5")]
        base: Option<f32>,
        #[arg(
            short = 'e',
            long,
            help = "Sets the hEight of the triangle | e.g. -e 5"
        )]
        height: Option<f32>,

        #[arg(short = None, long="side-a", help = "Sets the side A of the triangle | e.g. --side-a 5")]
        side_a: Option<f32>,
        #[arg(short = None, long="side-b", help = "Sets the side B of the triangle | e.g. --side-b 5")]
        side_b: Option<f32>,
        #[arg(short = None, long="side-c", help = "Sets the side c of the triangle | e.g. --side-c 5")]
        side_c: Option<f32>,

        #[arg(short, long, help = "Get the Area of the triangle")]
        area: bool,
        #[arg(short, long, help = "Get the Perimeter of the triangle")]
        perimeter: bool,
    },
}
