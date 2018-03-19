//! A Rust implementation of [Ecoji](https://github.com/keith-turner/ecoji), a base-1024 encoding
//! with an emoji alphabet.
//!
//! This crate includes both encoding and decoding functionality, as well as a binary with an
//! interface similar to the `base64` tool to perform Ecoji encoding and/or decoding from the
//! command line.
//!
//! ## Features
//!
//! Features of the Ecoji encoding are described in depth in the
//! [original implementation's](https://github.com/keith-turner/ecoji) repository. In short, it has
//! the following key characteristics:
//!
//! * While Ecoji-encoded strings take more bytes than their base-64 or other ASCII-using
//!   counterparts, they take less *visible* characters. More specifically, each visible character
//!   in Ecoji encodes 10 bits of data, while for example each visible character in Base64 encodes
//!   6 bits of data.
//! * Ecoji-encoded strings can be concatenated and then decoded, giving the concatenation of the
//!   original strings:
//!   ```
//!   use ecoji::{encode_to_string, decode_to_string};
//!
//!   # fn test() -> ::std::io::Result<()> {
//!   let (input1, input2) = ("hello ", "world");
//!
//!   // Encode both input strings and concatenate the encoded output
//!   let output1 = encode_to_string(&mut input1.as_bytes())?;
//!   let output2 = encode_to_string(&mut input2.as_bytes())?;
//!   let output = output1 + &output2;
//!
//!   // Then decode the concatenated output
//!   let input = decode_to_string(&mut output.as_bytes())?;
//!
//!   // The result is the same as concatenation of the input strings
//!   assert_eq!(input, input1.to_owned() + input2);
//!   # Ok(())
//!   # }
//!   # test().unwrap();
//!   ```
//! * Data encoded with Ecoji has the same sorting order as the input data:
//!   ```
//!   use ecoji::{encode_to_string, decode_to_string};
//!
//!   # fn test() -> ::std::io::Result<()> {
//!   // The input vector is sorted
//!   let inputs = vec![
//!       "a", "ab", "abc", "abcd",
//!       "ac",
//!       "b", "ba"
//!   ];
//!
//!   // Encode each element of input and sort the resulting strings again
//!   let mut outputs: Vec<_> = inputs.iter().cloned()
//!     .map(|s| encode_to_string(&mut s.as_bytes()))
//!     .collect::<Result<_, _>>()?;
//!   outputs.sort_unstable();
//!
//!   // Decode each output item back
//!   let mut inputs2: Vec<_> = outputs.iter()
//!     .map(|mut s| decode_to_string(&mut s.as_bytes()))
//!     .collect::<Result<_, _>>()?;
//!   let mut inputs2: Vec<_> = inputs2.iter()
//!     .map(|s| s.as_str())
//!     .collect();  // to have a Vec<&str> instead of Vec<String> for assert below
//!
//!   // Input (which is sorted) and decoded output (whose source is sorted) should be the same
//!   assert_eq!(inputs, inputs2);
//!
//!   # Ok(())
//!   # }
//!   # test().unwrap();
//!   ```
//!
//! ## Usage
//!
//! The two main functions provided by this library are [`encode`](fn.encode.html) and
//! [`decode`](fn.decode.html), which both have the same signature: they accept a reference
//! to an `std::io::Read` and a reference to `std::io::Write` and return an `std::io::Result<usize>`
//! with the number of bytes written to the output `std::io::Write`.
//!
//! Additionally, this library provides shortcut functions,
//! [`encode_to_string`](fn.encode_to_string.html), [`decode_to_vec`](fn.decode_to_vec.html) and
//! [`decode_to_string`](fn.decode_to_string.html), whose output is an in-memory `String` or
//! `Vec<u8>`. Note that there is no need to support special versions of the encode/decode
//! operations which would *accept* strings or vectors, because slices of bytes (`&[u8]`) implement
//! the `std::io::Read` trait by default. Therefore, if you have a string or a byte vector, you
//! can invoke the encoding/decoding functions like this:
//!
//! ```
//! # fn test() -> ::std::io::Result<()> {
//! let input_1: &str = "some data";
//! let input_2: &[u8] = b"some data";
//!
//! // Pass a mutable reference to the intermediate &[u8] object returned by `str::as_bytes()`
//! let result_1 = ecoji::encode_to_string(&mut input_1.as_bytes())?;
//!
//! // Pass a mutable reference to a cloned &[u8] object if you already have a byte slice
//! let result_2 = ecoji::encode_to_string(&mut input_2.clone())?;
//! #   Ok(())
//! # }
//! ```
//!
//! ## Command line tool
//!
//! This crate also provides an executable binary, `ecoji`, which provides a command line
//! interface similar to that of the standard `base64` command and which can encode or decode data
//! coming on the standard input and write the results of this processing to the standard output.
//! You can install it by invoking the following command:
//!
//! ```none
//! $ cargo install --bin ecoji --features build-binary ecoji
//! ```
//!
//! ## Issues and limitations
//!
//! Currently this crate does not provide an ability to do wrapping of the encoded text, like
//! e.g. what the `base64` command does with the `-w` flag. It is possible that this feature will
//! be implemented in future; pull requests for this functionality are welcome!
//!
//! This library is almost a direct line-by-line reimplementation of the original algorithm
//! which is implemented in Go. There were almost zero attempts at optimization, therefore
//! performance characteristics may not be stellar. No benchmarking is done either. This is another
//! area where contributions are very welcome.
//!
//! The core API of this library expects `std::io::Read` and `std::io::Write` instances. This
//! implies that the only supported encoding for the emoji output is UTF-8.

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

        fn encoded_data_has_the_same_sort_order(input: Vec<Vec<u8>>) -> bool {
            // input          ---sort--->  input_sorted
            //
            // input          --encode-->  output
            // output         ---sort--->  output_sorted
            // output_sorted  --decode-->  input2_sorted
            //
            // input_sorted       ==       input2_sorted

            let mut input_sorted = input.clone();
            input_sorted.sort_unstable();

            let output: Vec<_> = input.into_iter()
                .map(|b| encode_to_string(&mut b.as_slice()).unwrap())
                .collect();

            let mut output_sorted = output.clone();
            output_sorted.sort_unstable();

            let input2_sorted: Vec<_> = output_sorted.into_iter()
                .map(|s| decode_to_vec(&mut s.as_bytes()).unwrap())
                .collect();

            input_sorted == input2_sorted
        }
    }
}
