//! This crate is a collection of parsers and data types
//! To enable reading of data from LEGO Universe game files.

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate nom;
extern crate encoding;

/// Core types and parsers
pub mod core;
/// Loading CoreDataBase from a FileDataBase
pub mod fdb;
/// Loading zone data from a LEGO Universe Zone file
pub mod luz;

#[cfg(test)]
mod tests {
    use crate::fdb::core::Schema;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_schema() {
        let schema = Schema::new();
        assert_eq!(schema.table("Test").is_none(), true);
    }
}
