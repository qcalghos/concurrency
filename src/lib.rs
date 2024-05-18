mod matrix;
mod metrics;
pub mod metrics1;
mod vector;
pub use matrix::{multiply, Matrix};
pub use metrics::Metrics;
pub use vector::{dot_product, Vector};
