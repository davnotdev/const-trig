use crate::{Radians, DEFAULT_PRECISION, Σ};

///
/// Find <a href="https://en.wikipedia.org/wiki/Sine">Sine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn sin(rads: Radians, precision: Option<usize>) -> f32 {
    sine_cosine_helper(rads, precision, rads.get(), 1.0)
}

///
/// Find <a href="https://en.wikipedia.org/wiki/Cosine">Cosine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn cos(rads: Radians, precision: Option<usize>) -> f32 {
    sine_cosine_helper(rads, precision, 1.0, 0.0)
}

const fn sine_cosine_helper(
    rads: Radians,
    precision: Option<usize>,
    mut fraction: f32,
    mut current_fac: f32,
) -> f32 {
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x,
    };

    let rads = rads.get();

    let neg_x_squared = -(rads * rads);

    Σ!(precision => {
        let calculated = fraction;
        fraction *= neg_x_squared;
        fraction /= (current_fac + 1.0) * (current_fac + 2.0);
        current_fac += 2.0;
        calculated
    })
}
