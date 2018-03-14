extern crate ecoji;
#[macro_use]
extern crate clap;

use std::io;

use clap::{App, AppSettings};

fn main() {
    let matches = App::new("ecoji")
        .version(crate_version!())
        .author("Vladimir Matveev <vladimir.matweev@gmail.com>")
        .about("ecoji encode/decode data and print to standard output")
        .setting(AppSettings::ColoredHelp)
        .args_from_usage(
            "-d --decode 'Decode data'"
        )
        .get_matches();

    if matches.value_of("d").is_some() {
        unimplemented!();
    } else {
        let (stdin, stdout) = (io::stdin(), io::stdout());
        let (mut stdin, mut stdout) = (stdin.lock(), stdout.lock());
        ecoji::encode(&mut stdin, &mut stdout).expect("Failed to encode data");
    }
}
