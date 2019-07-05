# Assembly

[![docs_badge](https://docs.rs/assembly/badge.svg)](https://docs.rs/assembly/)
[![crate_badge](https://img.shields.io/crates/v/assembly.svg)](https://crates.io/crates/assembly)
[![license_badge](https://img.shields.io/crates/l/assembly.svg?color=green)](https://github.com/Xiphoseer/assembly_rs/blob/master/LICENSE)

This is a [Rust](https://rust-lang.org) version of the [Assembly][assembly] C++ library. It is a
library to read and possibly write files, formats and resources of LEGO Universe
game files.

[assembly]: https://github.com/xiphoseer/assembly

## Example Tools

Load the table from the database file and print all rows corresponding to the given key:

```sh
$ cargo run --example fdb-index DB-FILE TABLE KEY
```
