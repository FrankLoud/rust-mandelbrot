extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        )?;
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.2,0.35 -1,0.2",
            args[0]
        )?;
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing pixel bounds");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("Error writing image file");

    Ok(())
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair("12x14", 'x'), Some((12, 14)));
    assert_eq!(parse_pair("1.5,2", ','), Some((1.5, 2.0)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex { re, im }),
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("12,13.5"),
        Some(Complex { re: 12.0, im: 13.5 })
    );
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() >= 4.0 {
            return Some(i);
        }
    }

    return None;
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex { re: 3.0, im: 4.0 }, 255), Some(0));
    assert_eq!(escape_time(Complex { re: 0.1, im: 0.8 }, 255), Some(5));
    assert_eq!(escape_time(Complex { re: 0.1, im: 0.8 }, 3), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + (pixel.0 as f64 / bounds.0 as f64) * width,
        im: upper_left.im - (pixel.1 as f64 / bounds.1 as f64) * height,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (200, 100),
            (0, 0),
            Complex { re: 0.0, im: 1.0 },
            Complex { re: 1.0, im: 0.0 }
        ),
        Complex { re: 0.0, im: 1.0 }
    );

    assert_eq!(
        pixel_to_point(
            (200, 100),
            (0, 0),
            Complex { re: 1.0, im: 2.0 },
            Complex { re: 2.0, im: 1.0 }
        ),
        Complex { re: 1.0, im: 2.0 }
    );

    assert_eq!(
        pixel_to_point(
            (200, 100),
            (50, 50),
            Complex { re: 1.0, im: 2.0 },
            Complex { re: 2.0, im: 1.0 }
        ),
        Complex { re: 1.25, im: 1.5 }
    );
}

fn render(
    buffer: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(buffer.len() == bounds.0 * bounds.1);

    for i in 0..bounds.1 {
        for j in 0..bounds.0 {
            let point = pixel_to_point(bounds, (i, j), upper_left, lower_right);
            buffer[i * bounds.0 + j] = match escape_time(point, 255) {
                Some(n) => 255 - n as u8,
                None => 0,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}
