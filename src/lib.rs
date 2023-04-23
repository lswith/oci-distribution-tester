//! # Test framework for an OCI compliant registry.
mod client;
mod fake;
mod image;
mod tester;

pub use tester::{load_test_pull, load_test_push};
