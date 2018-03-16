//! Copied from libstd, because it is unstable there.

use std::result;
use std::io::*;
use std::fmt;
use std::error as std_error;
use std::str;

#[derive(Debug)]
pub struct Chars<R> {
    inner: R,
}

impl<R> Chars<R> {
    pub fn new(inner: R) -> Chars<R> {
        Chars { inner, }
    }
}

#[derive(Debug)]
pub enum CharsError {
    NotUtf8,
    Other(Error),
}

impl CharsError {
    pub fn into_io(self) -> Error {
        Error::new(ErrorKind::InvalidInput, self)
    }
}

impl<R: Read> Iterator for Chars<R> {
    type Item = result::Result<char, CharsError>;

    fn next(&mut self) -> Option<result::Result<char, CharsError>> {
        let first_byte = match read_one_byte(&mut self.inner)? {
            Ok(b) => b,
            Err(e) => return Some(Err(CharsError::Other(e))),
        };
        let width = utf8_char_width(first_byte);
        if width == 1 { return Some(Ok(first_byte as char)) }
        if width == 0 { return Some(Err(CharsError::NotUtf8)) }
        let mut buf = [first_byte, 0, 0, 0];
        {
            let mut start = 1;
            while start < width {
                match self.inner.read(&mut buf[start..width]) {
                    Ok(0) => return Some(Err(CharsError::NotUtf8)),
                    Ok(n) => start += n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Some(Err(CharsError::Other(e))),
                }
            }
        }
        Some(match str::from_utf8(&buf[..width]).ok() {
            Some(s) => Ok(s.chars().next().unwrap()),
            None => Err(CharsError::NotUtf8),
        })
    }
}

impl std_error::Error for CharsError {
    fn description(&self) -> &str {
        match *self {
            CharsError::NotUtf8 => "invalid utf8 encoding",
            CharsError::Other(ref e) => std_error::Error::description(e),
        }
    }
    fn cause(&self) -> Option<&std_error::Error> {
        match *self {
            CharsError::NotUtf8 => None,
            CharsError::Other(ref e) => e.cause(),
        }
    }
}

impl fmt::Display for CharsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharsError::NotUtf8 => "byte stream did not contain valid utf8".fmt(f),
            CharsError::Other(ref e) => e.fmt(f),
        }
    }
}

fn read_one_byte(reader: &mut Read) -> Option<Result<u8>> {
    let mut buf = [0];
    loop {
        return match reader.read(&mut buf) {
            Ok(0) => None,
            Ok(..) => Some(Ok(buf[0])),
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => Some(Err(e)),
        };
    }
}

// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

fn utf8_char_width(b: u8) -> usize {
    return UTF8_CHAR_WIDTH[b as usize] as usize;
}

