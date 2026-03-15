// src/lib.rs

pub mod train;
pub mod infer;
pub mod bootstrap;
pub(crate) mod train_xml;

pub use infer::infer;
pub use train::train;
pub use bootstrap::bootstrap;
