use clap::Parser;

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
