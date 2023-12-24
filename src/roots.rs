use crate::DEFAULT_PRECISION;
///
/// Find <a href="https://en.wikipedia.org/wiki/Nth_root">`n`th root</a> of `x`,
/// i.e.
/// ```rust
/// fn main() {
///     let x = 34.0;
///     let n = 5;
///     let b = const_trig::root(x, n, None);
///     assert!(const_trig::pow(b, n) - x < 0.0000000000001)
/// }
/// ```
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn root(x: f32, n: usize, precision: Option<usize>) -> f32 {
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x,
    };

    let mut a = x * 0.5;
    let mut dx;
    let mut i = 0;
    while i < precision {
        dx = (x / crate::pow(a, n - 1) - a) / n as f32;
        a += dx;
        i += 1
    }
    a
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Square_root">Square root</a>.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn sqrt(x: f32, precision: Option<usize>) -> f32 {
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x,
    };

    let mut result = x;

    let mut i = 0;
    while i < precision {
        result = 0.5 * (result + x / result);
        i += 1
    }
    result
}
