// src/train/mod.rs

mod train;
mod train_write_xmls;
mod train_write_bins;

pub use train::train;
pub use train_write_xmls::train_write_xmls;
pub use train_write_bins::train_write_bins;
