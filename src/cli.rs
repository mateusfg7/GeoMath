use clap::{Parser, Subcommand};

use crate::shapes::*;

#[derive(Parser)]
#[command(name="GeoMath", author, version, about="Geometric calcs", long_about = None, arg_required_else_help=true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Square(square::Command),
    Circle(circle::Command),
    Triangle(triangle::Command),
    Trapezoid(trapezoid::Command),
    Rhombus(rhombus::Command),
}
