# assembly-data

This crate contains the database related file formats of the
[assembly](https://crates.io/crates/assembly) library.

## Example Tools

This crate comes with a selection of example tools that can
be installed using:

```shell
$ cargo install assembly-data --examples
```

### xmldb-to-fdb

Convert an XML database into an FDB file:

```shell
$ cargo run --example xmldb-to-fdb <input xml> <output fdb>
```
