//!
//! This crate exists to provide `const` versions of functions,
//! such as `cos`, `sin`, etc.
//!
//! Note that this crate requires nightly Rust
//! to unify behaviour of functions.
//!
//! Crate is no longer needed when all this function will
//! be const in std.
//!
//! Any function, such as `cos`, `sin` from std
//! can be accessed as `const_trig::cos`.
//!
//! There aren't all trig functions,
//! but if you want to add them, you can open a <a href="https://github.com/Roman-Tarasenko-27/const-trig/issues">GitHub issue</a>
//! (someday I will read it) :)
//!
//! Usage:
//! ```rust
//! fn main() {
//!     let sixty_degrees = const_trig::radians(60.0f32);
//!     println!("const_trig::sin = {}", const_trig::sin(sixty_degrees, None));
//!     println!("std sin = {}", sixty_degrees.sin());
//!     println!("const_trig::cos = {}", const_trig::cos(sixty_degrees, None));
//!     println!("std cos = {}", sixty_degrees.cos());
//! }
//! ```
//!

#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]

#![no_std]

mod impls;

pub use impls::*;

///
/// Default precision of functions such as `sin`, `cos`.
///
pub const DEFAULT_PRECISION: usize = 100;
