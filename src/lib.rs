//! Recursively loop through the files in a directory.
//!
//! # Examples
//!
//! ```rust
//! for file in recurse::Recurse::new(".") {
//! 	let file = file.unwrap();
//! 	println!("{}", file.display());
//! }
//! ```
//!
//! Look at [Recurse] for more information and the examples directory for a more complex usage.

mod error;
pub use error::Error;

mod recurse;
pub use recurse::Recurse;
