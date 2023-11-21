mod cli;
mod shapes;

use clap::Parser;
use cli::{Commands, CLI};
use shapes::*;

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Square(square::Command { area, base, height })) => {
            square::square_action(*area, *base, *height)
        }
        Some(Commands::Circle(circle::Command { radius, area })) => {
            circle::circle_action(*area, *radius)
        }
        Some(Commands::Triangle(triangle::Command {
            base,
            height,
            side_a,
            side_b,
            side_c,
            area,
            perimeter,
        })) => {
            triangle::triangle_actions(*base, *height, *side_a, *side_b, *side_c, *area, *perimeter)
        }
        Some(Commands::Trapezoid(trapezoid::Command {
            l_base,
            s_base,
            height,
            area,
        })) => trapezoid::trapezoid_actions(*s_base, *l_base, *height, *area),
        Some(Commands::Rhombus(rhombus::Command {
            l_diagonal,
            s_diagonal,
            area,
        })) => rhombus::rhombus_actions(*s_diagonal, *l_diagonal, *area),
        None => {}
    }
}
