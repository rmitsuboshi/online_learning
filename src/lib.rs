#[warn(missing_docs)]

/// Defines a trait for loss functions.
pub mod loss_function;
/// Defines a trait for adversaries (i.e., environments).
pub mod adversary;

pub use adversary::core;
