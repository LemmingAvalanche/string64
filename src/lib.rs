/// A string that can fit eight bytes
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

///
/// The rest (unused) bytes are null-padded. This means that the
/// null byte (`\0`) cannot be used in the string proper.
#[derive(Debug, PartialEq, Eq)]
pub struct String64(u64);

impl String64 {
    /// Convert to `String64` if it fits and it contains no null bytes
    // NOTE can’t be `const` due to the for loop
    pub fn new(s: &str) -> Option<String64> {
        if s.len() > 8 || s.contains("\0") {
            return None;
        }
        let mut res = 0u64;
        assert!(res == 0);
        let mut counter = 56;
        for b in s.bytes() {
            res += (b as u64) << counter;
            counter -= 8;
        }
        Some(String64(res))
    }

    fn new_alt1(s: &str) -> Option<String64> {
        if s.len() > 8 || s.contains("\0") {
            return None;
        }
        let mut array = [0u8; 8];
        s.bytes()
            .zip(array.iter_mut())
            .for_each(|(b, ptr)| *ptr = b);
        Some(String64(u64::from_be_bytes(array)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_result() {
        let s = "hello w";
        let actual1 = String64::new(&s);
        let actual2 = String64::new(&s);
        assert!(actual1 != None);
        assert_eq!(actual1, actual2);
    }

    #[test]
    fn null_byte_string_is_none() {
        let actual = String64::new("\0\0");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_string_is_none() {
        let actual = String64::new("Good day to you from the other side of the World!");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn a_repeated_string() {
        let actual = String64::new("aaaaaaaa");
        let expected = Some(String64(0x6161616161616161));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unused_is_null_padded() {
        let actual = String64::new("aaaa");
        let expected = Some(String64(0x6161616100000000));
        assert_eq!(actual, expected);
    }

    #[test]
    fn four_a_with_ring_above() {
        let actual = String64::new("åååå");
        let expected = Some(String64(0xc3a5c3a5c3a5c3a5));
        assert_eq!(actual, expected);
    }

    #[test]
    fn five_a_with_ring_above_is_too_long() {
        let actual = String64::new("ååååå");
        assert_eq!(actual, None);
    }

    #[quickcheck]
    fn new_and_alt1(s: String) -> bool {
        String64::new(&s) == String64::new_alt1(&s)
    }
}
