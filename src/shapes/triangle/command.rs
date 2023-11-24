use clap::Parser;

#[derive(Parser)]
#[command(about="Mathematical operations with triangles", long_about = None)]
pub struct Command {
    #[arg(short, long, help = "Sets the Base of the triangle | e.g. -b 5")]
    pub base: Option<f32>,
    #[arg(
        short = 'e',
        long,
        help = "Sets the hEight of the triangle | e.g. -e 5"
    )]
    pub height: Option<f32>,

    #[arg(short = None, long="side-a", help = "Sets the side A of the triangle | e.g. --side-a 5")]
    pub side_a: Option<f32>,
    #[arg(short = None, long="side-b", help = "Sets the side B of the triangle | e.g. --side-b 5")]
    pub side_b: Option<f32>,
    #[arg(short = None, long="side-c", help = "Sets the side c of the triangle | e.g. --side-c 5")]
    pub side_c: Option<f32>,
}
