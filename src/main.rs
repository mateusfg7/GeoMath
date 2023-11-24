mod cli;
use cli::{Commands, CLI};

mod shapes;
use shapes::*;

use clap::Parser;

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Square(square::Command { base, height })) => square::view(*base, *height),
        Some(Commands::Circle(circle::Command { radius })) => circle::view(*radius),
        Some(Commands::Triangle(triangle::Command {
            base,
            height,
            side_a,
            side_b,
            side_c,
        })) => triangle::view(*base, *height, *side_a, *side_b, *side_c),
        Some(Commands::Trapezoid(trapezoid::Command {
            l_base,
            s_base,
            height,
        })) => trapezoid::view(*s_base, *l_base, *height),
        Some(Commands::Rhombus(rhombus::Command {
            l_diagonal,
            s_diagonal,
        })) => rhombus::view(*s_diagonal, *l_diagonal),
        None => {}
    }
}
