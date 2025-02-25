use image::{Rgb, RgbImage};
use num::complex::{Complex64, ComplexFloat};
use rayon::prelude::*;

const REPS: u32 = 512;
const BOUND: f64 = 32.0;
const IMG_SIZE: u32 = 4096;
const N_PIXELS: usize = (IMG_SIZE * IMG_SIZE) as usize;

// Number of quadrants is 2^n_splits
// Offsets are 0,0 from top left quadrant
const OFFSET_X: f64 = 150.0;
const OFFSET_Y: f64 = 112.0;
const N_SPLITS: u32 = 8;
const SPLIT_SIZE: f64 = 3_f64 / (2u32.pow(N_SPLITS) as f64);
const SCALE: f64 = SPLIT_SIZE / IMG_SIZE as f64;
fn calculate_coords(x: u32, y: u32) -> (f64, f64) {
    let x0 = -2.5 + OFFSET_X * SPLIT_SIZE;
    let y0 = -1.5 + OFFSET_Y * SPLIT_SIZE;
    (x0 + (x as f64) * SCALE, y0 + (y as f64) * SCALE)
}

fn main() {
    // let mut image = Image::new(IMG_SIZE, IMG_SIZE);
    let mut image = RgbImage::new(IMG_SIZE, IMG_SIZE);

    let pixels: Vec<Rgb<u8>> = (0..N_PIXELS)
        .into_par_iter()
        .map(|n| {
            let y = n as u32 / IMG_SIZE;
            let x = n as u32 % IMG_SIZE;
            let (a, b) = calculate_coords(x, y);

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
        image.put_pixel(x, y, *pixel);
    }

    image.save("out.png").unwrap();
}

// TODO: Review generated code
fn u32_to_rgb_spectrum(input: u32) -> Rgb<u8> {
    // Define the spectrum "points".  We'll interpolate between them.
    let spectrum_points: [Rgb<u8>; 6] = [
        Rgb([0, 0, 0]),
        Rgb([255, 0, 0]),   // Red
        Rgb([255, 255, 0]), // Yellow
        Rgb([0, 255, 0]),   // Green
        Rgb([0, 255, 255]), // Cyan
        Rgb([0, 0, 255]),   // Blue
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
    let red = (start_color.0[0] as f64
        + (end_color.0[0] as f64 - start_color.0[0] as f64) * segment_position)
        .round() as u8;
    let green = (start_color.0[1] as f64
        + (end_color.0[1] as f64 - start_color.0[1] as f64) * segment_position)
        .round() as u8;
    let blue = (start_color.0[2] as f64
        + (end_color.0[2] as f64 - start_color.0[2] as f64) * segment_position)
        .round() as u8;

    Rgb([red, green, blue])
}
