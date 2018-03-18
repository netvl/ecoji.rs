use std::io::{self, Read, Write};

use chars::{Chars, CharsError};
use emojis::*;

pub fn decode<R: Read + ?Sized, W: Write + ?Sized>(source: &mut R, destination: &mut W) -> io::Result<usize> {
    let mut input = Chars::new(source);

    let mut bytes_written = 0;
    loop {
        let mut chars = ['\0'; 4];

        match input.next() {
            Some(c) => chars[0] = c.map_err(CharsError::into_io)?,
            None => break,
        };
        for i in 1..4 {
            match input.next() {
                Some(c) => chars[i] = c.map_err(CharsError::into_io)?,
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
