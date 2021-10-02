# assembly-pack

This crate contains the pack/archives related file formats of the
[assembly](https://crates.io/crates/assembly) library.

## Examples

This crate comes with some example binaries

### Installation

```shell
$ cargo install --examples assembly-pack
```

### `sd0-decode`

Decompress sd0 streams

```shell
$ sd0-decode infile.sd0 outfile
```

### `sd0-encode`

Compress sd0 streams

```shell
$ sd0-encode infile outfile.sd0
```

### `pk-crc`

Calculate the Cyclic-Redundancy-Code (CRC) for a relative file path

```shell
$ pk-crc client/res/data.xml
```

### `pk-entries`

List all entries in a PK file

```shell
$ pk-entries data.pk
```

### `pk-file`

Print a single entry from a PK file given the numeric CRC value

```shell
$ pk-file data.pk crc
```

### `pki-find`

Find a specific CRC in the PKI file

```shell
$ pki-find primary.pki crc
```

### `pki-list`

List all entries in a PKI file

```shell
# List all files
$ pki-find -f primary.pki
# List all PK archives
$ pki-find -p primary.pki
```

### `md5-sum`

Calculate the md5sum of a file

```shell
md5-sum file
```
