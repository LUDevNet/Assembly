//! Reading data directly from a buffer
use thiserror::Error;
use displaydoc::Display;

/// Errors from casting a minimally-aligned type
#[derive(Debug, Error, Display)]
pub enum CastError {
    /// Some byte between start and end was outside of the given buffer
    OutOfBounds {
        /// The offset that failed
        offset: u32,
    },
}

/// Asserts that the type has a minimal ABI alignment of `1`
///
/// ## Safety
///
/// Implementor need to verify that [`std::mem::align_of`]`::<Self>() == 1`
pub unsafe trait MinimallyAligned: Sized {}

/// Cast a buffer to a reference
///
/// ## Panics
///
/// - If the `[offset, offset + size_of::<Self>]` is not contained by the buffer
pub fn cast<T: MinimallyAligned>(buffer: &[u8], offset: u32) -> &T {
    try_cast(buffer, offset).unwrap()
}

/// Try to cast a buffer to a reference
pub fn try_cast<T: MinimallyAligned>(buffer: &[u8], offset: u32) -> Result<&T, CastError> {
    let base = buffer.as_ptr();
    let len = buffer.len();

    if offset as usize + std::mem::size_of::<T>() <= len {
        unsafe {
            let addr = base.offset(offset as isize);
            Ok(&*(addr as *const T))
        }
    } else {
        Err(CastError::OutOfBounds { offset })
    }
}

/// Cast a buffer to a slice
///
/// ## Panics
///
/// - If the `[offset, offset + len]` is not contained by the buffer
pub fn cast_slice<T: MinimallyAligned>(buffer: &[u8], offset: u32, len: u32) -> &[T] {
    try_cast_slice(buffer, offset, len).unwrap()
}

/// Try to cast a buffer to a slice
pub fn try_cast_slice<T: MinimallyAligned>(buffer: &[u8], offset: u32, len: u32) -> Result<&[T], CastError> {
    let base = buffer.as_ptr();
    let buf_len = buffer.len();

    let ulen = len as usize;
    let needed = std::mem::size_of::<T>() * ulen;

    if offset as usize + needed <= buf_len {
        unsafe {
            let addr = base.offset(offset as isize) as *const T;
            Ok(std::slice::from_raw_parts(addr, ulen))
        }
    } else {
        Err(CastError::OutOfBounds { offset })
    }
}

/// Similar to `From<&U> for T`
pub trait Repr {
    /// The value that this struct encodes
    type Value;

    /// extract the contained value
    fn extract(&self) -> Self::Value;
}

/// little-endian u16
#[repr(C, align(1))]
pub struct LEU16([u8; 2]);

/// little-endian u32
#[repr(C, align(1))]
pub struct LEU32([u8; 4]);

/// little-endian u64
#[repr(C, align(1))]
pub struct LEI64([u8; 8]);

unsafe impl MinimallyAligned for LEU16 {}

impl Repr for LEU16 {
    type Value = u16;
    fn extract(&self) -> Self::Value {
        u16::from_le_bytes(self.0)
    }
}

unsafe impl MinimallyAligned for LEU32 { }

impl Repr for LEU32 {
    type Value = u32;
    fn extract(&self) -> Self::Value {
        u32::from_le_bytes(self.0)
    }
}

unsafe impl MinimallyAligned for LEI64 { }

impl Repr for LEI64 {
    type Value = i64;
    fn extract(&self) -> Self::Value {
        i64::from_le_bytes(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let buffer: &[u8] = &[0, 20, 0, 30, 0, 1, 1];
        let le: &LEU16 = cast(buffer, 1);
        assert_eq!(le.extract(), 20);
        let le: &LEU16 = cast(buffer, 3);
        assert_eq!(le.extract(), 30);
        let le: &LEU16 = cast(buffer, 5);
        assert_eq!(le.extract(), 257);

        let les: &[LEU16] = cast_slice(buffer, 1, 3);
        assert_eq!(les[0].extract(), 20);
        assert_eq!(les[1].extract(), 30);
        assert_eq!(les[2].extract(), 257);

        assert_eq!(std::mem::align_of::<LEU16>(), 1);
    }
}
