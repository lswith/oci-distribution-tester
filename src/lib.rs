//! # Test framework for an OCI compliant registry.
mod cli;
mod client;
mod fake;
mod image;
mod tester;

pub use cli::{pull_images, push_images};
