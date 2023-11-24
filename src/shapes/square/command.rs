use clap::Parser;

#[derive(Parser)]
#[command(about="Mathematical operations with squares", long_about = None, arg_required_else_help=true)]
pub struct Command {
    #[arg(short, long, help = "Sets the Base of the square | e.g. -b 5")]
    pub base: i32,

    #[arg(short = 'e', long, help = "Sets the hEight of the square | e.g. -e 5")]
    pub height: i32,
}
