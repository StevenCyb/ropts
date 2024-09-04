//! # Simple Option Library for Rust
//!
//! This library provides an easy way to parse command line or environment options.
//! It is designed to be simple and intuitive, making it easy to integrate into your Rust projects.
//!
//! ## Modules
//!
//! - [`compose`](src/compose.rs): Contains utilities for composing options.
//! - [`error`](src/error.rs): Defines error types used throughout the library.
//! - [`options`](src/options.rs): Core module for defining and handling options.
//!
//! ## Usage
//!
//! To use this library, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ropts = "0.1.0"
//! ```
//!
//! ## License
//!
//! This library is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
//!

pub mod compose;
pub mod error;
pub mod options;
