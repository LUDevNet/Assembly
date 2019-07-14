//! This crate is a collection of parsers and data types
//! To enable reading of data from LEGO Universe game files.

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate nom;
//#[macro_use]
extern crate nom_methods;

pub use assembly_core as core;

//pub mod core;
pub mod fdb;
pub mod luz;
pub mod lvl;
pub mod pki;
pub mod pk;
pub mod sd0;

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
