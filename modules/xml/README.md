# assembly-xml

This crate contains the definitions of multiple XML formats used in the
game LEGO Universe.

## Installation

```shell
$ cargo install --examples assembly-xml
```

## Examples

Check whether a UniverseConfig.svc file is well formed

```shell
$ dump-cfg EnvironmentInfo.xml
```

Print a subtree of a locale file

```shell
$ dump-locale-prefix res/locale/Locale.xml Preconditions_
```
