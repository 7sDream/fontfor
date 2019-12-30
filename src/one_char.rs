use {
    std::{
        str::FromStr,
        string::ToString,
        convert::TryFrom,
        hint::unreachable_unchecked,
    }
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OneChar(char);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseError {
    EmptyInput,
    InvalidDigitInRadix(u32),
    UTF8BytesEmpty,
    UTF8BytesStrLengthNotDividedByTwo,
    UTF8BytesParseResultHasMultipleChar,
    InvalidCharacter,
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl OneChar {
    pub fn from_codepoint(codepoint: u32) -> Result<Self, ParseError> {
        char::try_from(codepoint).map(Self).map_err(|_|ParseError::InvalidCharacter)
    }

    pub fn from_codepoint_str_radix(s: &str, radix: u32) -> Result<Self, ParseError> {
        Self::from_codepoint(u32::from_str_radix(s, radix).map_err(|_| ParseError::InvalidDigitInRadix(radix))?)
    }

    pub fn from_codepoint_str_bin(s: &str) -> Result<Self, ParseError> {
        Self::from_codepoint_str_radix(s, 2)
    }

    pub fn from_codepoint_str_oct(s: &str) -> Result<Self, ParseError> {
        Self::from_codepoint_str_radix(s, 8)
    }

    pub fn from_codepoint_str_dec(s: &str) -> Result<Self, ParseError> {
        Self::from_codepoint_str_radix(s, 10)
    }

    pub fn from_codepoint_str_hex(s: &str) -> Result<Self, ParseError> {
        Self::from_codepoint_str_radix(s, 16)
    }

    pub fn from_utf8_bytes(s: &str) -> Result<Self, ParseError> {
        let mut chars = s.chars();
        let chunks = (0..)
            .map(|_| chars.by_ref().take(2).collect::<String>())
            .take_while(|s| !s.is_empty());

        let utf8 = String::from_utf8(
            chunks
                .map(|two_digits| -> Result<u8, ParseError> {
                    let mut digits = two_digits.chars();
                    let c1 = digits.next().unwrap(); // at least one char because of the `take_while`
                    let c2 = digits.next().ok_or(ParseError::UTF8BytesStrLengthNotDividedByTwo)?;
                    if c1.is_ascii_hexdigit() && c2.is_ascii_hexdigit() {
                        #[allow(clippy::cast_possible_truncation)] // because two digit hex meets u8 type
                        Ok((c1.to_digit(16).unwrap() * 16 + c2.to_digit(16).unwrap()) as u8)
                    } else {
                        Err(ParseError::InvalidDigitInRadix(16))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
        ).map_err(|_| ParseError::InvalidCharacter)?;

        let mut iter = utf8.chars();

        match (iter.next(), iter.next()) {
            (Some(c), None) => Ok(Self(c)),
            (None, None) => Err(ParseError::UTF8BytesEmpty),
            (Some(_), Some(_)) => Err(ParseError::UTF8BytesParseResultHasMultipleChar),
            // Because an iterator can not produce value after returning None
            (None, Some(_)) => unsafe { unreachable_unchecked() }
        }
    }
}

impl FromStr for OneChar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self,ParseError> {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (None, None) => Err(ParseError::EmptyInput),
            (Some(c), None) => Ok(Self(c)),
            (Some(c1), Some(c2)) => {
                let c1 = c1.to_ascii_lowercase();
                let c2 = c2.to_ascii_lowercase();
                match (c1, c2) {
                    ('0', 'b') => Self::from_codepoint_str_bin(&s[2..]),
                    ('0', 'o') => Self::from_codepoint_str_oct(&s[2..]),
                    ('u', '+')  => Self::from_codepoint_str_hex(&s[2..]),
                    ('0', 'x') => Self::from_utf8_bytes(&s[2..]),
                    _ => Self::from_codepoint_str_dec(s),
                }
            }
            // Because an iterator can not produce value after returning None
            (None, Some(_)) => unsafe { unreachable_unchecked() }
        }
    }
}
