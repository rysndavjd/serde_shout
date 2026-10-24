use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};

use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, value::BytesDeserializer};

use crate::{
    error::DeError,
    std::{iter::Map, mem::take, slice::Split, str::from_utf8},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum State {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

// fn parse_line<'a>(v: &'a [u8]) -> Result<(&'a [u8], Cow<'a, [u8]>), DeError> {
//     if v.is_empty() {
//         return Err(DeError::EmptyString);
//     }

//     let equal_pos = v
//         .iter()
//         .position(|b| b == &b'=')
//         .ok_or(DeError::MissingEqualSeparator)?;

//     let key = &v[..equal_pos];
//     let val = &v[(equal_pos + 1)..];

//     let mut key_iter = key.iter().enumerate();

//     match key_iter.next() {
//         Some((_, c)) if c.is_ascii_digit() => {
//             return Err(DeError::IdentifierStartsWithDigit);
//         }
//         Some((i, c)) if !(c.is_ascii_alphabetic() || c == &b'_') => {
//             return Err(DeError::InvalidIdentifier { index: i });
//         }
//         None => return Err(DeError::EmptyIdentifier),
//         _ => (),
//     }

//     for (i, c) in key_iter {
//         if !(c.is_ascii_alphanumeric() || c == &b'_') {
//             return Err(DeError::InvalidIdentifier { index: i });
//         }
//     }

//     if !val.iter().any(|b| {
//         b == &b'"'
//             || b == &b'\''
//             || b == &b'\\'
//             || b == &b'|'
//             || b == &b'&'
//             || b == &b';'
//             || b == &b'<'
//             || b == &b'>'
//             || b == &b'('
//             || b == &b')'
//             || b == &b'`'
//             || b == &b' '
//             || b == &b'\t'
//             || b == &b'\n'
//     }) {
//         return Ok((key, Cow::Borrowed(val)));
//     }

//     let mut output: Vec<u8> = Vec::new();
//     let chars = &mut val.iter().enumerate();
//     let mut state = State::Unquoted;

//     while let Some((i, c)) = chars.next() {
//         if c == &b'\\' && state != State::SingleQuoted {
//             match chars.next() {
//                 Some((_, escaped_char)) => {
//                     output.push(*escaped_char);
//                     continue;
//                 }
//                 None => return Err(DeError::ValueDanglingEscape),
//             }
//         }

//         match state {
//             State::Unquoted => match c {
//                 &b'\'' => {
//                     state = State::SingleQuoted;
//                     continue;
//                 }
//                 &b'"' => {
//                     state = State::DoubleQuoted;
//                     continue;
//                 }
//                 &b'|' | &b'&' | &b';' | &b'<' | &b'>' | &b'(' | &b')' | &b'`' | &b' ' | &b'\t'
//                 | b'\n' => {
//                     return Err(DeError::ValueUnescapedShellChar { index: i });
//                 }
//                 _ => {}
//             },
//             State::SingleQuoted if c == &b'\'' => {
//                 state = State::Unquoted;
//                 continue;
//             }
//             State::DoubleQuoted if c == &b'"' => {
//                 state = State::Unquoted;
//                 continue;
//             }
//             _ => (),
//         }

//         output.push(*c);
//     }

//     match state {
//         State::SingleQuoted => {
//             return Err(DeError::ValueUnterminatedSingleQuote);
//         }
//         State::DoubleQuoted => {
//             return Err(DeError::ValueUnterminatedDoubleQuote);
//         }
//         _ => (),
//     }

//     Ok((key, Cow::Owned(output)))
// }

/// (equal_pos, end_pos)
fn next_pair(input: &[u8]) -> Result<(usize, usize), DeError> {
    if input.is_empty() {
        return Err(DeError::EmptyString);
    }

    let mut i = 0;

    match input[i] {
        b'\0' => return Err(DeError::NullByte),
        b'=' => return Err(DeError::EmptyKey),
        c if c.is_ascii_digit() => return Err(DeError::KeyStartsWithDigit),
        c if !(c.is_ascii_alphabetic() || c == b'_') => return Err(DeError::InvalidKey),
        _ => i += 1,
    }

    let mut equal_pos: Option<usize> = None;
    while equal_pos.is_none() && i < input.len() {
        match input[i] {
            b'\0' => return Err(DeError::NullByte),
            b'=' => equal_pos = Some(i),
            c if !(c.is_ascii_alphanumeric() || c == b'_') => return Err(DeError::InvalidKey),
            _ => i += 1,
        }
    }
    if equal_pos.is_none() {
        return Err(DeError::MissingEqualSeparator);
    }

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
                b' ' | b'\t' | b'\n' | b';' => break,
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
        State::SingleQuoted => Err(DeError::ValueUnterminatedSingleQuote),
        State::DoubleQuoted => Err(DeError::ValueUnterminatedDoubleQuote),
        State::Unquoted => Ok((
            equal_pos.expect("Checked by `if equal_pos.is_none() { ... }`"),
            i,
        )),
    }
}

pub struct ValueDeserializer<'a> {
    input: &'a [u8],
    pos: usize,
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
        let i8 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotI8)?;

        visitor.visit_i8(i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i16 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotI16)?;

        visitor.visit_i16(i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i32 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotI32)?;

        visitor.visit_i32(i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i64 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotI64)?;

        visitor.visit_i64(i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u8 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotU8)?;

        visitor.visit_u8(u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u16 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotU16)?;

        visitor.visit_u16(u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u32 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotU32)?;

        visitor.visit_u32(u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u64 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotU64)?;

        visitor.visit_u64(u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f32 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotF32)?;

        visitor.visit_f32(f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f64 = lexical_core::parse(self.input).map_err(|_| DeError::ValueNotF64)?;

        visitor.visit_f64(f64)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeError::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // match &self.input {
        //     Cow::Borrowed(s) => {
        //         visitor.visit_borrowed_str(from_utf8(s).map_err(|_| DeError::InvalidUtf8)?)
        //     }
        //     Cow::Owned(s) => visitor.visit_str(from_utf8(s).map_err(|_| DeError::InvalidUtf8)?),
        // }
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // match &mut self.input {
        //     Cow::Borrowed(s) => {
        //         visitor.visit_string(from_utf8(s).map_err(|_| DeError::InvalidUtf8)?.to_owned())
        //     }
        //     Cow::Owned(s) => visitor
        //         .visit_string(String::from_utf8(s.to_vec()).map_err(|_| DeError::InvalidUtf8)?),
        // }

        visitor
            .visit_string(String::from_utf8(self.input.to_vec()).map_err(|_| DeError::InvalidUtf8)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // match &self.raw {
        //     Cow::Borrowed(b) => visitor.visit_borrowed_bytes(b),
        //     Cow::Owned(b) => visitor.visit_bytes(b),
        // }
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // match &mut self.raw {
        //     Cow::Borrowed(b) => visitor.visit_borrowed_bytes(b),
        //     Cow::Owned(b) => visitor.visit_byte_buf(take(b)),
        // }
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input == b"None" {
            return visitor.visit_none();
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // if self.raw.as_ref() == b"Unit" {
        //     return visitor.visit_unit();
        // }

        // Err(DeError::ValueNotUnit)
        todo!()
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

pub struct Deserializer<'a> {
    input: &'a [u8],
    /// Offset into input that has been deserialized.
    pos: usize,
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
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

struct VarMapAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    /// Relative offset into current key where the equal is.
    equal_pos: usize,
}

impl<'a, 'de> MapAccess<'de> for VarMapAccess<'a, 'de> {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
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

        seed.deserialize(BytesDeserializer::new(&self.de.input[self.de.pos..i]))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // There nothing after the `=` so we return early.
        let input = if self.de.input[self.equal_pos..].len() == 1 {
            &[]
        } else {
            &self.de.input[self.equal_pos..]
        };

        seed.deserialize(&mut ValueDeserializer { input, pos: 0 })
    }
}

// let input = &self.de.input[self.de.pos..];

// let mut borrow = true;
// for c in &input[(self.equal_pos + 1)..] {
//     match c {
//         b'\0' => return Err(DeError::NullByte),
//         // I will probably expand this allow list to avoid allocation.
//         b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => (),
//         _ => borrow = false,
//     }
// }
// // There is no special charaters so no allocation is needed and we return early.
// if borrow {
//     return seed.deserialize(&mut ValueDeserializer { input, pos: 0 });
// }

// let mut val: Vec<u8> = Vec::new();
// let mut i = self.equal_pos;
// let mut state = State::Unquoted;
// while i < input.len() {
//     let c = input[i];
//     match state {
//         State::Unquoted => match c {
//             b'\\' => {
//                 i += 1;
//                 if i >= input.len() {
//                     return Err(DeError::ValueDanglingEscape);
//                 }
//             }
//             b'\'' => state = State::SingleQuoted,
//             b'"' => state = State::DoubleQuoted,
//             b' ' | b'\t' | b'\n' | b';' => break,
//             _ => {}
//         },
//         State::SingleQuoted if c == b'\'' => state = State::Unquoted,
//         State::DoubleQuoted => match c {
//             b'\\' => i += 1,
//             b'"' => state = State::Unquoted,
//             _ => {}
//         },
//         _ => {}
//     }
//     i += 1;
//     val.push(c);
// }
// match state {
//     State::SingleQuoted => return Err(DeError::ValueUnterminatedSingleQuote),
//     State::DoubleQuoted => return Err(DeError::ValueUnterminatedDoubleQuote),
//     State::Unquoted => (),
// };

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

// pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, DeError> {
//     T::deserialize(&mut Deserializer::new(input.as_bytes()))
// }

pub fn from_bytes<'de, T: Deserialize<'de>>(input: &'de [u8]) -> Result<T, DeError> {
    T::deserialize(&mut Deserializer { input, pos: 0 })
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
        }

        let b: Test = from_bytes(b"a=bytes").unwrap();

        println!("{:?}", b);
    }
}
