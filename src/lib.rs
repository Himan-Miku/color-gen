use clap::{Parser, Subcommand};
pub mod gradient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a random color
    Random,
    /// Generate a gradient color
    Gradient(gradient::GradientOptions),
}
