use alloc::{borrow::Cow, str::from_utf8, string::String, vec::Vec};

use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, value::BytesDeserializer};

use crate::error::DeError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
enum State {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

/// Removes quotes and escapes charaters, given input is already validated
/// by MapAccess.
fn extract_value(input: &[u8]) -> Result<Vec<u8>, DeError> {
    let mut val: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let c = input[i];
        match c {
            b'\'' | b'"' => (),
            b'\\' => {
                i += 1;
                val.push(input[i]);
            }
            _ => val.push(c),
        }
        i += 1;
    }

    Ok(val)
}

pub struct ValueDeserializer<'a> {
    input: &'a [u8],
}

impl<'de> de::Deserializer<'de> for &mut ValueDeserializer<'de> {
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
        let bool = match self.input {
            b"true" => true,
            b"false" => false,
            _ => return Err(DeError::ValueNotBoolean),
        };

        visitor.visit_bool(bool)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let i8 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotI8)?;

        visitor.visit_i8(i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let i16 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotI16)?;

        visitor.visit_i16(i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let i32 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotI32)?;

        visitor.visit_i32(i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let i64 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotI64)?;

        visitor.visit_i64(i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let u8 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotU8)?;

        visitor.visit_u8(u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let u16 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotU16)?;

        visitor.visit_u16(u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let u32 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotU32)?;

        visitor.visit_u32(u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let u64 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotU64)?;

        visitor.visit_u64(u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let f32 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotF32)?;

        visitor.visit_f32(f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let f64 = lexical_core::parse(&value).map_err(|_| DeError::ValueNotF64)?;

        visitor.visit_f64(f64)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        let utf8 = from_utf8(&value).map_err(|_| DeError::InvalidUtf8)?;
        if utf8.len() != 1 {
            return Err(DeError::ValueNotChar);
        }
        visitor.visit_str(utf8)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        visitor.visit_str(from_utf8(&value).map_err(|_| DeError::InvalidUtf8)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        visitor.visit_string(String::from_utf8(value).map_err(|_| DeError::InvalidUtf8)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        visitor.visit_bytes(&value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        visitor.visit_byte_buf(value)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        if value == b"None" {
            return visitor.visit_none();
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = extract_value(self.input)?;
        if value == b"Unit" {
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
    parser: P,
    input: &'a [u8],
    /// Offset into input that has been deserialized.
    pos: usize,
}

impl<'de, P> de::Deserializer<'de> for &mut Deserializer<'de, P>
where
    P: Parser,
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
            end_pos: 0,
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
    /// End offset of the current pair, exclusive.
    end_pos: usize,
}

impl<'a, 'de, P> MapAccess<'de> for VarMapAccess<'a, 'de, P>
where
    P: Parser,
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

        let (equal_pos, end_pos) = {
            let mut i = 0;

            match input[i] {
                b'\0' => return Err(DeError::NullByte),
                b'=' => return Err(DeError::EmptyKey),
                c if c.is_ascii_digit() => return Err(DeError::KeyStartsWithDigit),
                c if !(c.is_ascii_alphabetic() || c == b'_') => return Err(DeError::InvalidKey),
                _ => i += 1,
            }

            let equal_pos = {
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
                i
            };

            let mut state = State::Unquoted;
            while i < input.len() {
                let c = input[i];
                if c == b'\0' {
                    return Err(DeError::NullByte);
                }
                match state {
                    State::Unquoted => match c {
                        b'\\' => {
                            i += 1;
                            if i >= input.len() {
                                return Err(DeError::ValueDanglingEscape);
                            }
                        }
                        b'\'' => state = State::SingleQuoted,
                        b'"' => state = State::DoubleQuoted,
                        c if self.de.parser.is_pair_separator(c) => break,
                        _ => {}
                    },
                    State::SingleQuoted if c == b'\'' => state = State::Unquoted,
                    State::DoubleQuoted => match c {
                        b'\\' => i += 1,
                        b'"' => state = State::Unquoted,
                        _ => {}
                    },
                    _ => {}
                }
                i += 1;
            }
            match state {
                State::SingleQuoted => return Err(DeError::ValueUnterminatedSingleQuote),
                State::DoubleQuoted => return Err(DeError::ValueUnterminatedDoubleQuote),
                State::Unquoted => (equal_pos, i),
            }
        };

        self.equal_pos = equal_pos;
        self.end_pos = end_pos;

        seed.deserialize(BytesDeserializer::new(&input[..equal_pos]))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let input = &self.de.input[self.de.pos..];
        self.de.pos += self.end_pos;

        seed.deserialize(&mut ValueDeserializer {
            input: &input[(self.equal_pos + 1)..self.end_pos],
        })
    }
}

pub trait Parser {
    #[inline]
    fn is_pair_separator(&self, c: u8) -> bool {
        matches!(c, b' ' | b'\t' | b'\n' | b';')
    }

    #[inline]
    fn none_repr(&self) -> &'static [u8] {
        b"None"
    }

    #[inline]
    fn unit_repr(&self) -> &'static [u8] {
        b"Unit"
    }
}

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

        let b: Test = from_bytes(b"a='bytes'\nb=0xFFFF\nc=12").unwrap();

        println!("{}", b.a);
    }
}
