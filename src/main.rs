extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
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
