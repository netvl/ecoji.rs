use std::io::{self, Read, Write};

use chars::{Chars, CharsError};
use emojis::*;

/// Decodes the entire source from the Ecoji format (assumed to be UTF-8-encoded) and writes the
/// result of the decoding to the provided destination.
///
/// If successful, returns the number of bytes which were written to the destination writer.
///
/// Returns an error when either source or destination operation has failed, if the number of
/// code points in the input is wrong (it must be a multiple of 4), if the source is not
/// a valid UTF-8 stream or if one of the code points in the source is not a valid character
/// of the Ecoji alphabet. No guarantees are made about the state of the destination if an error
/// occurs, so it is possible for the destination to contain only a part of the decoded data.
///
/// # Examples
///
/// Successful read:
/// ```
/// # fn test() -> ::std::io::Result<()> {
/// let input = "👶😲🇲👅🍉🔙🌥🌩";
///
/// let mut output: Vec<u8> = Vec::new();
/// ecoji::decode(&mut input.as_bytes(), &mut output)?;
///
/// assert_eq!(output, b"input data");
/// #  Ok(())
/// # }
/// # test().unwrap();
/// ```
///
/// Invalid input data, not enough code points:
/// ```
/// use std::io;
///
/// let input = "👶😲🇲👅🍉🔙🌥";  // one less than needed
///
/// let mut output: Vec<u8> = Vec::new();
/// match ecoji::decode(&mut input.as_bytes(), &mut output) {
///   Ok(_) => panic!("Unexpected success"),
///   Err(e) => assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof),
/// }
/// ```
///
/// Invalid input data, not a correct UTF-8 stream:
/// ```
/// use std::io;
///
/// let input: &[u8] = &[0xfe, 0xfe, 0xff, 0xff];
///
/// let mut output: Vec<u8> = Vec::new();
/// match ecoji::decode(&mut input.clone(), &mut output) {
///   Ok(_) => panic!("Unexpected success"),
///   Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData),
/// }
/// ```
///
/// Invalid input data, input code point is not a part of the Ecoji alphabet:
/// ```
/// use std::io;
///
/// // Padded with spaces for the length to be a multiple of 4
/// let input = "Not emoji data  ";
///
/// let mut output: Vec<u8> = Vec::new();
/// match ecoji::decode(&mut input.as_bytes(), &mut output) {
///   Ok(_) => panic!("Unexpected success"),
///   Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData),
/// }
/// ```
pub fn decode<R: Read + ?Sized, W: Write + ?Sized>(source: &mut R, destination: &mut W) -> io::Result<usize> {
    let mut input = Chars::new(source);

    let mut bytes_written = 0;
    loop {
        let mut chars = ['\0'; 4];

        match input.next() {
            Some(c) => chars[0] = check_char(c)?,
            None => break,
        };
        for i in 1..4 {
            match input.next() {
                Some(c) => chars[i] = check_char(c)?,
                None => return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unexpected end of data, input code points count is not a multiple of 4"
                ))
            }
        }

        let (bits1, bits2, bits3) = (
            EMOJIS_REV.get(&chars[0]).cloned().unwrap_or(0),
            EMOJIS_REV.get(&chars[1]).cloned().unwrap_or(0),
            EMOJIS_REV.get(&chars[2]).cloned().unwrap_or(0)
        );
        let bits4 = match chars[3] {
            PADDING_40 => 0,
            PADDING_41 => 1 << 8,
            PADDING_42 => 2 << 8,
            PADDING_43 => 3 << 8,
            other => EMOJIS_REV.get(&other).cloned().unwrap_or(0),
        };

        let out = [
            (bits1 >> 2) as u8,
            (((bits1 & 0x3) << 6) | (bits2 >> 4)) as u8,
            (((bits2 & 0xf) << 4) | (bits3 >> 6)) as u8,
            (((bits3 & 0x3f) << 2) | (bits4 >> 8)) as u8,
            (bits4 & 0xff) as u8
        ];

        let out = if chars[1] == PADDING {
            &out[..1]
        } else if chars[2] == PADDING {
            &out[..2]
        } else if chars[3] == PADDING {
            &out[..3]
        } else if chars[3] == PADDING_40 || chars[3] == PADDING_41 || chars[3] == PADDING_42 || chars[3] == PADDING_43 {
            &out[..4]
        } else {
            &out[..]
        };

        destination.write_all(out)?;
        bytes_written += out.len();
    }

    Ok(bytes_written)
}

/// Decodes the entire source from the Ecoji format (assumed to be UTF-8-encoded), storing the
/// result of the decoding to a new byte vector.
///
/// Returns a byte vector with the decoded data if successful.
///
/// Failure conditions are exactly the same as those of the [`decode`](fn.decode.html) function.
///
/// # Examples
///
/// Successful read:
/// ```
/// # fn test() -> ::std::io::Result<()> {
/// let input = "👶😲🇲👅🍉🔙🌥🌩";
/// let output: Vec<u8> = ecoji::decode_to_vec(&mut input.as_bytes())?;
///
/// assert_eq!(output, b"input data");
/// #  Ok(())
/// # }
/// # test().unwrap();
/// ```
///
/// See [`decode`](fn.decode.html) docs for error examples.
pub fn decode_to_vec<R: Read + ?Sized>(source: &mut R) -> io::Result<Vec<u8>> {
    let mut output = Vec::new();
    decode(source, &mut output)?;
    Ok(output)
}

/// Decodes the entire source from the Ecoji format (assumed to be UTF-8-encoded), storing the
/// result of the decoding to a new owned string.
///
/// Returns a string with the decoded data if successful.
///
/// In addition to the [`decode`](fn.decode.html) failure conditions, this function also returns
/// an error if the decoded data is not a valid UTF-8 string.
///
/// # Examples
///
/// Successful read:
/// ```
/// # fn test() -> ::std::io::Result<()> {
/// let input = "👶😲🇲👅🍉🔙🌥🌩";
/// let output: String = ecoji::decode_to_string(&mut input.as_bytes())?;
///
/// assert_eq!(output, "input data");
/// #  Ok(())
/// # }
/// # test().unwrap();
/// ```
///
/// Invalid input data, decoded string is not a valid UTF-8 string:
/// ```
/// use std::io;
///
/// let input = "🧑🦲🧕🙋";  // Encoded data: [0xfe, 0xfe, 0xff, 0xff]
///
/// match ecoji::decode_to_string(&mut input.as_bytes()) {
///   Ok(_) => panic!("Unexpected success"),
///   Err(e) => assert_eq!(e.kind(), io::ErrorKind::InvalidData),
/// }
/// ```
pub fn decode_to_string<R: Read + ?Sized>(source: &mut R) -> io::Result<String> {
    let output = decode_to_vec(source)?;
    String::from_utf8(output).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn check_char(c: Result<char, CharsError>) -> io::Result<char> {
    c.map_err(CharsError::into_io).and_then(|c| if is_valid_alphabet_char(c) {
        Ok(c)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Input character '{}' is not a part of the Ecoji alphabet", c)
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &[u8], output: &[u8]) {
        let buf = decode_to_vec(&mut input.clone()).unwrap();
        assert_eq!(output, buf.as_slice());
    }

    fn check_chars(input: &[char], output: &[u8]) {
        let input: String = input.iter().cloned().collect();
        let buf = decode_to_vec(&mut input.as_bytes()).unwrap();
        assert_eq!(output, buf.as_slice());
    }

    #[test]
    fn test_random() {
        check("👖📸🎈☕".as_bytes(), b"abc");
    }

    #[test]
    fn test_one_byte() {
        check_chars(&[EMOJIS[('k' as usize) << 2], PADDING, PADDING, PADDING], b"k");
    }

    #[test]
    fn test_two_bytes() {
        check_chars(&[EMOJIS[0], EMOJIS[16], PADDING, PADDING], &[0, 1]);
    }

    #[test]
    fn test_three_bytes() {
        check_chars(&[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING], &[0, 1, 2]);
    }

    #[test]
    fn test_four_bytes() {
        check_chars(&[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_40], &[0, 1, 2, 0]);
        check_chars(&[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_41], &[0, 1, 2, 1]);
        check_chars(&[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_42], &[0, 1, 2, 2]);
        check_chars(&[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_43], &[0, 1, 2, 3]);
    }

    #[test]
    fn test_five_bytes() {
        check_chars(&[EMOJIS[687], EMOJIS[222], EMOJIS[960], EMOJIS[291]], &[0xAB, 0xCD, 0xEF, 0x01, 0x23]);
    }
}
