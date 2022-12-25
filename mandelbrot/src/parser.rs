use num::Complex;
use std::str::FromStr;

pub fn complex(s: &str) -> Option<Complex<f64>> {
    pair(s, ',').map(|(re, im)| Complex { im, re })
}

pub fn dimensions(s: &str) -> Option<(usize, usize)> {
    pair(s, 'x')
}

fn pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
        None => None,
    }
}

#[test]
fn test_pair() {
    assert_eq!(pair::<i32>("", ','), None);
    assert_eq!(pair::<i32>("10,", ','), None);
    assert_eq!(pair::<i32>(",10", ','), None);
    assert_eq!(pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(pair::<i32>("10,20xy", ','), None);
    assert_eq!(pair::<f64>("0.5x", 'x'), None);
    assert_eq!(pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
