# assembly-xml

This crate contains the XML database format of the
[assembly](https://crates.io/crates/assembly) library.

## Example Tools

This crate comes with a selection of example tools that can
be installed using:

```shell
$ cargo install assembly-xml --examples
```

### xmldb-tree

Print the names of all tables and their columns:

```shell
$ cargo run --example xmldb-tree <file>
```
