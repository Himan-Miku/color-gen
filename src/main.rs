use clap::Parser;
use color_gen::{gradient, Cli, Commands};
use miette::{Context, Result};
use owo_colors::OwoColorize;
use rand::Rng;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gradient(grad_ops) => {
            gradient::generate(grad_ops).wrap_err("While generating gradient")
        }
        Commands::Random => {
            let mut rng = rand::thread_rng();

            let color = owo_colors::Rgb(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );

            let debug_str = "    ";

            print!(
                "{} #{:x}{:x}{:x}",
                debug_str.on_color(color),
                color.0,
                color.1,
                color.2
            );
            Ok(())
        }
    }
}
