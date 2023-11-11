// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2020 7sDream <i@7sdre.am> and contributors
//
// This file is part of FontFor.
//
// FontFor is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{convert::TryFrom, hint::unreachable_unchecked, str::FromStr};

use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OneChar(pub char,);

impl OneChar {
    pub fn description(self,) -> String {
        let scalar_value = u32::from(self.0,);
        let mut utf8 = vec![0; self.0.len_utf8()];
        self.0.encode_utf8(utf8.as_mut_slice(),);
        let bytes = utf8.iter().map(|byte| format!("0x{:X}", byte),).collect::<Vec<_,>>();
        format!(
            "\"{}\"(U+{:0scalar_value_length$X}, {}, {})",
            self.0,
            scalar_value,
            scalar_value,
            bytes.join(" "),
            scalar_value_length = if scalar_value > 0xFFFF { 6 } else { 4 },
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Error)]
pub enum ParseError {
    #[error("you don't input anything")]
    EmptyInput,
    #[error("invalid unicode scalar value {0}")]
    InvalidUnicodeScalarValue(u32,),
    #[error("has invalid char when parse as a base {0} number")]
    InvalidDigitInRadix(u32,),
    #[error("need bytes str after `0x`")]
    UTF8BytesEmpty,
    #[error("can't align to bytes")]
    UTF8BytesStrCantAlignToBytes,
    #[error("invalid utf8 bytes")]
    UTF8BytesInvalid,
    #[error("utf8 bytes represent for more then one char")]
    UTF8BytesParseResultMoreThenOneChar,
}

impl OneChar {
    pub fn from_scalar_value(scalar_value: u32,) -> Result<Self, ParseError,> {
        #[allow(clippy::map_err_ignore)]
        char::try_from(scalar_value,)
            .map(Self,)
            .map_err(|_| ParseError::InvalidUnicodeScalarValue(scalar_value,),)
    }

    pub fn from_scalar_value_str_radix(s: &str, radix: u32,) -> Result<Self, ParseError,> {
        Self::from_scalar_value(
            #[allow(clippy::map_err_ignore)]
            u32::from_str_radix(s, radix,).map_err(|_| ParseError::InvalidDigitInRadix(radix,),)?,
        )
    }

    pub fn from_scalar_value_str_bin(s: &str,) -> Result<Self, ParseError,> {
        Self::from_scalar_value_str_radix(s, 2,)
    }

    pub fn from_scalar_value_str_oct(s: &str,) -> Result<Self, ParseError,> {
        Self::from_scalar_value_str_radix(s, 8,)
    }

    pub fn from_scalar_value_str_dec(s: &str,) -> Result<Self, ParseError,> {
        Self::from_scalar_value_str_radix(s, 10,)
    }

    pub fn from_scalar_value_str_hex(s: &str,) -> Result<Self, ParseError,> {
        Self::from_scalar_value_str_radix(s, 16,)
    }

    pub fn from_utf8_bytes(s: &str,) -> Result<Self, ParseError,> {
        let mut digits = s.chars();

        let bytes = (0..)
            .map(|_| {
                let mut byte = digits.by_ref().take(2,);
                (byte.next(), byte.next(),)
            },)
            .take_while(|(c1, _,)| c1.is_some(),)
            .map(|(c1, c2,)| -> Result<u8, ParseError,> {
                let c1 = c1.unwrap(); // at least one char because of the `take_while`
                let c2 = c2.ok_or(ParseError::UTF8BytesStrCantAlignToBytes,)?;
                if c1.is_ascii_hexdigit() && c2.is_ascii_hexdigit() {
                    #[allow(clippy::cast_possible_truncation)] // two hex digit is a 8-bit number
                    Ok((c1.to_digit(16,).unwrap() << 4 | c2.to_digit(16,).unwrap()) as u8,)
                } else {
                    Err(ParseError::InvalidDigitInRadix(16,),)
                }
            },)
            .collect::<Result<Vec<_,>, _,>>()?;

        #[allow(clippy::map_err_ignore)]
        let utf8 = String::from_utf8(bytes,).map_err(|_| ParseError::UTF8BytesInvalid,)?;

        let mut iter = utf8.chars();

        match (iter.next(), iter.next(),) {
            (Some(c,), None,) => Ok(Self(c,),),
            (None, None,) => Err(ParseError::UTF8BytesEmpty,),
            (Some(_,), Some(_,),) => Err(ParseError::UTF8BytesParseResultMoreThenOneChar,),
            // Because an iterator can not produce value after returning None
            (None, Some(_,),) => unsafe { unreachable_unchecked() },
        }
    }
}

impl FromStr for OneChar {
    type Err = ParseError;

    fn from_str(s: &str,) -> Result<Self, ParseError,> {
        let mut chars = s.chars();
        match (chars.next(), chars.next(),) {
            (None, None,) => Err(ParseError::EmptyInput,),
            (Some(c,), None,) => Ok(Self(c,),),
            (Some(c1,), Some(c2,),) => {
                let c1 = c1.to_ascii_lowercase();
                let c2 = c2.to_ascii_lowercase();
                match (c1, c2,) {
                    ('0', 'b',) => Self::from_scalar_value_str_bin(&s[2..],),
                    ('0', 'o',) => Self::from_scalar_value_str_oct(&s[2..],),
                    ('u', '+',) => Self::from_scalar_value_str_hex(&s[2..],),
                    ('0', 'x',) => Self::from_utf8_bytes(&s[2..],),
                    _ => Self::from_scalar_value_str_dec(s,),
                }
            }
            // Because an iterator can not produce value after returning None
            (None, Some(_,),) => unsafe { unreachable_unchecked() },
        }
    }
}
