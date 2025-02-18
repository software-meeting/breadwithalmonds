use num::complex::{Complex64, ComplexFloat};
use bmp::{Image, Pixel};

const REPS: u8 = 255;
const BOUND: f64 = 3.0;
const IMG_SIZE: u32 = 4096;

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "something_else" {
        // Add more cases here!
    } else if operation.as_str() == "mandelbrot" {
        draw_mandelbrot(&path);
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn draw_mandelbrot(path: &str) {
    let mut image = Image::new(IMG_SIZE, IMG_SIZE);

    for x in 0..IMG_SIZE {
        let a: f64 = -2.5+(x as f64)*3.0/(IMG_SIZE as f64);

        for y in 0..IMG_SIZE {
            let b: f64 = -1.5+(y as f64)*3.0/(IMG_SIZE as f64);

            let mut z: Complex64 = Complex64::new(0.0, 0.0);
            let c: Complex64 = Complex64::new(a, b);

            let mut val: u8 = 0;

            for n in 0..REPS {
                if z.abs() >= BOUND {
                    val = n;
                    break;
                } else {
                    z = z.powi(2) + c;
                }
            }

            image.set_pixel(x, y, Pixel::new(val * 25, val * 25, val * 25));
        }
    }

    let _ = image.save(path);
}
