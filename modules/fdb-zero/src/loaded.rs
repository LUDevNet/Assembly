use assembly_fdb_raw::{ule::TableHeaderULE, FDBHeader, FDBTableHeader};
use yoke::ZeroCopyFrom;
use zerovec::{ule::AsULE, ZeroVec};

use crate::{
    ctx::WithCtx,
    unloaded::{seek, seek_slice},
};

pub struct Database<'a> {
    pub tables: ZeroVec<'a, FDBTableHeader>,
}

impl ZeroCopyFrom<[u8]> for Database<'static> {
    fn zero_copy_from<'b>(cart: &'b [u8]) -> Database<'b> {
        let h = seek::<FDBHeader>(cart, 0);
        let h = FDBHeader::from_unaligned(h);
        let off = h.tables.base_offset.usize();
        let len = h.tables.count as usize;
        let tables: &'b [TableHeaderULE] = seek_slice::<FDBTableHeader>(cart, off, len);
        Database {
            tables: ZeroVec::Borrowed(tables),
        }
    }
}

impl ZeroCopyFrom<[u8]> for WithCtx<'static, Database<'static>> {
    fn zero_copy_from<'b>(cart: &'b [u8]) -> WithCtx<'b, Database<'b>> {
        let h = seek::<FDBHeader>(cart, 0);
        let h = FDBHeader::from_unaligned(h);
        let off = h.tables.base_offset.usize();
        let len = h.tables.count as usize;
        let tables: &'b [TableHeaderULE] = seek_slice::<FDBTableHeader>(cart, off, len);
        let db: Database<'b> = Database {
            tables: ZeroVec::Borrowed(tables),
        };
        WithCtx::new(cart, db)
    }
}

super::yoke_impl!(Database);
