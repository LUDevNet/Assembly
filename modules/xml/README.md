# assembly-xml

This crate contains the definitions of multiple XML formats used in the
game LEGO Universe.

## Example Tools

This crate comes with a selection of example tools that can
be installed using:

```shell
$ cargo install --examples assembly-xml
```

### xmldb-tree

Print the names of all tables and their columns:

```shell
$ cargo run --example xmldb-tree <file>
```

### dump-cfg

Check whether a UniverseConfig.svc file is well formed

```shell
$ dump-cfg EnvironmentInfo.xml
```

### dump-locale-prefix

Print a subtree of a locale file

```shell
$ dump-locale-prefix res/locale/Locale.xml Preconditions_
```
