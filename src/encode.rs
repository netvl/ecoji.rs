use emojis::*;
use std::io::{self, Read, Write};

fn encode_chunk<W: Write + ?Sized>(s: &[u8], out: &mut W) -> io::Result<usize> {
    assert!(s.len() > 0 && s.len() <= 5, "Unexpected slice length");

    let (b0, b1, b2, b3, b4) = (
        s[0] as usize,
        s.get(1).cloned().unwrap_or(0) as usize,
        s.get(2).cloned().unwrap_or(0) as usize,
        s.get(3).cloned().unwrap_or(0) as usize,
        s.get(4).cloned().unwrap_or(0) as usize,
    );

    let mut chars = [
        EMOJIS[b0 << 2 | b1 >> 6] as char,
        PADDING,
        PADDING,
        PADDING,
    ];

    match s.len() {
        1 => {}
        2 => {
            chars[1] = EMOJIS[(b1 & 0x3f) << 4 | b2 >> 4]
        }
        3 => {
            chars[1] = EMOJIS[(b1 & 0x3f) << 4 | b2 >> 4];
            chars[2] = EMOJIS[(b2 & 0x0f) << 6 | b3 >> 2];
        }
        4 => {
            chars[1] = EMOJIS[(b1 & 0x3f) << 4 | b2 >> 4];
            chars[2] = EMOJIS[(b2 & 0x0f) << 6 | b3 >> 2];

            chars[3] = match b3 & 0x03 {
                0 => PADDING_40,
                1 => PADDING_41,
                2 => PADDING_42,
                3 => PADDING_43,
                _ => unreachable!(),
            }
        }
        5 => {
            chars[1] = EMOJIS[(b1 & 0x3f) << 4 | b2 >> 4];
            chars[2] = EMOJIS[(b2 & 0x0f) << 6 | b3 >> 2];
            chars[3] = EMOJIS[(b3 & 0x03) << 8 | b4];
        }
        _ => unreachable!(),
    }

    let mut buf = [0; 4];
    let mut bytes_written = 0;
    for c in chars.iter() {
        let s = c.encode_utf8(&mut buf).as_bytes();
        out.write_all(s)?;
        bytes_written += s.len();
    }

    Ok(bytes_written)
}

fn read_exact<R: Read + ?Sized>(source: &mut R, mut buf: &mut [u8]) -> io::Result<usize> {
    let mut bytes_read = 0;
    while !buf.is_empty() {
        match source.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
                bytes_read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(bytes_read)
}

/// Encodes the entire source into the Ecoji format and writes a UTF-8 representation of
/// the encoded data to the provided destination.
///
/// If successful, returns the number of bytes which were written to the destination writer.
///
/// Returns an error when either source or destination operation has failed. No guarantees are
/// made about the state of the destination if an error occurs, so it is possible for the
/// destination to contain only a part of the encoded data.
///
/// # Examples
///
/// Successful encoding:
///
/// ```
/// # fn test() -> ::std::io::Result<()> {
/// let input = "input data";
///
/// let mut output: Vec<u8> = Vec::new();
/// ecoji::encode(&mut input.as_bytes(), &mut output)?;
///
/// assert_eq!(output, "👶😲🇲👅🍉🔙🌥🌩".as_bytes());
/// #  Ok(())
/// # }
/// # test().unwrap();
/// ```
pub fn encode<R: Read + ?Sized, W: Write + ?Sized>(source: &mut R, destination: &mut W) -> io::Result<usize> {
    let mut buf = [0; 5];
    let mut bytes_written = 0;

    loop {
        let n = read_exact(source, &mut buf)?;

        // EOF
        if n == 0 {
            break;
        }

        bytes_written += encode_chunk(&buf[..n], destination)?;
    }

    Ok(bytes_written)
}

/// Encodes the entire source into the Ecoji format, storing the result of the encoding to a
/// new owned string.
///
/// Returns a string with the encoded data if successful.
///
/// Failure conditions are exactly the same as those of the [`encode`](fn.encode.html) function;
/// because the encoding output is always a valid sequence of emoji code points, it is guaranteed
/// to be representable as a valid UTF-8 sequence.
///
/// # Examples
///
/// Successful encoding:
///
/// ```
/// # fn test() -> ::std::io::Result<()> {
/// let input = "input data";
/// let output: String = ecoji::encode_to_string(&mut input.as_bytes())?;
///
/// assert_eq!(output, "👶😲🇲👅🍉🔙🌥🌩");
/// #  Ok(())
/// # }
/// # test().unwrap();
/// ```
pub fn encode_to_string<R: Read + ?Sized>(source: &mut R) -> io::Result<String> {
    let mut output = Vec::new();
    encode(source, &mut output)?;
    // encoded output is guaranteed to be valid UTF-8
    Ok(unsafe { String::from_utf8_unchecked(output) })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &[u8], output: &[u8]) {
        let buf = encode_to_string(&mut input.clone()).unwrap();
        assert_eq!(output, buf.as_bytes());
    }

    fn check_chars(input: &[u8], output: &[char]) {
        let buf = encode_to_string(&mut input.clone()).unwrap();
        let chars: Vec<_> = buf.chars().collect();
        assert_eq!(output, chars.as_slice());
    }

    #[test]
    fn test_random() {
        check(b"abc", "👖📸🎈☕".as_bytes());
    }

    #[test]
    fn test_one_byte() {
        check_chars(b"k", &[EMOJIS[('k' as usize) << 2], PADDING, PADDING, PADDING]);
    }

    #[test]
    fn test_two_bytes() {
        check_chars(&[0, 1], &[EMOJIS[0], EMOJIS[16], PADDING, PADDING]);
    }

    #[test]
    fn test_three_bytes() {
        check_chars(&[0, 1, 2], &[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING]);
    }

    #[test]
    fn test_four_bytes() {
        check_chars(&[0, 1, 2, 0], &[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_40]);
        check_chars(&[0, 1, 2, 1], &[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_41]);
        check_chars(&[0, 1, 2, 2], &[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_42]);
        check_chars(&[0, 1, 2, 3], &[EMOJIS[0], EMOJIS[16], EMOJIS[128], PADDING_43]);
    }

    #[test]
    fn test_five_bytes() {
        check_chars(&[0xAB, 0xCD, 0xEF, 0x01, 0x23], &[EMOJIS[687], EMOJIS[222], EMOJIS[960], EMOJIS[291]]);
    }
}
