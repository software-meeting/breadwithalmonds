use bmp::{px, Image, Pixel};
use num::complex::{Complex64, ComplexFloat};
use rayon::prelude::*;

const REPS: u32 = 512;
const BOUND: f64 = 255.0;
const IMG_SIZE: u32 = 16258;
const N_PIXELS: usize = (IMG_SIZE * IMG_SIZE) as usize;

fn main() {
    let mut image = Image::new(IMG_SIZE, IMG_SIZE);

    let pixels: Vec<Pixel> = (0..N_PIXELS)
        .into_par_iter()
        .map(|n| {
            let y = n as u32 / IMG_SIZE;
            let x = n as u32 % IMG_SIZE;
            let a: f64 = -0.71875 + (x as f64) * 0.09375 / (IMG_SIZE as f64);
            let b: f64 = 0.28125 + (y as f64) * 0.09375 / (IMG_SIZE as f64);

            let mut z: Complex64 = Complex64::new(0.0, 0.0);
            let c: Complex64 = Complex64::new(a, b);

            let mut val: u32 = 0;

            for n in 0..REPS {
                if z.abs() >= BOUND {
                    val = n;
                    break;
                } else {
                    z = z.powi(2) + c;
                }
            }

            u32_to_rgb_spectrum(val)
        })
        .collect();

    for (n, pixel) in pixels.iter().enumerate() {
        let y = n as u32 / IMG_SIZE;
        let x = n as u32 % IMG_SIZE;
        image.set_pixel(x, y, *pixel);
    }

    image.save("out.bmp").unwrap();
}

// TODO: Review generated code
fn u32_to_rgb_spectrum(input: u32) -> Pixel {
    // Define the spectrum "points".  We'll interpolate between them.
    let spectrum_points: [Pixel; 6] = [
        px!(0, 0, 0),
        Pixel::new(255, 0, 0),   // Red
        Pixel::new(255, 255, 0), // Yellow
        Pixel::new(0, 255, 0),   // Green
        Pixel::new(0, 255, 255), // Cyan
        Pixel::new(0, 0, 255),   // Blue
    ];

    let max_input = REPS;

    // Calculate the normalized position (0.0 to 1.0)
    let normalized_position = input as f64 / max_input as f64;

    // Determine which segment of the spectrum we're in
    let segment_index = (normalized_position * (spectrum_points.len() - 1) as f64).floor() as usize;

    // Handle edge cases (important!)
    if segment_index >= spectrum_points.len() - 1 {
        return spectrum_points[spectrum_points.len() - 1]; // Return last color if input is at/near max
    }

    let start_color = spectrum_points[segment_index];
    let end_color = spectrum_points[segment_index + 1];

    // Calculate the interpolation factor within the segment
    let segment_position = (normalized_position * (spectrum_points.len() - 1) as f64) % 1.0;

    // Linear interpolation for each color component
    let red = (start_color.r as f64
        + (end_color.r as f64 - start_color.r as f64) * segment_position)
        .round() as u8;
    let green = (start_color.g as f64
        + (end_color.g as f64 - start_color.g as f64) * segment_position)
        .round() as u8;
    let blue = (start_color.b as f64
        + (end_color.b as f64 - start_color.b as f64) * segment_position)
        .round() as u8;

    Pixel::new(red, green, blue)
}
