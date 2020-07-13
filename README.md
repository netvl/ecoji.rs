# Ecoji [![Crates](https://img.shields.io/crates/v/ecoji.svg?style=flat-square)](https://crates.io/crates/ecoji) [![Travis](https://img.shields.io/travis/netvl/ecoji.rs.svg?style=flat-square)](https://travis-ci.org/netvl/ecoji.rs) [![docs.rs](https://img.shields.io/badge/documentation-docs.rs-green.svg?style=flat-square)](https://docs.rs/ecoji)

A Rust implementation of the [Ecoji](https://github.com/keith-turner/ecoji) encoding standard.

Provides a library for encoding and decoding data as a base-1024 sequence of emojis, as well as a `base64`-like command
line tool to perform these transformations in your shell.

Visit [ecoji.io](https://ecoji.io) to try Ecoji in your browser.

## Usage

To use the library, add a dependency to your `Cargo.toml`:

```toml
[dependencies]
ecoji = "1.0"
```

See the [crate documentation](https://docs.rs/ecoji) for more information and examples.

To use the CLI binary, execute the following command in your shell:

```
$ cargo install --bin ecoji --features build-binary ecoji
```

After compilation finishes, an `ecoji` binary will be available in your default Cargo binaries directory (usually `~/.cargo/bin` on Unix systems). Run `ecoji --help` to see documentation on how to invoke it.

## License

This program is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed 
as above, without any additional terms or conditions.

