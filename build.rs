use std::fs::File;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::path::Path;
use std::env;
use std::error::Error;

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

    writeln!(&mut output, "pub const EMOJIS: [char; 1024] = [")?;
    for line in lines.into_iter().take(1024) {
        writeln!(&mut output, r"    '\u{{{}}}',", line)?;
    }
    writeln!(&mut output, "];")?;

    Ok(())
}

