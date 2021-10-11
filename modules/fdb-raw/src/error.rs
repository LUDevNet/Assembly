use core::fmt;

#[derive(Debug)]
/// Error indicating that bytes could not be cast to a slice.
///
/// This happends when the number of bytes given is not a multiple of `std::mem::size_of::<T>()`
pub struct ModuloMismatch {
    pub(crate) input_len: usize,
    pub(crate) type_size: usize,
}

impl fmt::Display for ModuloMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Could not cast slice: {} is not a multiple of {}",
            self.input_len, self.type_size
        )
    }
}
