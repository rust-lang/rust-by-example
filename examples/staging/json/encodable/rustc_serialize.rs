// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support code for encoding and decoding types.

/*
Core encoding and decoding interfaces.
*/


pub trait Encoder<E> {
    fn emit_f32(&mut self, v: f32) -> Result<(), E>;
    fn emit_f64(&mut self, v: f64) -> Result<(), E>;
    fn emit_str(&mut self, v: &str) -> Result<(), E>;

    fn emit_struct<F>(&mut self, name: &str, len: uint, f: F) -> Result<(), E> where
        F: FnOnce(&mut Self) -> Result<(), E>;
    fn emit_struct_field<F>(&mut self, f_name: &str, f_idx: uint, f: F) -> Result<(), E> where
        F: FnOnce(&mut Self) -> Result<(), E>;

    fn emit_seq<F>(&mut self, len: uint, f: F) -> Result<(), E> where
        F: FnOnce(&mut Self) -> Result<(), E>;
    fn emit_seq_elt<F>(&mut self, idx: uint, f: F) -> Result<(), E> where
        F: FnOnce(&mut Self) -> Result<(), E>;
}

pub trait Decoder<E> {
    // Primitive types:
    fn read_f32(&mut self) -> Result<f32, E>;
    fn read_f64(&mut self) -> Result<f64, E>;
    fn read_str(&mut self) -> Result<String, E>;

    fn read_struct<T, F>(&mut self, s_name: &str, len: uint, f: F) -> Result<T, E> where
        F: FnOnce(&mut Self) -> Result<T, E>;
    fn read_struct_field<T, F>(&mut self,
                               f_name: &str,
                               f_idx: uint,
                               f: F)
                               -> Result<T, E> where
        F: FnOnce(&mut Self) -> Result<T, E>;

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, E> where
        F: FnOnce(&mut Self, uint) -> Result<T, E>;
    fn read_seq_elt<T, F>(&mut self, idx: uint, f: F) -> Result<T, E> where
        F: FnOnce(&mut Self) -> Result<T, E>;

    // Failure
    fn error(&mut self, err: &str) -> E;
}

pub trait Encodable<S:Encoder<E>, E> for Sized? {
    fn encode(&self, s: &mut S) -> Result<(), E>;
}

pub trait Decodable<D:Decoder<E>, E> {
    fn decode(d: &mut D) -> Result<Self, E>;
}

impl<E, S:Encoder<E>> Encodable<S, E> for str {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_str(self)
    }
}

impl<E, S:Encoder<E>> Encodable<S, E> for String {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_str(self[])
    }
}

impl<E, D:Decoder<E>> Decodable<D, E> for String {
    fn decode(d: &mut D) -> Result<String, E> {
        d.read_str()
    }
}

impl<E, S:Encoder<E>> Encodable<S, E> for f32 {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_f32(*self)
    }
}

impl<E, D:Decoder<E>> Decodable<D, E> for f32 {
    fn decode(d: &mut D) -> Result<f32, E> {
        d.read_f32()
    }
}

impl<'a, E, S: Encoder<E>, Sized? T: Encodable<S, E>> Encodable<S, E> for &'a T {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        (**self).encode(s)
    }
}

// ___________________________________________________________________________
// Helper routines

pub trait EncoderHelpers<E> {
    fn emit_from_vec<T, F>(&mut self, v: &[T], f: F) -> Result<(), E> where
        F: FnMut(&mut Self, &T) -> Result<(), E>;
}

impl<E, S:Encoder<E>> EncoderHelpers<E> for S {
    fn emit_from_vec<T, F>(&mut self, v: &[T], mut f: F) -> Result<(), E> where
        F: FnMut(&mut S, &T) -> Result<(), E>,
    {
        self.emit_seq(v.len(), |this| {
            for (i, e) in v.iter().enumerate() {
                try!(this.emit_seq_elt(i, |this| {
                    f(this, e)
                }));
            }
            Ok(())
        })
    }
}

pub trait DecoderHelpers<E> {
    fn read_to_vec<T, F>(&mut self, f: F) -> Result<Vec<T>, E> where
        F: FnMut(&mut Self) -> Result<T, E>;
}

impl<E, D:Decoder<E>> DecoderHelpers<E> for D {
    fn read_to_vec<T, F>(&mut self, mut f: F) -> Result<Vec<T>, E> where F:
        FnMut(&mut D) -> Result<T, E>,
    {
        self.read_seq(|this, len| {
            let mut v = Vec::with_capacity(len);
            for i in range(0, len) {
                v.push(try!(this.read_seq_elt(i, |this| f(this))));
            }
            Ok(v)
        })
    }
}

pub mod json {
    extern crate unicode;

    use std::collections::BTreeMap;
    use std::{io, f64, string};
    use std::num::Float;
    use std::mem::transmute;

    use super::Encodable;

    type Array = Vec<Json>;
    type Object = BTreeMap<string::String, Json>;
    type EncodeResult = io::IoResult<()>;

    /// The errors that can arise while parsing a JSON stream.
    #[deriving(Clone, Copy, PartialEq, Show)]
    pub enum ErrorCode {
        InvalidSyntax,
        InvalidNumber,
        EOFWhileParsingObject,
        EOFWhileParsingArray,
        EOFWhileParsingValue,
        EOFWhileParsingString,
        KeyMustBeAString,
        ExpectedColon,
        TrailingCharacters,
        TrailingComma,
        InvalidEscape,
        InvalidUnicodeCodePoint,
        LoneLeadingSurrogateInHexEscape,
        UnexpectedEndOfHexEscape,
        UnrecognizedHex,
        NotFourDigit,
        NotUtf8,
    }

    #[deriving(Clone, Copy, PartialEq, Show)]
    pub enum ParserError {
        /// msg, line, col
        SyntaxError(ErrorCode, uint, uint),
        IoError(io::IoErrorKind, &'static str),
    }

    pub type BuilderError = ParserError;

    /// Represents a json value
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum Json {
        I64(i64),
        U64(u64),
        F64(f64),
        String(string::String),
        Boolean(bool),
        Array(self::Array),
        Object(self::Object),
        Null,
    }

    pub fn encode<'a, T: Encodable<Encoder<'a>, io::IoError>>(object: &T) -> string::String {
        let buff = Encoder::buffer_encode(object);
        string::String::from_utf8(buff).unwrap()
    }

    pub struct Encoder<'a> {
        writer: &'a mut (io::Writer+'a),
    }

    impl<'a> Encoder<'a> {
        /// Creates a new JSON encoder whose output will be written to the writer
        /// specified.
        pub fn new(writer: &'a mut io::Writer) -> Encoder<'a> {
            Encoder { writer: writer }
        }

        /// Encode the specified struct into a json [u8]
        pub fn buffer_encode<T: Encodable<Encoder<'a>, io::IoError>>(object: &T) -> Vec<u8>  {
            //Serialize the object in a string using a writer
            let mut m = Vec::new();
            unsafe {
                let mut encoder = Encoder::new(&mut m as &mut io::Writer);
                // Vec<u8> never Errs
                let _ = object.encode(transmute(&mut encoder));
            }
            m
        }
    }

    impl<'a> super::Encoder<io::IoError> for Encoder<'a> {
        fn emit_f32(&mut self, v: f32) -> EncodeResult {
            self.emit_f64(v as f64)
        }
        fn emit_f64(&mut self, v: f64) -> EncodeResult {
            write!(self.writer, "{}", fmt_number_or_null(v))
        }
        fn emit_str(&mut self, v: &str) -> EncodeResult {
            escape_str(self.writer, v)
        }

        fn emit_struct<F>(&mut self, _: &str, _: uint, f: F) -> EncodeResult where
            F: FnOnce(&mut Encoder<'a>) -> EncodeResult,
        {
            try!(write!(self.writer, "{{"));
            try!(f(self));
            write!(self.writer, "}}")
        }

        fn emit_struct_field<F>(&mut self, name: &str, idx: uint, f: F) -> EncodeResult where
            F: FnOnce(&mut Encoder<'a>) -> EncodeResult,
        {
            if idx != 0 { try!(write!(self.writer, ",")); }
            try!(escape_str(self.writer, name));
            try!(write!(self.writer, ":"));
            f(self)
        }

        fn emit_seq<F>(&mut self, _len: uint, f: F) -> EncodeResult where
            F: FnOnce(&mut Encoder<'a>) -> EncodeResult,
        {
            try!(write!(self.writer, "["));
            try!(f(self));
            write!(self.writer, "]")
        }

        fn emit_seq_elt<F>(&mut self, idx: uint, f: F) -> EncodeResult where
            F: FnOnce(&mut Encoder<'a>) -> EncodeResult,
        {
            if idx != 0 {
                try!(write!(self.writer, ","));
            }
            f(self)
        }
    }

    fn escape_bytes(wr: &mut io::Writer, bytes: &[u8]) -> Result<(), io::IoError> {
        try!(wr.write_str("\""));

        let mut start = 0;

        for (i, byte) in bytes.iter().enumerate() {
            let escaped = match *byte {
                b'"' => "\\\"",
                b'\\' => "\\\\",
                b'\x00' => "\\u0000",
                b'\x01' => "\\u0001",
                b'\x02' => "\\u0002",
                b'\x03' => "\\u0003",
                b'\x04' => "\\u0004",
                b'\x05' => "\\u0005",
                b'\x06' => "\\u0006",
                b'\x07' => "\\u0007",
                b'\x08' => "\\b",
                b'\t' => "\\t",
                b'\n' => "\\n",
                b'\x0b' => "\\u000b",
                b'\x0c' => "\\f",
                b'\r' => "\\r",
                b'\x0e' => "\\u000e",
                b'\x0f' => "\\u000f",
                b'\x10' => "\\u0010",
                b'\x11' => "\\u0011",
                b'\x12' => "\\u0012",
                b'\x13' => "\\u0013",
                b'\x14' => "\\u0014",
                b'\x15' => "\\u0015",
                b'\x16' => "\\u0016",
                b'\x17' => "\\u0017",
                b'\x18' => "\\u0018",
                b'\x19' => "\\u0019",
                b'\x1a' => "\\u001a",
                b'\x1b' => "\\u001b",
                b'\x1c' => "\\u001c",
                b'\x1d' => "\\u001d",
                b'\x1e' => "\\u001e",
                b'\x1f' => "\\u001f",
                b'\x7f' => "\\u007f",
                _ => { continue; }
            };

            if start < i {
                try!(wr.write(bytes[start..i]));
            }

            try!(wr.write_str(escaped));

            start = i + 1;
        }

        if start != bytes.len() {
            try!(wr.write(bytes[start..]));
        }

        wr.write_str("\"")
    }

    fn escape_str(writer: &mut io::Writer, v: &str) -> Result<(), io::IoError> {
        escape_bytes(writer, v.as_bytes())
    }

    fn fmt_number_or_null(v: f64) -> string::String {
        use std::num::FpCategory::{Nan, Infinite};

        match v.classify() {
            Nan | Infinite => string::String::from_str("null"),
            _ if v.fract() != 0f64 => f64::to_str_digits(v, 6u),
            _ => f64::to_str_digits(v, 6u) + ".0",
        }
    }
}
