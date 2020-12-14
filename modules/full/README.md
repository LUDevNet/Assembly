# assembly

[![docs_badge](https://docs.rs/assembly/badge.svg)](https://docs.rs/assembly/)
[![crate_badge](https://img.shields.io/crates/v/assembly.svg)](https://crates.io/crates/assembly)
[![license_badge](https://img.shields.io/crates/l/assembly.svg?color=green)](https://github.com/Xiphoseer/assembly_rs/blob/master/LICENSE)

This is a [Rust](https://rust-lang.org) version of the [Assembly][assembly] C++ library. It is a
library to read and possibly write files, formats and resources of LEGO Universe
game files.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
assembly = "0.7"
```

## Modules

This crate is a meta-crate combining multiple modules. Each submodule can
be enabled or disabled by using the suffix after `assembly-` as the feature flag.

For example, to only use the [data][assembly-data] and [maps][assembly-data]
modules, update your `Cargo.toml` to include:

```toml
[dependencies.assembly]
version = "0.7"
default-features = false
features = ["data", "maps"]
```

[assembly]: https://github.com/xiphoseer/assembly
[assembly-data]: https://crates.io/crates/assembly-data
[assembly-maps]: https://crates.io/crates/assembly-maps
