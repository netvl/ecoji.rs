extern crate ecoji;
#[macro_use]
extern crate clap;

use std::io;

use clap::{App, AppSettings};

fn main() {
    let matches = App::new("ecoji")
        .version(crate_version!())
        .author("Vladimir Matveev <vladimir.matweev@gmail.com>")
        .about("Ecoji encode/decode data and print to standard output")
        .setting(AppSettings::ColoredHelp)
        .args_from_usage(
            "-d, --decode 'Decode data'"
        )
        .get_matches();

    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), stdout.lock());
    if matches.is_present("decode") {
        ecoji::decode(&mut stdin, &mut stdout).expect("Failed to decode data");
    } else {
        ecoji::encode(&mut stdin, &mut stdout).expect("Failed to encode data");
    }
}
