mod cli;
mod shapes;

use clap::Parser;
use cli::{Commands, CLI};
use shapes::*;

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Square(square::Command { base, height })) => {
            square::square_action(*base, *height)
        }
        Some(Commands::Circle(circle::Command { radius })) => circle::circle_action(*radius),
        Some(Commands::Triangle(triangle::Command {
            base,
            height,
            side_a,
            side_b,
            side_c,
        })) => triangle::triangle_actions(*base, *height, *side_a, *side_b, *side_c),
        Some(Commands::Trapezoid(trapezoid::Command {
            l_base,
            s_base,
            height,
        })) => trapezoid::trapezoid_actions(*s_base, *l_base, *height),
        Some(Commands::Rhombus(rhombus::Command {
            l_diagonal,
            s_diagonal,
        })) => rhombus::rhombus_actions(*s_diagonal, *l_diagonal),
        None => {}
    }
}
