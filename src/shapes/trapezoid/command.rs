use clap::Parser;

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
