extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(left), Ok(right)) => Some((left, right)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair("12x14", 'x'), Some((12, 14)));
    assert_eq!(parse_pair("1.5,2", ','), Some((1.5, 2.0)));
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
    assert_eq!(escape_time(Complex { re: 0.1, im: 0.8 }, 3),   None);
}
