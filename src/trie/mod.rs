pub mod classic;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Char(u8);
impl Char {
    pub const ALPHABET: usize = 26;

    pub fn index(self) -> usize {
        (self.0 - b'a') as usize
    }
}
impl TryFrom<u8> for Char {
    type Error = &'static str;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'a'..=b'z' => Ok(Char(c)),
            _ => Err("invalid character, expect [a-z]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chars(Vec<Char>);
impl TryFrom<&str> for Chars {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.as_bytes()
            .iter()
            .map(|&b| Char::try_from(b))
            .collect::<Result<Vec<_>, Self::Error>>()
            .map(|v| Chars(v))
    }
}

impl AsRef<[Char]> for Chars {
    fn as_ref(&self) -> &[Char] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Char;
    use super::Chars;

    #[test]
    fn test_char() {
        assert_eq!(Char::try_from(b'a'), Ok(Char(b'a')));
        assert!(Char::try_from(b'A').is_err());
        assert_eq!(
            Chars::try_from("hello"),
            Ok(Chars(vec![
                Char(b'h'),
                Char(b'e'),
                Char(b'l'),
                Char(b'l'),
                Char(b'o')
            ]))
        );
        assert!(Chars::try_from("Hello").is_err());
    }
}
