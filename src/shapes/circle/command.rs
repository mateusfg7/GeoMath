use clap::Parser;

#[derive(Parser)]
#[command(about="Mathematical operations with circles", long_about = None)]
pub struct Command {
    #[arg(short, long, help = "Sets the radius of the circle | e.g. -r 5")]
    pub radius: f64,
}
