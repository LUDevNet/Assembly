use std::cmp::Ordering;

/// Compares two name strings
///
/// ## Safety
///
/// This panics if name_bytes does not contains a null terminator
pub(crate) fn compare_bytes(bytes: &[u8], name_bytes: &[u8]) -> Ordering {
    for i in 0..bytes.len() {
        match name_bytes[i].cmp(&bytes[i]) {
            Ordering::Equal => {}
            Ordering::Less => {
                // the null terminator is a special case of this one
                return Ordering::Less;
            }
            Ordering::Greater => {
                return Ordering::Greater;
            }
        }
    }
    if name_bytes[bytes.len()] == 0 {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
