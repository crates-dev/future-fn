//! future-fn
//!
//! A Rust library providing macros to simplify the creation of
//! asynchronous closures with external state captured by move.
//! Useful for structuring asynchronous code with ease and clarity.

pub(crate) mod cfg;
pub(crate) mod r#macro;

#[allow(unused_imports)]
pub use r#macro::*;
