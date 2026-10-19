use crate::{error::SerError, std::num::FpCategory};
use alloc::{string::String, vec::Vec};
use lexical_core::FormattedSize;
use serde::ser::{self, Impossible, Serialize};

#[cfg(feature = "std")]
use std::io::Write;

#[cfg(feature = "no_std")]
use embedded_io::Write;

/// Validates given key if
/// - starts with `A-Z`, `a-z`, or `_`
/// - contains only `A-Z`, `a-z`, `0-9`, or `_` after that
/// - is not empty
///
/// [POSIX specification](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap03.html#tag_03_3.216)
struct MapKeySerializer<'a, W: 'a, F: Formatter> {
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> ser::Serializer for MapKeySerializer<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if v.is_empty() {
            return Err(SerError::EmptyKey);
        }

        let mut characters = v.iter();

        if let Some(c) = characters.next() {
            if c.is_ascii_digit() {
                return Err(SerError::KeyStartsWithDigit);
            }
            if !(c.is_ascii_alphabetic() || *c == b'_') {
                return Err(SerError::InvalidKey);
            }
        }

        for c in characters {
            if !(c.is_ascii_alphanumeric() || *c == b'_') {
                return Err(SerError::InvalidKey);
            }
        }

        self.ser.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }
}

pub struct Compound<'a, W: 'a, F: Formatter> {
    ser: &'a mut Serializer<W, F>,
    first: bool,
}

impl<'a, W, F> ser::SerializeMap for Compound<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.first {
            self.ser.formatter.split_pair(&mut self.ser.writer)?;
        }
        self.ser.formatter.begin_pair(&mut self.ser.writer)?;

        self.first = false;
        key.serialize(MapKeySerializer { ser: self.ser })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.ser.writer.write_all(b"=")?;
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.in_map = false;
        self.ser.formatter.end_document(&mut self.ser.writer)?;
        Ok(())
    }
}

impl<'a, W, F> ser::SerializeStruct for Compound<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}

impl<'a, W, F> ser::SerializeSeq for Compound<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.first {
            self.ser.formatter.split_seq(&mut self.ser.writer)?;
        }
        self.first = false;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.in_seq = false;
        self.ser.formatter.end_seq(&mut self.ser.writer)
    }
}

impl<'a, W, F> ser::SerializeTuple for Compound<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W, F> ser::SerializeTupleStruct for Compound<'a, W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

/// A serde serializer that writes shell assignments.
///
/// Quotes if necessary a string so a POSIX shell reads it back as exactly
/// the same string, with no expansion, globbing, or word splitting.
///
/// Only single quotes are used for quoting, which keeps the output portable
/// and should be immune to any shell-quirks even if they follow the POSIX
/// specification loosely.
///
/// [POSIX specification](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/V3_chap02.html)
pub struct Serializer<W, F> {
    writer: W,
    formatter: F,
    in_seq: bool,
    in_map: bool,
}

impl<W> Serializer<W, PosixCompactFormatter>
where
    W: Write,
{
    /// Creates a serializer that writes space separated POSIX assignments.
    #[inline]
    pub fn new_compact_posix(writer: W) -> Self {
        Serializer::with_formatter(writer, PosixCompactFormatter)
    }
}

impl<W> Serializer<W, BashCompactFormatter>
where
    W: Write,
{
    /// Creates a serializer that writes space separated assignments with
    /// Bash style `(a b c)` arrays.
    #[inline]
    pub fn new_compact_bash(writer: W) -> Self {
        Serializer::with_formatter(writer, BashCompactFormatter)
    }
}

impl<W> Serializer<W, PosixPrettyFormatter>
where
    W: Write,
{
    /// Creates a serializer that writes one POSIX assignment per line.
    #[inline]
    pub fn new_pretty_posix(writer: W) -> Self {
        Serializer::with_formatter(writer, PosixPrettyFormatter)
    }
}

impl<W> Serializer<W, BashPrettyFormatter>
where
    W: Write,
{
    /// Creates a serializer that writes one assignment per line with
    /// Bash style `(a b c)` arrays.
    #[inline]
    pub fn new_pretty_bash(writer: W) -> Self {
        Serializer::with_formatter(writer, BashPrettyFormatter)
    }
}

impl<W, F> Serializer<W, F>
where
    W: Write,
    F: Formatter,
{
    /// Creates a serializer that writes to `writer` using [`Formatter`].
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer {
            writer,
            formatter,
            in_seq: false,
            in_map: false,
        }
    }

    /// Consumes the serializer and returns the underlying writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = ();

    type Error = SerError;

    type SerializeSeq = Compound<'a, W, F>;
    type SerializeTuple = Compound<'a, W, F>;
    type SerializeTupleStruct = Compound<'a, W, F>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Compound<'a, W, F>;
    type SerializeStruct = Compound<'a, W, F>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_bool(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_i8(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_i16(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_i32(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_i64(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_i128(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_u8(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_u16(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_u32(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_u64(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_u128(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_f32(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_f64(&mut self.writer, v)?;

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if v.is_empty() {
            if self.in_seq && self.formatter.escape_seq() {
                self.writer.write_all(b"'\\'''\\''")?;
            } else {
                self.writer.write_all(b"''")?;
            }
            return Ok(());
        }

        let mut borrow = true;
        for c in v {
            match c {
                b'+'
                | b'-'
                | b'.'
                | b'/'
                | b':'
                | b'@'
                | b']'
                | b'_'
                | b'0'..=b'9'
                | b'A'..=b'Z'
                | b'a'..=b'z' => (),
                b'\0' => return Err(SerError::NullByte),
                _ => {
                    borrow = false;
                }
            };
        }

        if borrow {
            self.writer.write_all(v)?;
            return Ok(());
        }

        let escape = self.in_seq && self.formatter.escape_seq();

        if escape {
            self.writer.write_all(b"'\\''")?;
        } else {
            self.writer.write_all(b"'")?;
        }

        let mut parts = v.split(|&c| c == b'\'');
        if let Some(first) = parts.next() {
            self.writer.write_all(first)?;
        }
        for part in parts {
            self.writer.write_all(b"'\\''")?;
            self.writer.write_all(part)?;
        }

        if escape {
            self.writer.write_all(b"'\\''")?;
        } else {
            self.writer.write_all(b"'")?;
        }

        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.formatter.write_none(&mut self.writer)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.write_unit(&mut self.writer)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.in_seq {
            // Serializing a sequence within a sequence is unsupported
            return Err(SerError::UnsupportedSerialization);
        }
        self.in_seq = true;
        self.formatter.begin_seq(&mut self.writer)?;

        Ok(Compound {
            ser: self,
            first: true,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.in_map || self.in_seq {
            return Err(SerError::UnsupportedSerialization);
        }
        self.in_map = true;
        Ok(Compound {
            ser: self,
            first: true,
        })
    }

    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::UnsupportedSerialization)
    }
}

/// Controls the syntax of the output written by [`Serializer`].
///
/// Built in formatters:
///
/// | Formatter                  | Pairs           | Sequences       |
/// |----------------------------|-----------------|-----------------|
/// | [`PosixCompactFormatter`]  | space separated | one quoted word |
/// | [`PosixPrettyFormatter`]   | one per line    | one quoted word |
/// | [`BashCompactFormatter`]   | space separated | Bash array      |
/// | [`BashPrettyFormatter`]    | one per line    | Bash array      |
/// | [`ExportCompactFormatter`] | `;` separated   | one quoted word |
/// | [`ExportPrettyFormatter`]  | one per line    | one quoted word |
pub trait Formatter {
    /// Writes [`Option::None`] to the specified writer.
    #[inline]
    fn write_none<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"None")?;
        Ok(())
    }

    /// Writes [`unit`] to the specified writer.
    #[inline]
    fn write_unit<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"Unit")?;
        Ok(())
    }

    /// Writes a [`true`] or [`false`] value to the specified writer.
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, v: bool) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let s = if v {
            b"true".as_slice()
        } else {
            b"false".as_slice()
        };
        writer.write_all(s)?;
        Ok(())
    }

    /// Writes an integer [`i8`] to the specified writer.
    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, v: i8) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; i8::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`i16`] to the specified writer.
    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, v: i16) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; i16::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`i32`] to the specified writer.
    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, v: i32) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; i32::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`i64`] to the specified writer.
    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, v: i64) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; i64::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`i128`] to the specified writer.
    #[inline]
    fn write_i128<W>(&mut self, writer: &mut W, v: i128) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; i128::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`u8`] to the specified writer.
    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, v: u8) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; u8::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`u16`] to the specified writer.
    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, v: u16) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; u16::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`u32`] to the specified writer.
    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, v: u32) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; u32::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`u64`] to the specified writer.
    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, v: u64) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; u64::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes an integer [`u128`] to the specified writer.
    #[inline]
    fn write_u128<W>(&mut self, writer: &mut W, v: u128) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        let mut buf = [0u8; u128::FORMATTED_SIZE];
        writer.write_all(lexical_core::write(v, &mut buf))?;
        Ok(())
    }

    /// Writes a floating point [`f32`] to the specified writer.
    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, v: f32) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(SerError::FloatNotFinite);
            }
            _ => {
                let mut buf = [0u8; f32::FORMATTED_SIZE_DECIMAL];
                writer.write_all(lexical_core::write(v, &mut buf))?;
            }
        }

        Ok(())
    }

    /// Writes a floating point [`f64`] to the specified writer.
    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, v: f64) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(SerError::FloatNotFinite);
            }
            _ => {
                let mut buf = [0u8; f64::FORMATTED_SIZE_DECIMAL];
                writer.write_all(lexical_core::write(v, &mut buf))?;
            }
        }

        Ok(())
    }

    /// Writes text before each key, including the first one.
    ///
    /// # Example
    ///
    /// With `export ` ([`ExportPrettyFormatter`]):
    ///
    /// ```text
    /// export crate=serde_shout
    /// export version=1
    /// ```
    #[inline]
    fn begin_pair<W>(&mut self, _writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    /// Writes the separator between two key value pairs, not after the last
    /// one, so the output has no trailing separator.
    ///
    /// # Example
    ///
    /// With a space ([`PosixCompactFormatter`]):
    ///
    /// ```text
    /// crate=serde_shout version=1 owner=rysndavjd
    /// ```
    ///
    /// With a newline ([`PosixPrettyFormatter`]):
    ///
    /// ```text
    /// crate=serde_shout
    /// version=1
    /// owner=rysndavjd
    /// ```
    #[inline]
    fn split_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b" ")?;
        Ok(())
    }

    /// Whether values that need quoting inside a sequence are wrapped in an
    /// escaped quote (`'\''`) instead of a plain quote (`'`).
    ///
    /// # Example
    ///
    /// For the elements `a b` and `c`:
    ///
    /// With `true` ([`PosixCompactFormatter`]):
    ///
    /// ```text
    /// list=''\''a b'\'' c'
    /// ```
    ///
    /// With `false` ([`BashCompactFormatter`]):
    ///
    /// ```text
    /// list=('a b' c)
    /// ```
    #[inline]
    fn escape_seq(&self) -> bool {
        true
    }

    /// Writes the text that opens a sequence, before the first element.
    ///
    /// # Example
    ///
    /// With a `'` ([`PosixCompactFormatter`]):
    ///
    /// ```text
    /// list='1 2 3'
    /// ```
    ///
    /// With a `(` ([`BashCompactFormatter`]):
    ///
    /// ```text
    /// list=(1 2 3)
    /// ```
    #[inline]
    fn begin_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"'")?;
        Ok(())
    }

    /// Writes the separator between two sequence elements, not before the
    /// first or after the last.
    #[inline]
    fn split_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b" ")?;
        Ok(())
    }

    /// Writes the text that closes a sequence, after the last element.
    ///
    /// # Example
    ///
    /// With a `'` ([`PosixCompactFormatter`]):
    ///
    /// ```text
    /// list='1 2 3'
    /// ```
    ///
    /// With a `)` ([`BashCompactFormatter`]):
    ///
    /// ```text
    /// list=(1 2 3)
    /// ```
    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"'")?;
        Ok(())
    }

    /// Writes text after the last key value pair.
    ///
    /// # Example
    ///
    /// With a newline ([`PosixPrettyFormatter`]):
    ///
    /// ```text
    /// crate=serde_shout
    /// version=1
    /// ```
    #[inline]
    fn end_document<W>(&mut self, _writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }
}

/// Writes pairs separated by a space, with sequences as a quoted word.
///
/// Output should be valid in any POSIX shell.
///
/// ```text
/// name=app ports='80 443' tags=''\''a b'\'' c'
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct PosixCompactFormatter;

impl Formatter for PosixCompactFormatter {}

/// Writes pairs separated by a space, with sequences as Bash arrays.
///
/// Arrays are not part of POSIX, so the output needs Bash, Zsh or a similar
/// shell.
///
/// ```text
/// name=app ports=(80 443) tags=('a b' c)
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct BashCompactFormatter;

impl Formatter for BashCompactFormatter {
    #[inline]
    fn escape_seq(&self) -> bool {
        false
    }

    fn begin_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"(")?;
        Ok(())
    }

    fn end_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b")")?;
        Ok(())
    }
}

/// Writes one pair per line, with sequences as a quoted word.
///
/// Output should be valid in any POSIX shell.
///
/// ```text
/// name=app
/// ports='80 443'
/// tags=''\''a b'\'' c'
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct PosixPrettyFormatter;

impl Formatter for PosixPrettyFormatter {
    fn split_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn end_document<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }
}

/// Writes one pair per line, with sequences as Bash arrays.
///
/// Arrays are not part of POSIX, so the output needs Bash, Zsh or a similar
/// shell.
///
/// ```text
/// name=app
/// ports=(80 443)
/// tags=('a b' c)
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct BashPrettyFormatter;

impl Formatter for BashPrettyFormatter {
    fn split_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }

    #[inline]
    fn escape_seq(&self) -> bool {
        false
    }

    fn begin_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"(")?;
        Ok(())
    }

    fn end_seq<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b")")?;
        Ok(())
    }

    fn end_document<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }
}

/// Writes `export` statement separated by a `;`, with sequences as a quoted word.
///
/// ```text
/// export name=app;export ports='80 443'
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct ExportCompactFormatter;

impl Formatter for ExportCompactFormatter {
    fn split_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b";")?;
        Ok(())
    }

    fn begin_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"export ")?;
        Ok(())
    }
}

/// Writes one `export` statement per line, with sequences as a quoted word.
///
/// ```text
/// export name=app
/// export ports='80 443'
/// ```
#[derive(Debug, Copy, Clone, Default, Hash)]
pub struct ExportPrettyFormatter;

impl Formatter for ExportPrettyFormatter {
    fn split_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn begin_pair<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"export ")?;
        Ok(())
    }

    fn end_document<W>(&mut self, writer: &mut W) -> Result<(), SerError>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        Ok(())
    }
}

/// Serializes `value` into `writer` using given [`Formatter`].
///
/// # Example
///
/// ```
/// use std::io::stdout;
/// use serde_shout::{to_writer, PosixPrettyFormatter};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Debug {
///     fn_name: String,
///     code: u64,
/// }
///
/// let error = Debug {
///     fn_name: String::from("to_string"),
///     code: 1,
/// };
///
/// to_writer(stdout(), PosixPrettyFormatter, &error);
/// ```
pub fn to_writer<W, F, T>(writer: W, formatter: F, value: &T) -> Result<(), SerError>
where
    W: Write,
    F: Formatter,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::with_formatter(writer, formatter);
    value.serialize(&mut ser)
}

/// Serializes `value` into a [`Vec<u8>`] using given [`Formatter`].
///
/// # Example
///
/// ```
/// use std::io::stdout;
/// use serde_shout::{to_vec, PosixCompactFormatter};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Packat {
///     ipv4: [u8; 4],
///     service: String,
///     owner: String,
///     version: f32,
/// }
///
/// let packat = Packat {
///     ipv4: [1, 1, 1, 1],
///     service: String::from("DNS"),
///     owner: String::from("Cloudflare, Inc."),
///     version: 2.1,
/// };
///
/// let output = to_vec(PosixCompactFormatter, &packat).unwrap();
///
/// assert_eq!(output, b"ipv4='1 1 1 1' service=DNS owner='Cloudflare, Inc.' version=2.1".to_vec())
/// ```
pub fn to_vec<F, T>(formatter: F, value: &T) -> Result<Vec<u8>, SerError>
where
    F: Formatter,
    T: ?Sized + Serialize,
{
    let mut writer = Vec::new();
    to_writer(&mut writer, formatter, value)?;
    Ok(writer)
}

/// Serializes `value` into a [`String`] using given [`Formatter`].
///
/// # Example
///
/// ```
/// use std::io::stdout;
/// use serde_shout::{to_string, BashPrettyFormatter};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Device {
///     path: String,
///     size: u64
/// }
///
/// let dev = Device {
///     path: String::from("/dev/sda"),
///     size: 500000000000
/// };
///
/// let output = to_string(BashPrettyFormatter, &dev).unwrap();
///
/// assert_eq!(output, String::from("path=/dev/sda\nsize=500000000000\n"))
/// ```
pub fn to_string<F, T>(formatter: F, value: &T) -> Result<String, SerError>
where
    F: Formatter,
    T: ?Sized + Serialize,
{
    let vec = to_vec(formatter, value)?;
    String::from_utf8(vec).map_err(|_| SerError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;
    use core::f64::consts::PI;

    use super::*;

    fn key(k: &str) -> Result<String, SerError> {
        let mut map = BTreeMap::new();
        map.insert(k, 1u8);
        to_string(PosixCompactFormatter, &map)
    }

    fn value(input: &str) -> Result<String, SerError> {
        to_string(PosixCompactFormatter, &input)
    }

    #[test]
    fn key_valid() {
        assert_eq!(key("key"), Ok(String::from("key=1")));
        assert_eq!(key("_unused"), Ok(String::from("_unused=1")));
        assert_eq!(key("Object_Type1"), Ok(String::from("Object_Type1=1")));
    }

    #[test]
    fn key_invalid() {
        assert_eq!(key(""), Err(SerError::EmptyKey));
        assert_eq!(key("1a"), Err(SerError::KeyStartsWithDigit));
        assert_eq!(key("-a"), Err(SerError::InvalidKey));
        assert_eq!(key("\u{e9}"), Err(SerError::InvalidKey));
        assert_eq!(key("a b"), Err(SerError::InvalidKey));
        assert_eq!(key("a=b"), Err(SerError::InvalidKey));
    }

    #[test]
    fn value_valid() {
        assert_eq!(value("a+b-c.d/e:f@g]h_i09").unwrap(), "a+b-c.d/e:f@g]h_i09");

        assert_eq!(value("").unwrap(), "''");
        assert_eq!(value("test").unwrap(), "test");
        assert_eq!(value("121").unwrap(), "121");
        assert_eq!(value("hello world").unwrap(), "'hello world'");
        assert_eq!(value("$HOME").unwrap(), "'$HOME'");
        assert_eq!(value("a\\b").unwrap(), "'a\\b'");
        assert_eq!(value("say \"hi\"").unwrap(), "'say \"hi\"'");
        assert_eq!(value("a\nb").unwrap(), "'a\nb'");
        assert_eq!(value("it's").unwrap(), "'it'\\''s'");
        assert_eq!(value("it's $5").unwrap(), "'it'\\''s $5'");
        assert_eq!(value("it's a\\b").unwrap(), "'it'\\''s a\\b'");
        assert_eq!(value("it's \"x\"").unwrap(), "'it'\\''s \"x\"'");
        assert_eq!(value("it's\nmore").unwrap(), "'it'\\''s\nmore'");
    }

    #[test]
    fn value_invalid() {
        assert_eq!(value("a\0b"), Err(SerError::NullByte));
        assert_eq!(value("a b\0"), Err(SerError::NullByte));
    }

    #[test]
    fn empty_in_seq() {
        let items = ["", "a"].to_vec();
        assert_eq!(
            to_string(PosixCompactFormatter, &items).unwrap(),
            "''\\'''\\'' a'"
        );
        assert_eq!(to_string(BashCompactFormatter, &items).unwrap(), "('' a)");
    }

    #[derive(serde::Serialize)]
    struct ValidExample {
        unit: (),
        option_none: Option<()>,
        integer: u64,
        float: f64,
        string: String,
        numbers: Vec<u8>,
        strings: Vec<&'static str>,
    }

    fn valid_example() -> ValidExample {
        ValidExample {
            unit: (),
            option_none: None,
            integer: 256,
            float: PI,
            string: String::from("Example"),
            numbers: [10, 5, 64].to_vec(),
            strings: ["\"Non Nested\"", "String", "Are working", "'¯\\_(ツ)_/¯'"].to_vec(),
        }
    }

    #[test]
    fn posix_compact() {
        assert_eq!(
            to_string(PosixCompactFormatter, &valid_example()).unwrap(),
            "unit=Unit option_none=None integer=256 float=3.141592653589793 string=Example numbers='10 5 64' strings=''\\''\"Non Nested\"'\\'' String '\\''Are working'\\'' '\\'''\\''¯\\_(ツ)_/¯'\\'''\\'''"
        );
    }

    #[test]
    fn posix_pretty() {
        assert_eq!(
            to_string(PosixPrettyFormatter, &valid_example()).unwrap(),
            "unit=Unit
option_none=None
integer=256
float=3.141592653589793
string=Example
numbers='10 5 64'
strings=''\\''\"Non Nested\"'\\'' String '\\''Are working'\\'' '\\'''\\''¯\\_(ツ)_/¯'\\'''\\'''
"
        );
    }

    #[test]
    fn bash_compact() {
        assert_eq!(
            to_string(BashCompactFormatter, &valid_example()).unwrap(),
            "unit=Unit option_none=None integer=256 float=3.141592653589793 string=Example numbers=(10 5 64) strings=('\"Non Nested\"' String 'Are working' ''\\''¯\\_(ツ)_/¯'\\''')"
        );
    }

    #[test]
    fn bash_pretty() {
        assert_eq!(
            to_string(BashPrettyFormatter, &valid_example()).unwrap(),
            "unit=Unit
option_none=None
integer=256
float=3.141592653589793
string=Example
numbers=(10 5 64)
strings=('\"Non Nested\"' String 'Are working' ''\\''¯\\_(ツ)_/¯'\\''')
"
        );
    }

    #[test]
    fn export_compact() {
        assert_eq!(
            to_string(ExportCompactFormatter, &valid_example()).unwrap(),
            "export unit=Unit;export option_none=None;export integer=256;export float=3.141592653589793;export string=Example;export numbers='10 5 64';export strings=''\\''\"Non Nested\"'\\'' String '\\''Are working'\\'' '\\'''\\''¯\\_(ツ)_/¯'\\'''\\'''"
        );
    }

    #[test]
    fn export_pretty() {
        assert_eq!(
            to_string(ExportPrettyFormatter, &valid_example()).unwrap(),
            "export unit=Unit
export option_none=None
export integer=256
export float=3.141592653589793
export string=Example
export numbers='10 5 64'
export strings=''\\''\"Non Nested\"'\\'' String '\\''Are working'\\'' '\\'''\\''¯\\_(ツ)_/¯'\\'''\\'''
"
        );
    }
}
