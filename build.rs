extern crate phf_codegen;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::path::Path;
use std::env;
use std::error::Error;
use std::char;

fn main() {
    run().expect("Failed to generate 'emojis.rs'");
}

fn run() -> Result<(), Box<Error>> {
    let input = BufReader::new(File::open("emojis.txt")?);
    let mut lines: Vec<_> = input.lines().collect::<Result<_, _>>()?;

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("emojis.rs");
    let mut output = BufWriter::new(File::create(&dest_path)?);

    writeln!(&mut output, r"pub const PADDING: char = '\u{{2615}}';")?;
    writeln!(&mut output, r"pub const PADDING_40: char = '\u{{269C}}';")?;
    writeln!(&mut output, r"pub const PADDING_41: char = '\u{{{}}}';", lines.remove(256))?;
    writeln!(&mut output, r"pub const PADDING_42: char = '\u{{{}}}';", lines.remove(512))?;
    writeln!(&mut output, r"pub const PADDING_43: char = '\u{{{}}}';", lines.remove(768))?;

    let mut rev_map = phf_codegen::Map::new();

    writeln!(&mut output, "pub const EMOJIS: [char; 1024] = [")?;
    for (i, line) in lines.into_iter().take(1024).enumerate() {
        writeln!(&mut output, r"    '\u{{{}}}',", line)?;
        rev_map.entry(char::from_u32(u32::from_str_radix(&line, 16).unwrap()).unwrap(), &i.to_string());
    }
    writeln!(&mut output, "];")?;

    write!(&mut output, "static EMOJIS_REV: ::phf::Map<char, usize> = ")?;
    rev_map.build(&mut output)?;
    writeln!(&mut output, ";")?;

    Ok(())
}

