/// A string that can fit eight bytes
///
/// The rest (unused) bytes are null-padded. This means that the
/// null byte (`\0`) cannot be used in the string proper.
pub struct String64(u64);

/// Convert to `String64` if it fits
pub fn to(s: &str) -> Option<String64> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
