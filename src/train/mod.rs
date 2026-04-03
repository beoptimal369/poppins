// src/train/mod.rs

mod train;
mod train_write_txts;
mod train_write_bins;

pub use train::train;
pub use train_write_txts::train_write_txts;
pub use train_write_bins::train_write_bins;
