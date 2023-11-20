mod cli;
mod shapes;

use clap::Parser;
use cli::{Commands, CLI};
use shapes::*;

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Square { area, base, height }) => {
            square::square_action(*area, *base, *height)
        }
        Some(Commands::Circle { radius, area }) => circle::circle_action(*area, *radius),
        None => {}
    }
}
