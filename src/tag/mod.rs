// src/tag/mod.rs

mod tag_writer;
mod tag_write_tag;
mod tag_write_code_open;
mod tag_write_code_close;
mod tag_write_line_break;
mod tag_write_ai_section;
mod tag_write_prompt_section;
mod tag_get_byte_length_ai_item;
mod tag_get_byte_offset_last_ai_start;

pub use tag_writer::TagWriter;
pub use tag_write_tag::tag_write_tag;
pub use tag_write_code_open::tag_write_code_open;
pub use tag_write_code_close::tag_write_code_close;
pub use tag_write_line_break::tag_write_line_break;
pub use tag_write_ai_section::tag_write_ai_section;
pub use tag_write_prompt_section::tag_write_prompt_section;
pub use tag_get_byte_length_ai_item::tag_get_byte_length_ai_item;
pub use tag_get_byte_offset_last_ai_start::tag_get_byte_offset_last_ai_start;
