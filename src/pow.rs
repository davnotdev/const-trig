use core::ops::MulAssign;

///
/// Find <a href="https://en.wikipedia.org/wiki/Exponentiation">`n`-th power of `x`</a>
///
pub const fn pow <T: Copy + ~const MulAssign> (x: T, n: usize) -> T {
    let mut result = x;
    let mut i = 1;
    while i < n {
        result *= x;
        i += 1
    }
    result
}
