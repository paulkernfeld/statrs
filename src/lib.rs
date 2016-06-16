//! This crate aims to be a functional
//! port of the Math.NET Numerics Distribution package and in doing so providing the Rust numerical
//! computing community with a robust, well-tested statistical distribution package. This crate
//! also ports over some of the special statistical functions from Math.NET in so far as they are 
//! used in the computation of distribution values. This crate depends on the `rand` crate to provide
//! RNG.
//!
//! # Example
//! The following example samples from a standard normal distribution
//!
//! ```
//! use rand::StdRng;
//! 
//! let r = rand::StdRng::new().unwrap();
//! let n = distribution::Normal::new(0.0, 1.0).unwrap();
//! for 0..10 {
//!     print!("{}", n.sample(r));
//! }
//! ```

#![crate_type = "lib"]
#![crate_name = "statrs"]

extern crate rand;

pub mod distribution;
pub mod function;
pub mod consts;
pub mod prec;

mod result;
mod error;

pub use result::Result;
pub use error::StatsError;
