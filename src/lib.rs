#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

// Just testing that we can do it
const _world: String64 = String64::const_new("HolaM");

type Underlying = [u8; 8];

/// A string that can fit eight bytes
///
/// Unused bytes are filled with the null byte. This has the side effect of
/// e.g. `String64::new("h\0\0") == String64::new("h")`.
#[derive(Debug, PartialEq, Eq)]
pub struct String64(Underlying);

impl String64 {
    /// Convert to `String64` if it fits
    fn new(s: &str) -> Option<String64> {
        if s.len() > 8 {
            return None;
        }
        let mut array = [b'\0'; 8];
        s.bytes()
            .zip(array.iter_mut())
            .for_each(|(b, ptr)| *ptr = b);
        Some(String64(array))
    }

    pub const fn const_new(s: &str) -> String64 {
        let len = s.len();
        let bs = s.as_bytes();
        if s.len() > 8 {
            panic!("Expected size at most 8 but was greater than that");
        }
        let mut res = [0u8; 8];
        // unrolled
        if s.len() >= 1 {
            res[0] = bs[0];
        }
        if s.len() >= 2 {
            res[1] = bs[1];
        }
        if s.len() >= 3 {
            res[2] = bs[2];
        }
        if s.len() >= 4 {
            res[3] = bs[3];
        }
        if s.len() >= 5 {
            res[4] = bs[4];
        }
        if s.len() >= 6 {
            res[5] = bs[5];
        }
        if s.len() >= 7 {
            res[6] = bs[6];
        }
        if s.len() == 8 {
            res[7] = bs[7];
        }
        String64(res)
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
    fn no_surprises() {
        let actual1 = String64::new("h");
        let actual2 = String64::new("h\0\0");
        assert!(actual1 == actual2);
    }

    #[test]
    fn null_byte_string_is_empty() {
        let actual = String64::new("\0\0");
        let expected = String64::new("");
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
        let expected = Some(String64([b'a'; 8]));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unused_is_null_padded() {
        let actual = String64::new("aaaa");
        let expected = Some(String64([b'a', b'a', b'a', b'a', b'\0', b'\0', b'\0', b'\0']));
        assert_eq!(actual, expected);
    }

    #[test]
    fn four_a_with_ring_above() {
        let actual = String64::new("åååå");
        let expected = Some(String64([0xc3, 0xa5, 0xc3, 0xa5, 0xc3, 0xa5, 0xc3, 0xa5]));
        assert_eq!(actual, expected);
    }

    #[test]
    fn five_a_with_ring_above_is_too_long() {
        let actual = String64::new("ååååå");
        assert_eq!(actual, None);
    }

    #[test]
    fn endiannes() {
        let actual = String64::new("aaaabb");
        let expected = Some(String64([b'a', b'a', b'a', b'a', b'b', b'b', 0, 0]));
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn new_and_const_new(s: String) -> bool {
        let new = String64::new(&s);
        if let Some(m) = String64::new(&s) {
            m == String64::const_new(&s)
        } else {
            true
        }
    }

    #[quickcheck]
    fn unicode_strings_less_than_nine_bytes(s: String) -> bool {
        !(s.len() <= 8) || String64::new(&s).is_some()
    }

    #[test]
    fn string() {
        // NOTE Secret sauce!
        std::str::from_utf8(&[b'h', b'e', b'l', b'l', b'o']).unwrap();
    }
}
