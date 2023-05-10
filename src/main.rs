use clap::Parser;
use color_gen::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gradient(grad_ops) => {
            println!("gradient match : {:?}", grad_ops)
        }
        Commands::Random => {
            println!("Random match")
        }
    }
}
