extern crate phf;

mod emojis;
mod encode;
mod decode;
mod chars;

pub use encode::{encode, encode_to_string};
pub use decode::decode;
