use crate::ln;
use crate::{DEFAULT_PRECISION, Σ};

///
/// Find <a href="https://en.wikipedia.org/wiki/Exponentiation">`n`-th power of `x`</a>
///
pub const fn pow(x: f32, n: usize) -> f32 {
    let mut result = x;
    let mut i = 1;
    while i < n {
        result *= x;
        i += 1
    }
    result
}

pub const fn powf(x: f32, a: f32, precision: Option<usize>) -> f32 {
    exp(a * ln(x, precision), precision)
}

pub const fn exp(x: f32, precision: Option<usize>) -> f32 {
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x,
    };

    let mut n = 0.0;
    let mut a = 1.0;
    let mut b = 1.0;

    Σ!(precision => {
        let calculated = a / b;
        a *= x;
        n += 1.0;
        b *= n;
        calculated
    })
}
