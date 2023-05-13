use clap::{Parser, Subcommand};
use miette::Diagnostic;
use thiserror::Error;

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

#[derive(Debug, Diagnostic, Error)]
pub enum ColorGenerationError {
    #[error(transparent)]
    #[diagnostic(code(color_gen::io_error))]
    IoError(#[from] std::io::Error),

    #[error("number of colors and number of steps mismatch")]
    #[diagnostic(code(color_gen::colors_and_steps_must_match))]
    ColorsAndStepsMustMatch {
        #[source_code]
        input: String,
        #[help]
        advice: String,
        #[label("colors")]
        color_src: (usize, usize),
        #[label("Steps")]
        steps_src: (usize, usize),
    },
}
