// src/lib.rs

pub mod train;
pub mod infer;
pub mod bootstrap;

pub use infer::infer;
pub use train::train;
pub use bootstrap::bootstrap;
