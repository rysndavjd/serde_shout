use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};

use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, value::BytesDeserializer};

use crate::{
    error::Error,
    std::{iter::Map, mem::take, slice::SplitInclusive, str::from_utf8},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum State {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

fn parse_line<'a>(v: &'a [u8]) -> Result<(&'a [u8], Cow<'a, [u8]>), Error> {
    if v.is_empty() {
        return Err(Error::EmptyString);
    }

    let equal_pos = v
        .iter()
        .position(|b| b == &b'=')
        .ok_or(Error::MissingEqualSeparator)?;

    let key = &v[..equal_pos];
    let val = &v[(equal_pos + 1)..];

    let mut key_iter = key.iter().enumerate();

    match key_iter.next() {
        Some((_, c)) if c.is_ascii_digit() => {
            return Err(Error::IdentifierStartsWithDigit);
        }
        Some((i, c)) if !(c.is_ascii_alphabetic() || c == &b'_') => {
            return Err(Error::InvalidIdentifier { index: i });
        }
        None => return Err(Error::EmptyIdentifier),
        _ => (),
    }

    for (i, c) in key_iter {
        if !(c.is_ascii_alphanumeric() || c == &b'_') {
            return Err(Error::InvalidIdentifier { index: i });
        }
    }

    if !val.iter().any(|b| {
        b == &b'"'
            || b == &b'\''
            || b == &b'\\'
            || b == &b'|'
            || b == &b'&'
            || b == &b';'
            || b == &b'<'
            || b == &b'>'
            || b == &b'('
            || b == &b')'
            || b == &b'`'
            || b == &b' '
            || b == &b'\t'
            || b == &b'\n'
    }) {
        return Ok((key, Cow::Borrowed(val)));
    }

    let mut output: Vec<u8> = Vec::new();
    let chars = &mut val.iter().enumerate();
    let mut state = State::Unquoted;

    while let Some((i, c)) = chars.next() {
        if c == &b'\\' && state != State::SingleQuoted {
            match chars.next() {
                Some((_, escaped_char)) => {
                    output.push(*escaped_char);
                    continue;
                }
                None => return Err(Error::ValueDanglingEscape),
            }
        }

        match state {
            State::Unquoted => match c {
                &b'\'' => {
                    state = State::SingleQuoted;
                    continue;
                }
                &b'"' => {
                    state = State::DoubleQuoted;
                    continue;
                }
                &b'|' | &b'&' | &b';' | &b'<' | &b'>' | &b'(' | &b')' | &b'`' | &b' ' | &b'\t'
                | b'\n' => {
                    return Err(Error::ValueUnescapedShellChar { index: i });
                }
                _ => {}
            },
            State::SingleQuoted if c == &b'\'' => {
                state = State::Unquoted;
                continue;
            }
            State::DoubleQuoted if c == &b'"' => {
                state = State::Unquoted;
                continue;
            }
            _ => (),
        }

        output.push(*c);
    }

    match state {
        State::SingleQuoted => {
            return Err(Error::ValueUnterminatedSingleQuote);
        }
        State::DoubleQuoted => {
            return Err(Error::ValueUnterminatedDoubleQuote);
        }
        _ => (),
    }

    Ok((key, Cow::Owned(output)))
}

pub struct ValueDeserializer<'a> {
    raw: Cow<'a, [u8]>,
}

impl<'de> de::Deserializer<'de> for &mut ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let bool = match self.raw.as_ref() {
            b"true" => true,
            b"false" => false,
            _ => return Err(Error::ValueNotBoolean),
        };

        visitor.visit_bool(bool)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i8 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotI8)?;

        visitor.visit_i8(i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i16 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotI16)?;

        visitor.visit_i16(i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i32 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotI32)?;

        visitor.visit_i32(i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i64 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotI64)?;

        visitor.visit_i64(i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u8 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotU8)?;

        visitor.visit_u8(u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u16 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotU16)?;

        visitor.visit_u16(u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u32 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotU32)?;

        visitor.visit_u32(u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u64 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotU64)?;

        visitor.visit_u64(u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f32 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotF32)?;

        visitor.visit_f32(f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f64 = lexical_core::parse(&self.raw).map_err(|_| Error::ValueNotF64)?;

        visitor.visit_f64(f64)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &self.raw {
            Cow::Borrowed(s) => {
                visitor.visit_borrowed_str(from_utf8(s).map_err(|_| Error::InvaildUtf8)?)
            }
            Cow::Owned(s) => visitor.visit_str(from_utf8(s).map_err(|_| Error::InvaildUtf8)?),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &mut self.raw {
            Cow::Borrowed(s) => {
                visitor.visit_string(from_utf8(s).map_err(|_| Error::InvaildUtf8)?.to_owned())
            }
            Cow::Owned(s) => {
                visitor.visit_string(String::from_utf8(s.to_vec()).map_err(|_| Error::InvaildUtf8)?)
            }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &self.raw {
            Cow::Borrowed(b) => visitor.visit_borrowed_bytes(b),
            Cow::Owned(b) => visitor.visit_bytes(b),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &mut self.raw {
            Cow::Borrowed(b) => visitor.visit_borrowed_bytes(b),
            Cow::Owned(b) => visitor.visit_byte_buf(take(b)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.raw.is_empty() {
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.raw.is_empty() {
            return visitor.visit_unit();
        }

        Err(Error::ValueNotUnit)
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
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
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
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
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
        Err(Error::UnsupportedDeserialization)
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
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
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
}

impl<'a> Deserializer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
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
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(VarMapAccess::new(self.input))
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
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }
}

struct ByteLines<'a>(Map<SplitInclusive<'a, u8, fn(&u8) -> bool>, fn(&'a [u8]) -> &'a [u8]>);

impl<'a> ByteLines<'a> {
    fn is_newline(c: &u8) -> bool {
        c == &b'\n'
    }

    fn strip_newlines(line: &[u8]) -> &[u8] {
        let Some(line) = line.strip_suffix(b"\n") else {
            return line;
        };
        let Some(line) = line.strip_suffix(b"\r") else {
            return line;
        };
        line
    }

    fn new(s: &'a [u8]) -> ByteLines<'a> {
        ByteLines(
            s.split_inclusive(ByteLines::is_newline as fn(&u8) -> bool)
                .map(ByteLines::strip_newlines as fn(&'a [u8]) -> &'a [u8]),
        )
    }
}

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<&'a [u8]> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<&'a [u8]> {
        self.next_back()
    }
}

impl<'a> DoubleEndedIterator for ByteLines<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [u8]> {
        self.0.next_back()
    }
}

struct VarMapAccess<'a> {
    lines: ByteLines<'a>,
    current_value: Option<Cow<'a, [u8]>>,
}

impl<'a> VarMapAccess<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        VarMapAccess {
            lines: ByteLines::new(s),
            current_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for VarMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        for line in self.lines.by_ref() {
            let trimmed = line.trim_ascii();
            if trimmed.is_empty() || trimmed.starts_with(b"#") {
                continue;
            }

            let (key, val) = parse_line(trimmed)?;

            self.current_value = Some(val);

            return seed.deserialize(BytesDeserializer::new(key)).map(Some);
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = self
            .current_value
            .take()
            .ok_or_else(|| Error::Custom("value called before key".into()))?;

        seed.deserialize(&mut ValueDeserializer { raw: val })
    }
}

pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, Error> {
    T::deserialize(&mut Deserializer::new(input.as_bytes()))
}

pub fn from_bytes<'de, T: Deserialize<'de>>(input: &'de [u8]) -> Result<T, Error> {
    T::deserialize(&mut Deserializer::new(input))
}

#[cfg(test)]
mod tests {
    use std::{println, string::String};

    use serde::Deserialize;

    use super::{Error, parse_line};
    use crate::{de::from_bytes, from_str};

    #[test]
    fn invalid_str() {
        assert_eq!(parse_line(b""), Err(Error::EmptyString));
        assert_eq!(
            parse_line(b"just some string"),
            Err(Error::MissingEqualSeparator)
        );
        assert_eq!(parse_line(b"=value"), Err(Error::EmptyIdentifier));
        assert_eq!(
            parse_line(b"1=value"),
            Err(Error::IdentifierStartsWithDigit)
        );
        assert_eq!(
            parse_line(b"\xE2\x9D\x8C=\xE2\x9D\x93"),
            Err(Error::InvalidIdentifier { index: 0 })
        );
        assert_eq!(
            parse_line(b"value='Hello World"),
            Err(Error::ValueUnterminatedSingleQuote)
        );
        assert_eq!(
            parse_line(b"Identifier=Hello\\ World\'"),
            Err(Error::ValueUnterminatedSingleQuote)
        );
        assert_eq!(
            parse_line(b"Identifier=\'Hello\'\\ \'World"),
            Err(Error::ValueUnterminatedSingleQuote)
        );

        assert_eq!(
            parse_line(b"Identifier=\"Hello World"),
            Err(Error::ValueUnterminatedDoubleQuote)
        );
        assert_eq!(
            parse_line(b"Identifier=Hello\\ World\""),
            Err(Error::ValueUnterminatedDoubleQuote)
        );
        assert_eq!(
            parse_line(b"Identifier=\"Hello\"\\ \"World"),
            Err(Error::ValueUnterminatedDoubleQuote)
        );

        assert_eq!(
            parse_line(b"Identifier=Hello\\"),
            Err(Error::ValueDanglingEscape)
        );
    }

    #[test]
    fn deserialize() {
        #[derive(Deserialize, Debug)]
        struct Test {
            a: String,
            b: String,
            c: u8,
        }

        let t: Test = from_str("a=whata\nb=whatb\nc=12").unwrap();
        let b: Test = from_bytes(b"a=bytes\nb=0xFFFF\nc=12").unwrap();

        // println!("{:?}", t);
        // println!("{:?}", b);
    }
}
