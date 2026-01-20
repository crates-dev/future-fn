//! future-fn
//!
//! A Rust library providing macros to simplify the creation of
//! asynchronous closures with external state captured by move.
//! Useful for structuring asynchronous code with ease and clarity.

pub(crate) mod r#macro;

#[cfg(test)]
mod test;

#[cfg(test)]
pub(crate) use std::time::Duration;

#[cfg(test)]
pub(crate) use tokio::time::sleep;
