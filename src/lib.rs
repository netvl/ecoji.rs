extern crate phf;
#[cfg(test)] #[macro_use] extern crate quickcheck;

mod emojis;
mod encode;
mod decode;
mod chars;

pub use encode::{encode, encode_to_string};
pub use decode::{decode, decode_to_vec, decode_to_string};

#[cfg(test)]
mod test {
    use super::*;

    quickcheck! {
        fn encode_then_decode_identity(input: Vec<u8>) -> bool {
            let encoded = encode_to_string(&mut input.as_slice()).unwrap();
            let output = decode_to_vec(&mut encoded.as_bytes()).unwrap();
            input == output
        }
    }
}
