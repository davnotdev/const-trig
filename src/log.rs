use crate::{DEFAULT_PRECISION, Σ};

///
/// Find a <a href="https://en.wikipedia.org/wiki/Logarithm">Logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn log(a: f32, b: f32, precision: Option<usize>) -> f32 {
    ln(a, precision) / ln(b, precision)
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Binary_logarithm">Binary logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn lb(x: f32, precision: Option<usize>) -> f32 {
    ln(x, precision) * core::f32::consts::LOG2_E
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Common_logarithm">Common(decimal) logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn lg(x: f32, precision: Option<usize>) -> f32 {
    ln(x, precision) * core::f32::consts::LOG10_E
}

///
/// Find a <a href="https://en.wikipedia.org/wiki/Natural_logarithm">Natural logarithm</a>
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn ln(x: f32, precision: Option<usize>) -> f32 {
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x,
    };

    let x = (x - 1.0) / (x + 1.0);
    let xx = x * x;
    let mut fraction = x;
    let mut cur_div = 1.0;

    2.0 * Σ!(precision => {
        let calculated = fraction / cur_div;
        cur_div += 2.0;
        fraction *= xx;
        calculated
    })
}
