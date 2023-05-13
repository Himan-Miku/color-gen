use std::{iter::zip, str::FromStr};

use clap::Args;
use owo_colors::OwoColorize;
use palette::{rgb::Rgb, Gradient, LinSrgb};
use rand::Rng;

#[derive(Args, Debug)]
pub struct GradientOptions {
    /// color
    #[arg(short = 'c', long = "color")]
    colors: Vec<String>,
    /// number of steps
    #[arg(short = 'n', long = "num_steps", default_value_t = 10)]
    num_step: usize,
    /// stops
    #[arg(short = 's', long = "stops")]
    stops: Vec<f32>,
}

pub fn generate(
    GradientOptions {
        colors,
        num_step,
        stops,
    }: &GradientOptions,
) {
    if colors.len() > 0 {
        if colors.len() != stops.len() {
            panic!("The number of colors and stops is not same:\nNumber of colors: {:?}\nNumber of stops: {:?}",colors, stops)
        }

        let color_list = zip(stops, colors)
            .map(|(&stop, color)| {
                (
                    stop,
                    LinSrgb::from_str(color)
                        .expect("Hex code required")
                        .into_format(),
                )
            })
            .collect::<Vec<(f32, LinSrgb)>>();

        let gradient = Gradient::with_domain(color_list);

        let taken_colors: Vec<_> = gradient
            .take(*num_step)
            .map(
                |Rgb {
                     red,
                     green,
                     blue,
                     standard: _,
                 }| {
                    owo_colors::Rgb(
                        (red * 255.0) as u8,
                        (green * 255.0) as u8,
                        (blue * 255.0) as u8,
                    )
                },
            )
            .collect();

        for color in taken_colors.iter() {
            let debug_str = "    ";

            print!("{}", debug_str.on_color(*color))
        }
    } else {
        let mut rng = rand::thread_rng();

        let gradient = Gradient::new(vec![
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        ]);

        let taken_colors: Vec<_> = gradient
            .take(*num_step)
            .map(
                |Rgb {
                     red,
                     green,
                     blue,
                     standard: _,
                 }| {
                    owo_colors::Rgb(
                        (red * 255.0) as u8,
                        (green * 255.0) as u8,
                        (blue * 255.0) as u8,
                    )
                },
            )
            .collect();

        for color in taken_colors.iter() {
            let debug_str = "    ";

            print!("{}", debug_str.on_color(*color))
        }
    }
}
