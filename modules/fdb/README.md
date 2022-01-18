# assembly-fdb

This crate contains the FDB database format of the
[assembly](https://crates.io/crates/assembly) library.

## Example Tools

This crate comes with a selection of example tools that can
be installed using:

```shell
$ cargo install assembly-fdb --examples
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

### fdb-to-sqlite

Convert an FDB file to SQLite

```shell
$ cargo run --example fdb-to-sqlite <input fdb> <output sqlite>
```

### sqlite-to-fdb

Convert an SQLite database to FDB

```shell
$ cargo run --example sqlite-to-fdb <input sqlite> <output fdb>
```

If your SQLite database was generated with an old version of `fdb-to-sqlite`, it will be missing column type information. In this case, you can can first turn an existing FDB file into a template containing only the column names and types, and then supply this file to `sqlite-to-fdb`:

```shell
$ cargo run --example template-fdb <input fdb> <output template fdb>
$ cargo run --example sqlite-to-fdb <input sqlite> <output fdb> --template <input template fdb> 
```