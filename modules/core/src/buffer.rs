//! Reading data directly from a buffer

/// Mark this type aligned to a single byte so that it can be used to adress a struct
/// anywhere in a buffer
pub unsafe trait Unaligned: Sized {
    /// The value that this struct encodes
    type Value;
    /// extract the contained value
    fn extract(&self) -> Self::Value;

    /// Cast a buffer to a reference
    fn cast(buffer: &[u8], offset: u32) -> &Self {
        let base = buffer.as_ptr();
        let len = buffer.len();

        if offset as usize + std::mem::size_of::<Self>() <= len {
            unsafe {
                let addr = base.offset(offset as isize);
                &*(addr as *const Self)
            }
        } else {
            panic!("Out of bounds")
        }
    }

    /// Cast a buffer to a slice
    fn cast_slice(buffer: &[u8], offset: u32, len: u32) -> &[Self] {
        let base = buffer.as_ptr();
        let buf_len = buffer.len();

        let ulen = len as usize;
        let needed = std::mem::size_of::<Self>() * ulen;

        if offset as usize + needed <= buf_len {
            unsafe {
                let addr = base.offset(offset as isize) as *const Self;
                std::slice::from_raw_parts(addr, ulen)
            }
        } else {
            panic!("Out of bounds")
        }
    }
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

unsafe impl Unaligned for LEU16 {
    type Value = u16;
    fn extract(&self) -> Self::Value {
        u16::from_le_bytes(self.0)
    }
}

unsafe impl Unaligned for LEU32 {
    type Value = u32;
    fn extract(&self) -> Self::Value {
        u32::from_le_bytes(self.0)
    }
}

unsafe impl Unaligned for LEI64 {
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
        let le = LEU16::cast(buffer, 1);
        assert_eq!(le.extract(), 20);
        let le = LEU16::cast(buffer, 3);
        assert_eq!(le.extract(), 30);
        let le = LEU16::cast(buffer, 5);
        assert_eq!(le.extract(), 257);

        let les = LEU16::cast_slice(buffer, 1, 3);
        assert_eq!(les[0].extract(), 20);
        assert_eq!(les[1].extract(), 30);
        assert_eq!(les[2].extract(), 257);

        assert_eq!(std::mem::align_of::<LEU16>(), 1);
    }
}
