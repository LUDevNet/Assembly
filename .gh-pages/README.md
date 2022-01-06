# Assembly RS

*Assembly* is a set of libraries and tools that can be used to process data from the defunct
game LEGO Universe. This library is used in tools such as [LUnpack](https://lu-dev.net/LUnpack)
or [ParadoxServer](https://github.com/LUDevNet/ParadoxServer).

## Modules

The library is split into several modules depending on the type of file to process:

- [`assembly-core`](assembly_core/) for generic data
- [`assembly-data`](assembly_data/) for database content
- [`assembly-fdb`](assembly_fdb/) for the "flat-file" DB export format
- [`assembly-maps`](assembly_maps/) for zone and level files
- [`assembly-pack`](assembly_pack/) for patcher and packaging
- [`assembly-xml`](assembly_xml/) for various XML formats

