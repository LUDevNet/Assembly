# assembly-data

This crate contains the database related file formats of the
[assembly](https://crates.io/crates/assembly) library.

## Example Tools

This crate comes with a selection of example tools that can
be installed using:

```shell
$ cargo install assembly-data --examples
```

### fdb-columns

Show all columns and their types for some table:

```shell
$ cargo run --example fdb-columns <file> <table>
```

### fdb-copy

Read an FDB file an create another one with the same content:

```shell
$ cargo run --example fdb-copy <src> <dest>
```

### fdb-index

Show all rows for a single key in a table:

```shell
$ cargo run --example fdb-index <file> <table> <key>
```

### fdb-stat

Print statistics on an FDB file:

```shell
$ cargo run --example fdb-stat <file>
```

### fdb-tables

Show all tables in an FDB file

```shell
$ cargo run --example fdb-tables <file>
```

### fdb-tree

Print the names of all tables and their columns

```shell
$ cargo run --example fdb-tree <file>
```