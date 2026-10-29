#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, vec::Vec};

use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, value::BytesDeserializer};

use crate::{error::DeError, std::str::from_utf8};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
enum State {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

struct ValueDeserializer<'a, P> {
    #[cfg(not(feature = "alloc"))]
    input: &'a [u8],
    #[cfg(feature = "alloc")]
    input: Cow<'a, [u8]>,
    parser: P,
}

impl<'de, P> de::Deserializer<'de> for &mut ValueDeserializer<'de, P>
where
    P: Parser + Copy,
{
    type Error = DeError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Needed for `alloc` feature
        #[allow(clippy::useless_asref)]
        let bool = if self.input.as_ref() == self.parser.true_repr() {
            true
        } else if self.input.as_ref() == self.parser.false_repr() {
            false
        } else {
            return Err(DeError::ValueNotBoolean);
        };

        visitor.visit_bool(bool)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parser.parse_i8(self.input.as_ref())?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parser.parse_i16(self.input.as_ref())?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parser.parse_i32(self.input.as_ref())?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parser.parse_i64(self.input.as_ref())?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parser.parse_u8(self.input.as_ref())?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parser.parse_u16(self.input.as_ref())?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parser.parse_u32(self.input.as_ref())?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parser.parse_u64(self.input.as_ref())?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.parser.parse_f32(self.input.as_ref())?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.parser.parse_f64(self.input.as_ref())?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Needed for `alloc` feature
        #[allow(clippy::useless_asref)]
        let utf8 = from_utf8(self.input.as_ref()).map_err(|_| DeError::InvalidUtf8)?;
        if utf8.len() != 1 {
            return Err(DeError::ValueNotChar);
        }
        visitor.visit_str(utf8)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Needed for `alloc` feature
        #[allow(clippy::useless_asref)]
        visitor.visit_str(from_utf8(self.input.as_ref()).map_err(|_| DeError::InvalidUtf8)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bytes(self.input.as_ref())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.input.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Needed for `alloc` feature
        #[allow(clippy::useless_asref)]
        if self.input.as_ref() == self.parser.none_repr() {
            return visitor.visit_none();
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // Needed for `alloc` feature
        #[allow(clippy::useless_asref)]
        if self.input.as_ref() == self.parser.unit_repr() {
            return visitor.visit_unit();
        }

        Err(DeError::ValueNotUnit)
    }

    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

pub struct Deserializer<'a, P> {
    input: &'a [u8],
    parser: P,
    /// Offset into input that has been deserialized.
    pos: usize,
}

impl<'de, P> de::Deserializer<'de> for &mut Deserializer<'de, P>
where
    P: Parser + Copy,
{
    type Error = DeError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(VarMapAccess {
            de: self,
            equal_pos: 0,
        })
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }
}

struct VarMapAccess<'a, 'de, P: Parser> {
    de: &'a mut Deserializer<'de, P>,
    /// Offset of '=' relative to the current pair.
    equal_pos: usize,
}

impl<'a, 'de, P> MapAccess<'de> for VarMapAccess<'a, 'de, P>
where
    P: Parser + Copy,
{
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        while self.de.pos < self.de.input.len()
            && self.de.parser.is_pair_separator(self.de.input[self.de.pos])
        {
            self.de.pos += 1;
        }

        let input = &self.de.input[self.de.pos..];
        if input.is_empty() {
            return Ok(None);
        }

        let mut i = 0;
        match input[i] {
            b'\0' => return Err(DeError::NullByte),
            b'=' => return Err(DeError::EmptyKey),
            c if c.is_ascii_digit() => return Err(DeError::KeyStartsWithDigit),
            c if !(c.is_ascii_alphabetic() || c == b'_') => return Err(DeError::InvalidKey),
            _ => i += 1,
        }

        let mut found = false;
        while !found && i < input.len() {
            match input[i] {
                b'\0' => return Err(DeError::NullByte),
                b'=' => found = true,
                c if !(c.is_ascii_alphanumeric() || c == b'_') => {
                    return Err(DeError::InvalidKey);
                }
                _ => i += 1,
            }
        }
        if !found {
            return Err(DeError::MissingEqualSeparator);
        }
        self.equal_pos = i;

        seed.deserialize(BytesDeserializer::new(&input[..self.equal_pos]))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val_start = self.equal_pos + 1;
        let rest = &self.de.input[self.de.pos + val_start..];

        let mut val_len = 0;
        let mut borrow = true;

        for &b in rest {
            match b {
                b'\0' => return Err(DeError::NullByte),
                b'\'' | b'\\' | b'"' => {
                    borrow = false;
                    break;
                }
                b if self.de.parser.is_pair_separator(b) => break,
                _ => val_len += 1,
            }
        }

        #[cfg(not(feature = "alloc"))]
        {
            if borrow {
                self.de.pos += val_start + val_len;
                return seed.deserialize(&mut ValueDeserializer {
                    input: &rest[..val_len],
                    parser: self.de.parser,
                });
            }

            Err(DeError::AllocationRequired)
        }

        #[cfg(feature = "alloc")]
        {
            if borrow {
                self.de.pos += val_start + val_len;
                return seed.deserialize(&mut ValueDeserializer {
                    input: Cow::Borrowed(&rest[..val_len]),
                    parser: self.de.parser,
                });
            }

            val_len = 0;
            let mut val = Vec::new();
            let mut state = State::Unquoted;
            let mut iter = rest.iter().copied();

            while let Some(c) = iter.next() {
                if c == b'\0' {
                    return Err(DeError::NullByte);
                }

                if c == b'\\' && state != State::SingleQuoted {
                    match iter.next() {
                        Some(escaped_char) => {
                            if escaped_char == b'\0' {
                                return Err(DeError::NullByte);
                            }
                            val.push(escaped_char);
                            val_len += 2;
                            continue;
                        }
                        None => return Err(DeError::ValueDanglingEscape),
                    }
                }

                match state {
                    State::Unquoted => match c {
                        b'\'' => state = State::SingleQuoted,
                        b'"' => state = State::DoubleQuoted,
                        c if self.de.parser.is_pair_separator(c) => break,
                        _ => val.push(c),
                    },
                    State::SingleQuoted => match c {
                        b'\'' => state = State::Unquoted,
                        _ => val.push(c),
                    },
                    State::DoubleQuoted => match c {
                        b'"' => state = State::Unquoted,
                        _ => val.push(c),
                    },
                }
                val_len += 1;
            }

            match state {
                State::SingleQuoted => return Err(DeError::ValueUnterminatedSingleQuote),
                State::DoubleQuoted => return Err(DeError::ValueUnterminatedDoubleQuote),
                State::Unquoted => (),
            }

            self.de.pos += val_start + val_len;
            seed.deserialize(&mut ValueDeserializer {
                input: Cow::Owned(val),
                parser: self.de.parser,
            })
        }
    }
}

pub trait Parser {
    #[inline]
    fn is_pair_separator(&self, c: u8) -> bool {
        matches!(c, b' ' | b'\t' | b'\n' | b';')
    }

    #[inline]
    fn true_repr(&self) -> &'static [u8] {
        b"true"
    }

    #[inline]
    fn false_repr(&self) -> &'static [u8] {
        b"false"
    }

    #[inline]
    fn none_repr(&self) -> &'static [u8] {
        b"None"
    }

    #[inline]
    fn unit_repr(&self) -> &'static [u8] {
        b"Unit"
    }

    #[inline]
    fn parse_i8(&self, input: &[u8]) -> Result<i8, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotI8)
    }

    #[inline]
    fn parse_i16(&self, input: &[u8]) -> Result<i16, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotI16)
    }

    #[inline]
    fn parse_i32(&self, input: &[u8]) -> Result<i32, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotI32)
    }

    #[inline]
    fn parse_i64(&self, input: &[u8]) -> Result<i64, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotI64)
    }

    #[inline]
    fn parse_u8(&self, input: &[u8]) -> Result<u8, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotU8)
    }

    #[inline]
    fn parse_u16(&self, input: &[u8]) -> Result<u16, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotU16)
    }

    #[inline]
    fn parse_u32(&self, input: &[u8]) -> Result<u32, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotU32)
    }

    #[inline]
    fn parse_u64(&self, input: &[u8]) -> Result<u64, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotU64)
    }

    #[inline]
    fn parse_f32(&self, input: &[u8]) -> Result<f32, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotF32)
    }

    #[inline]
    fn parse_f64(&self, input: &[u8]) -> Result<f64, DeError> {
        lexical_core::parse(input).map_err(|_| DeError::ValueNotF64)
    }
}

#[derive(Debug, Copy, Clone, Default, Hash)]
struct AllParser;

impl Parser for AllParser {}

// pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, DeError> {
//     T::deserialize(&mut Deserializer::new(input.as_bytes()))
// }

pub fn from_bytes<'de, T: Deserialize<'de>>(input: &'de [u8]) -> Result<T, DeError> {
    T::deserialize(&mut Deserializer {
        parser: AllParser,
        input,
        pos: 0,
    })
}

#[cfg(test)]
mod tests {

    use std::{println, string::String};

    use serde::Deserialize;

    use super::*;

    #[test]
    fn deserialize() {
        #[derive(Deserialize, Debug)]
        struct Test {
            a: String,
            b: String,
            c: u8,
        }

        let b: Test = from_bytes(b"a=by\\''test'tes\nb=0xFFFF\nc=12").unwrap();

        println!("{:?}", b);
    }
}
