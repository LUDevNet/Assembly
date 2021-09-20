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
Because there is no 1:1 mapping of FDB and SQLite value types, you first need to create a 'template' FDB file containing the column names and types.

```shell
$ cargo run --example template-fdb <input fdb> <output template fdb>
$ cargo run --example sqlite-to-fdb <input template fdb> <input sqlite> <output fdb>
```
