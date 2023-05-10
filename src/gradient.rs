use clap::Args;

#[derive(Args, Debug)]
pub struct GradientOptions {
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