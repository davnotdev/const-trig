#![allow(non_snake_case)]

/// See <a href="https://en.wikipedia.org/wiki/Summation">Summation</a>.
macro_rules! Σ {
    ($finish:expr => $( $tt:tt )*) => {
        Σ!(0, $finish, 1 => $( $tt )*)
    };

    ($start:expr, $finish:expr, $step:literal => { $( $tt:tt )* }) => {{
        let mut sum = T::from(0.0);
        let mut i = $start;
        while i < $finish {
            sum += {
                $( $tt )*
            };
            i += $step;
        }
        sum
    }};
}

use crate::DEFAULT_PRECISION;
use core::f32::consts::PI;
use core::ops::*;

///
/// Convert radians to degrees.
/// See <a href="https://en.wikipedia.org/wiki/Radian">Radian</a> and <a href="https://en.wikipedia.org/wiki/Degree_(angle)">Degree</a>.
///
pub const fn degrees <T: ~const From <f32> + ~const Mul <Output = T>> (rads: T) -> T {
    const F: f32 = 180.0 / PI;
    rads * T::from(F)
}

///
/// Convert degrees to radians.
/// See <a href="https://en.wikipedia.org/wiki/Degree_(angle)">Degree</a> and <a href="https://en.wikipedia.org/wiki/Radian">Radian</a>.
///
pub const fn radians <T: ~const From <f32> + ~const Mul <Output = T>> (degs: T) -> T {
    const F: f32 = PI / 180.0;
    degs * T::from(F)
}

///
/// Find <a href="https://en.wikipedia.org/wiki/Sine">Sine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn sin <T: Copy> (rads: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    sine_cosine_helper(rads, precision, rads, T::from(1.0))
}

///
/// Find <a href="https://en.wikipedia.org/wiki/Cosine">Cosine</a>.
///
/// Note that `rads` is radians, not degrees.
///
/// You can use custom precision, by passing
/// `Some(your_precision)` instead of `None` as `precision`.
///
pub const fn cos <T: Copy> (rads: T, precision: Option <usize>) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    sine_cosine_helper(rads, precision, T::from(1.0), T::from(0.0))
}

const fn sine_cosine_helper <T> (rads: T, precision: Option <usize>, mut fraction: T, mut current_fac: T) -> T
where T:
    ~const From <f32> +
    ~const Mul <Output = T> +
    ~const MulAssign +
    ~const DivAssign +
    ~const Add <Output = T> +
    ~const AddAssign +
    ~const Neg <Output = T> +
    Copy
{
    let precision = match precision {
        None => DEFAULT_PRECISION,
        Some(x) => x
    };

    let neg_x_squared = -(rads * rads);

    Σ!(precision => {
        let calculated = fraction;
        fraction *= neg_x_squared;
        fraction /= (current_fac + T::from(1.0)) * (current_fac + T::from(2.0));
        current_fac += T::from(2.0);
        calculated
    })
}
