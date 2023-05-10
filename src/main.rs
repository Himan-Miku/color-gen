use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random color
    Random,
    /// Generate a gradient color
    Gradient(GradientOptions),
}

#[derive(Args, Debug)]
struct GradientOptions {
    /// color
    #[arg(short = 'c', long = "color")]
    color: Vec<String>,
    /// number of steps
    #[arg(short = 'n', long = "num_steps", default_value_t = 10)]
    num_step: usize,
    /// stops
    #[arg(short = 's', long = "stops")]
    stops: Vec<f32>,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Gradient(grad_ops) => {
            println!("gradient match : {:?}", grad_ops)
        }
        Commands::Random => {
            println!("Random match")
        }
    }
}
