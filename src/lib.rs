//! future-fn
//!
//! A Rust library providing macros to simplify the creation of
//! asynchronous closures with external state captured by move.
//! Useful for structuring asynchronous code with ease and clarity.

mod r#macro;
#[cfg(test)]
mod test;

#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
use tokio::time::sleep;
