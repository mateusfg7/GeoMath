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
        #[arg(short, long, help = "Sets the Base of the form | e.g. -b=5")]
        base: i32,

        #[arg(short = 'e', long, help = "Sets the hEight of the form | e.g. -e=5")]
        height: i32,

        #[arg(short, long, help = "Get tha Area of the form")]
        area: bool,
    },
}
