/// A string that can fit eight bytes
///
/// The rest (unused) bytes are null-padded. This means that the
/// null byte (`\0`) cannot be used in the string proper.
#[derive(Debug, PartialEq, Eq)]
pub struct String64(u64);

/// Convert to `String64` if it fits
pub fn to(s: &str) -> Option<String64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_result() {
        let s = "hello w";
        let actual1 = to(&s);
        let actual2 = to(&s);
        assert!(actual1 != None);
        assert_eq!(actual1, actual2);
    }

    #[test]
    fn null_byte_string_is_none() {
        let actual = to("\0\0");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_string_is_none() {
        let actual = to("Good day to you from the other side of the World!");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn a_repeated_string() {
        let actual = to("aaaaaaaa");
        let expected = Some(String64(0x6161616161616161));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unused_is_null_padded() {
        let actual = to("aaaa");
        let expected = Some(String64(0x6161616100000000));
        assert_eq!(actual, expected);
    }
}
