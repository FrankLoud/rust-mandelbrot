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
