use std::{ops::Deref, rc::Rc};

use assembly_fdb_raw::{ule::HeaderULE, FDBHeader};
use yoke::{Yoke, ZeroCopyFrom};
use zerovec::ule::AsULE;

pub enum CowULE<'a, T: AsULE> {
    Owned(T),
    Borrowed(&'a T::ULE),
}

impl<'a, T: AsULE + Clone> CowULE<'a, T> {
    pub fn to_owned(&self) -> T {
        match self {
            Self::Owned(o) => o.clone(),
            Self::Borrowed(b) => T::from_unaligned(b),
        }
    }

    pub fn into_owned(self) -> T {
        match self {
            Self::Owned(o) => o,
            Self::Borrowed(b) => T::from_unaligned(b),
        }
    }
}

pub struct Database<'a> {
    tables: CowULE<'a, FDBHeader>,
}

super::yoke_impl!(Database);

impl Database<'static> {
    pub fn new_box(cart: Box<[u8]>) -> Yoke<Self, Box<[u8]>> {
        Yoke::attach_to_box_cart(cart)
    }

    pub fn new_rc(cart: Rc<[u8]>) -> Yoke<Self, Rc<[u8]>> {
        Yoke::attach_to_rc_cart(cart)
    }
}

pub fn seek<T: AsULE>(buf: &[u8], off: usize) -> &T::ULE {
    assert!(buf.len() >= std::mem::size_of::<T::ULE>() + off);
    let p = buf as *const [u8] as *const u8 as *const T::ULE;
    unsafe { &*p.add(off) }
}

pub fn seek_slice<T: AsULE>(buf: &[u8], off: usize, len: usize) -> &[T::ULE] {
    let need = len * std::mem::size_of::<T::ULE>();
    assert!(buf.len() >= need + off);
    let p = buf as *const [u8] as *const u8 as *const T::ULE;
    unsafe { std::slice::from_raw_parts(p.add(off), len) }
}

impl<'data> ZeroCopyFrom<&'data HeaderULE> for Database<'static> {
    fn zero_copy_from<'b>(cart: &'b &'data HeaderULE) -> Database<'b> {
        Database {
            tables: CowULE::Borrowed(*cart),
        }
    }
}

impl<'data> ZeroCopyFrom<[u8]> for Database<'static> {
    fn zero_copy_from(cart: &[u8]) -> Database {
        let head = seek::<FDBHeader>(cart, 0);
        Database {
            tables: CowULE::Borrowed(head),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, rc::Rc};

    use assembly_fdb_raw::FDBTableHeader;
    use yoke::{Yoke, Yokeable};

    use crate::{ctx::WithCtx, loaded, unloaded::seek_slice};

    use super::Database;

    type DB<'a> = <Database<'static> as Yokeable<'a>>::Output;
    type LDB<'a> = <loaded::Database<'static> as Yokeable<'a>>::Output;

    fn project_foo<'this, 'a>(db: &'this DB<'a>, t: &'this [u8], v: PhantomData<&'a ()>) -> LDB<'a> {
        todo!()
    }

    #[test]
    fn test() {
        let data: &[u8] = &[0, 0, 0, 0, 8, 0, 0, 0];
        let cart: Rc<[u8]> = Rc::from(data);
        let db = Database::new_rc(cart);
        let r = db.backing_cart().as_ref();
        let _y = db.project_cloned_with_capture(r, project_foo);

        /*

        let v = db.get();
        let c = db.backing_cart().as_ref();
        let h = v.tables.to_owned();
        let off = h.tables.base_offset.usize();
        let len = h.tables.count as usize;
        let tables = seek_slice::<FDBTableHeader>(c, off, len);
        */
        //type DB = WithCtx<'static, loaded::Database<'static>>;
        //let _ldb = Yoke::<DB, _>::attach_to_rc_cart(cart);

        //_ldb.project_cloned_with_capture(100, )
    }
}
