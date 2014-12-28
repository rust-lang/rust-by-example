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
    use std::{char, io, f64, str, string};
    use std::num::Float;
    use std::mem::swap;

    use self::unicode::str as unicode_str;
    use self::unicode::str::Utf16Item;

    type Array = Vec<Json>;
    type Object = BTreeMap<string::String, Json>;
    type EncodeResult = io::IoResult<()>;
    type DecodeResult<T> = Result<T, DecoderError>;

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

    #[deriving(PartialEq, Show)]
    pub enum DecoderError {
        ExpectedError(string::String, string::String),
        MissingFieldError(string::String),
        ApplicationError(string::String)
    }

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

    pub struct Decoder {
        stack: Vec<Json>,
    }

    impl Decoder {
        /// Creates a new decoder instance for decoding the specified JSON value.
        pub fn new(json: Json) -> Decoder {
            Decoder { stack: vec![json] }
        }

        fn pop(&mut self) -> Json {
            self.stack.pop().unwrap()
        }
    }

    macro_rules! expect {
        ($e:expr, Null) => ({
            match $e {
                Json::Null => Ok(()),
                other => Err(ExpectedError("Null".to_string(),
                                           format!("{}", other)))
            }
        });
        ($e:expr, $t:ident) => ({
            match $e {
                Json::$t(v) => Ok(v),
                other => {
                    Err(DecoderError::ExpectedError(stringify!($t).to_string(),
                                      format!("{}", other)))
                }
            }
        })
    }

    impl super::Decoder<DecoderError> for Decoder {
        fn read_f32(&mut self) -> DecodeResult<f32> { self.read_f64().map(|x| x as f32) }

        fn read_f64(&mut self) -> DecodeResult<f64> {
            match self.pop() {
                Json::I64(f) => Ok(f as f64),
                Json::U64(f) => Ok(f as f64),
                Json::F64(f) => Ok(f),
                Json::String(s) => {
                    // re: #12967.. a type w/ numeric keys (ie HashMap<uint, V> etc)
                    // is going to have a string here, as per JSON spec.
                    match s.parse() {
                        Some(f) => Ok(f),
                        None => Err(DecoderError::ExpectedError("Number".to_string(), s)),
                    }
                },
                Json::Null => Ok(f64::NAN),
                value => Err(DecoderError::ExpectedError("Number".to_string(), format!("{}", value)))
            }
        }

        fn read_str(&mut self) -> DecodeResult<string::String> {
            expect!(self.pop(), String)
        }

        fn read_struct<T, F>(&mut self, _name: &str, _len: uint, f: F) -> DecodeResult<T> where
            F: FnOnce(&mut Decoder) -> DecodeResult<T>,
        {
            let value = try!(f(self));
            self.pop();
            Ok(value)
        }

        fn read_struct_field<T, F>(&mut self,
                                   name: &str,
                                   _idx: uint,
                                   f: F)
                                   -> DecodeResult<T> where
            F: FnOnce(&mut Decoder) -> DecodeResult<T>,
        {
            let mut obj = try!(expect!(self.pop(), Object));

            let value = match obj.remove(&name.to_string()) {
                None => {
                    // Add a Null and try to parse it as an Option<_>
                    // to get None as a default value.
                    self.stack.push(Json::Null);
                    match f(self) {
                        Ok(x) => x,
                        Err(_) => return Err(DecoderError::MissingFieldError(name.to_string())),
                    }
                },
                Some(json) => {
                    self.stack.push(json);
                    try!(f(self))
                }
            };
            self.stack.push(Json::Object(obj));
            Ok(value)
        }

        fn read_seq<T, F>(&mut self, f: F) -> DecodeResult<T> where
            F: FnOnce(&mut Decoder, uint) -> DecodeResult<T>,
        {
            let array = try!(expect!(self.pop(), Array));
            let len = array.len();
            for v in array.into_iter().rev() {
                self.stack.push(v);
            }
            f(self, len)
        }

        fn read_seq_elt<T, F>(&mut self, _idx: uint, f: F) -> DecodeResult<T> where
            F: FnOnce(&mut Decoder) -> DecodeResult<T>,
        {
            f(self)
        }

        fn error(&mut self, err: &str) -> DecoderError {
            DecoderError::ApplicationError(err.to_string())
        }
    }

    /// A Stack represents the current position of the parser in the logical
    /// structure of the JSON stream.
    /// For example foo.bar[3].x
    pub struct Stack {
        stack: Vec<InternalStackElement>,
        str_buffer: Vec<u8>,
    }

    /// StackElements compose a Stack.
    /// For example, Key("foo"), Key("bar"), Index(3) and Key("x") are the
    /// StackElements compositing the stack that represents foo.bar[3].x
    #[deriving(PartialEq, Clone, Show)]
    pub enum StackElement<'l> {
        Index(u32),
        Key(&'l str),
    }

    // Internally, Key elements are stored as indices in a buffer to avoid
    // allocating a string for every member of an object.
    #[deriving(PartialEq, Clone, Show)]
    enum InternalStackElement {
        Index(u32),
        Key(u16, u16), // start, size
    }

    #[deriving(PartialEq)]
    enum ParserState {
        // Parse a value in an array, true means first element.
        Array(bool),
        // Parse ',' or ']' after an element in an array.
        ArrayComma,
        // Parse a key:value in an object, true means first element.
        Object(bool),
        // Parse ',' or ']' after an element in an object.
        ObjectComma,
        // Initial state.
        Start,
        // Expecting the stream to end.
        BeforeFinish,
        // Parsing can't continue.
        Finished,
    }

    #[deriving(PartialEq, Show)]
    pub enum JsonEvent {
        ObjectStart,
        ObjectEnd,
        ArrayStart,
        ArrayEnd,
        BooleanValue(bool),
        I64Value(i64),
        U64Value(u64),
        F64Value(f64),
        StringValue(string::String),
        NullValue,
        Error(ParserError),
    }

    impl Stack {
        pub fn new() -> Stack {
            Stack { stack: Vec::new(), str_buffer: Vec::new() }
        }

        /// Returns true if the stack is empty.
        pub fn is_empty(&self) -> bool { self.stack.is_empty() }

        /// Returns the top-most element (if any).
        pub fn top<'l>(&'l self) -> Option<StackElement<'l>> {
            return match self.stack.last() {
                None => None,
                Some(&InternalStackElement::Index(i)) => Some(StackElement::Index(i)),
                Some(&InternalStackElement::Key(start, size)) => {
                    Some(StackElement::Key(str::from_utf8(
                        self.str_buffer[start as uint .. (start+size) as uint]
                    ).unwrap()))
                }
            }
        }

        // Used by Parser to insert Key elements at the top of the stack.
        fn push_key(&mut self, key: string::String) {
            self.stack.push(InternalStackElement::Key(self.str_buffer.len() as u16, key.len() as u16));
            for c in key.as_bytes().iter() {
                self.str_buffer.push(*c);
            }
        }

        // Used by Parser to insert Index elements at the top of the stack.
        fn push_index(&mut self, index: u32) {
            self.stack.push(InternalStackElement::Index(index));
        }

        // Used by Parser to remove the top-most element of the stack.
        fn pop(&mut self) {
            assert!(!self.is_empty());
            match *self.stack.last().unwrap() {
                InternalStackElement::Key(_, sz) => {
                    let new_size = self.str_buffer.len() - sz as uint;
                    self.str_buffer.truncate(new_size);
                }
                InternalStackElement::Index(_) => {}
            }
            self.stack.pop();
        }

        // Used by Parser to test whether the top-most element is an index.
        fn last_is_index(&self) -> bool {
            if self.is_empty() { return false; }
            return match *self.stack.last().unwrap() {
                InternalStackElement::Index(_) => true,
                _ => false,
            }
        }

        // Used by Parser to increment the index of the top-most element.
        fn bump_index(&mut self) {
            let len = self.stack.len();
            let idx = match *self.stack.last().unwrap() {
                InternalStackElement::Index(i) => { i + 1 }
                _ => { panic!(); }
            };
            self.stack[len - 1] = InternalStackElement::Index(idx);
        }
    }

    /// an iterator of char.
    pub struct Parser<T> {
        rdr: T,
        ch: Option<char>,
        line: uint,
        col: uint,
        // We maintain a stack representing where we are in the logical structure
        // of the JSON stream.
        stack: Stack,
        // A state machine is kept to make it possible to interrupt and resume parsing.
        state: ParserState,
    }

    impl<T: Iterator<char>> Iterator<JsonEvent> for Parser<T> {
        fn next(&mut self) -> Option<JsonEvent> {
            if self.state == ParserState::Finished {
                return None;
            }

            if self.state == ParserState::BeforeFinish {
                self.parse_whitespace();
                // Make sure there is no trailing characters.
                if self.eof() {
                    self.state = ParserState::Finished;
                    return None;
                } else {
                    return Some(self.error_event(ErrorCode::TrailingCharacters));
                }
            }

            return Some(self.parse());
        }
    }

    impl<T: Iterator<char>> Parser<T> {
        /// Creates the JSON parser.
        pub fn new(rdr: T) -> Parser<T> {
            let mut p = Parser {
                rdr: rdr,
                ch: Some('\x00'),
                line: 1,
                col: 0,
                stack: Stack::new(),
                state: ParserState::Start,
            };
            p.bump();
            return p;
        }

        /// Provides access to the current position in the logical structure of the
        /// JSON stream.
        pub fn stack<'l>(&'l self) -> &'l Stack {
            return &self.stack;
        }

        fn eof(&self) -> bool { self.ch.is_none() }
        fn ch_or_null(&self) -> char { self.ch.unwrap_or('\x00') }
        fn bump(&mut self) {
            self.ch = self.rdr.next();

            if self.ch_is('\n') {
                self.line += 1u;
                self.col = 1u;
            } else {
                self.col += 1u;
            }
        }

        fn next_char(&mut self) -> Option<char> {
            self.bump();
            self.ch
        }
        fn ch_is(&self, c: char) -> bool {
            self.ch == Some(c)
        }

        fn error<T>(&self, reason: ErrorCode) -> Result<T, ParserError> {
            Err(ParserError::SyntaxError(reason, self.line, self.col))
        }

        fn parse_whitespace(&mut self) {
            while self.ch_is(' ') ||
                  self.ch_is('\n') ||
                  self.ch_is('\t') ||
                  self.ch_is('\r') { self.bump(); }
        }

        fn parse_number(&mut self) -> JsonEvent {
            let mut neg = false;

            if self.ch_is('-') {
                self.bump();
                neg = true;
            }

            let res = match self.parse_u64() {
                Ok(res) => res,
                Err(e) => { return JsonEvent::Error(e); }
            };

            if self.ch_is('.') || self.ch_is('e') || self.ch_is('E') {
                let mut res = res as f64;

                if self.ch_is('.') {
                    res = match self.parse_decimal(res) {
                        Ok(res) => res,
                        Err(e) => { return JsonEvent::Error(e); }
                    };
                }

                if self.ch_is('e') || self.ch_is('E') {
                    res = match self.parse_exponent(res) {
                        Ok(res) => res,
                        Err(e) => { return JsonEvent::Error(e); }
                    };
                }

                if neg {
                    res *= -1.0;
                }

                JsonEvent::F64Value(res)
            } else {
                if neg {
                    let res = -(res as i64);

                    // Make sure we didn't underflow.
                    if res > 0 {
                        JsonEvent::Error(ParserError::SyntaxError(
                                ErrorCode::InvalidNumber, self.line, self.col))
                    } else {
                        JsonEvent::I64Value(res)
                    }
                } else {
                    JsonEvent::U64Value(res)
                }
            }
        }

        fn parse_u64(&mut self) -> Result<u64, ParserError> {
            let mut accum = 0;
            let last_accum = 0; // necessary to detect overflow.

            match self.ch_or_null() {
                '0' => {
                    self.bump();

                    // A leading '0' must be the only digit before the decimal point.
                    match self.ch_or_null() {
                        '0' ... '9' => return self.error(ErrorCode::InvalidNumber),
                        _ => ()
                    }
                },
                '1' ... '9' => {
                    while !self.eof() {
                        match self.ch_or_null() {
                            c @ '0' ... '9' => {
                                accum *= 10;
                                accum += (c as u64) - ('0' as u64);

                                // Detect overflow by comparing to the last value.
                                if accum <= last_accum { return self.error(ErrorCode::InvalidNumber); }

                                self.bump();
                            }
                            _ => break,
                        }
                    }
                }
                _ => return self.error(ErrorCode::InvalidNumber),
            }

            Ok(accum)
        }

        fn parse_decimal(&mut self, mut res: f64) -> Result<f64, ParserError> {
            self.bump();

            // Make sure a digit follows the decimal place.
            match self.ch_or_null() {
                '0' ... '9' => (),
                 _ => return self.error(ErrorCode::InvalidNumber)
            }

            let mut dec = 1.0;
            while !self.eof() {
                match self.ch_or_null() {
                    c @ '0' ... '9' => {
                        dec /= 10.0;
                        res += (((c as int) - ('0' as int)) as f64) * dec;
                        self.bump();
                    }
                    _ => break,
                }
            }

            Ok(res)
        }

        fn parse_exponent(&mut self, mut res: f64) -> Result<f64, ParserError> {
            self.bump();

            let mut exp = 0u;
            let mut neg_exp = false;

            if self.ch_is('+') {
                self.bump();
            } else if self.ch_is('-') {
                self.bump();
                neg_exp = true;
            }

            // Make sure a digit follows the exponent place.
            match self.ch_or_null() {
                '0' ... '9' => (),
                _ => return self.error(ErrorCode::InvalidNumber)
            }
            while !self.eof() {
                match self.ch_or_null() {
                    c @ '0' ... '9' => {
                        exp *= 10;
                        exp += (c as uint) - ('0' as uint);

                        self.bump();
                    }
                    _ => break
                }
            }

            let exp = 10_f64.powi(exp as i32);
            if neg_exp {
                res /= exp;
            } else {
                res *= exp;
            }

            Ok(res)
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ParserError> {
            let mut i = 0u;
            let mut n = 0u16;
            while i < 4 && !self.eof() {
                self.bump();
                n = match self.ch_or_null() {
                    c @ '0' ... '9' => n * 16 + ((c as u16) - ('0' as u16)),
                    'a' | 'A' => n * 16 + 10,
                    'b' | 'B' => n * 16 + 11,
                    'c' | 'C' => n * 16 + 12,
                    'd' | 'D' => n * 16 + 13,
                    'e' | 'E' => n * 16 + 14,
                    'f' | 'F' => n * 16 + 15,
                    _ => return self.error(ErrorCode::InvalidEscape)
                };

                i += 1u;
            }

            // Error out if we didn't parse 4 digits.
            if i != 4 {
                return self.error(ErrorCode::InvalidEscape);
            }

            Ok(n)
        }

        fn parse_str(&mut self) -> Result<string::String, ParserError> {
            let mut escape = false;
            let mut res = string::String::new();

            loop {
                self.bump();
                if self.eof() {
                    return self.error(ErrorCode::EOFWhileParsingString);
                }

                if escape {
                    match self.ch_or_null() {
                        '"' => res.push('"'),
                        '\\' => res.push('\\'),
                        '/' => res.push('/'),
                        'b' => res.push('\x08'),
                        'f' => res.push('\x0c'),
                        'n' => res.push('\n'),
                        'r' => res.push('\r'),
                        't' => res.push('\t'),
                        'u' => match try!(self.decode_hex_escape()) {
                            0xDC00 ... 0xDFFF => {
                                return self.error(ErrorCode::LoneLeadingSurrogateInHexEscape)
                            }

                            // Non-BMP characters are encoded as a sequence of
                            // two hex escapes, representing UTF-16 surrogates.
                            n1 @ 0xD800 ... 0xDBFF => {
                                match (self.next_char(), self.next_char()) {
                                    (Some('\\'), Some('u')) => (),
                                    _ => return self.error(ErrorCode::UnexpectedEndOfHexEscape),
                                }

                                let buf = [n1, try!(self.decode_hex_escape())];
                                match unicode_str::utf16_items(&buf).next() {
                                    Some(Utf16Item::ScalarValue(c)) => res.push(c),
                                    _ => return self.error(ErrorCode::LoneLeadingSurrogateInHexEscape),
                                }
                            }

                            n => match char::from_u32(n as u32) {
                                Some(c) => res.push(c),
                                None => return self.error(ErrorCode::InvalidUnicodeCodePoint),
                            },
                        },
                        _ => return self.error(ErrorCode::InvalidEscape),
                    }
                    escape = false;
                } else if self.ch_is('\\') {
                    escape = true;
                } else {
                    match self.ch {
                        Some('"') => {
                            self.bump();
                            return Ok(res);
                        },
                        Some(c) => res.push(c),
                        None => unreachable!()
                    }
                }
            }
        }

        // Invoked at each iteration, consumes the stream until it has enough
        // information to return a JsonEvent.
        // Manages an internal state so that parsing can be interrupted and resumed.
        // Also keeps track of the position in the logical structure of the json
        // stream int the form of a stack that can be queried by the user using the
        // stack() method.
        fn parse(&mut self) -> JsonEvent {
            loop {
                // The only paths where the loop can spin a new iteration
                // are in the cases ParseArrayComma and ParseObjectComma if ','
                // is parsed. In these cases the state is set to (respectively)
                // ParseArray(false) and ParseObject(false), which always return,
                // so there is no risk of getting stuck in an infinite loop.
                // All other paths return before the end of the loop's iteration.
                self.parse_whitespace();

                match self.state {
                    ParserState::Start => {
                        return self.parse_start();
                    }
                    ParserState::Array(first) => {
                        return self.parse_array(first);
                    }
                    ParserState::ArrayComma => {
                        match self.parse_array_comma_or_end() {
                            Some(evt) => { return evt; }
                            None => {}
                        }
                    }
                    ParserState::Object(first) => {
                        return self.parse_object(first);
                    }
                    ParserState::ObjectComma => {
                        self.stack.pop();
                        if self.ch_is(',') {
                            self.state = ParserState::Object(false);
                            self.bump();
                        } else {
                            return self.parse_object_end();
                        }
                    }
                    _ => {
                        return self.error_event(ErrorCode::InvalidSyntax);
                    }
                }
            }
        }

        fn parse_start(&mut self) -> JsonEvent {
            let val = self.parse_value();
            self.state = match val {
                JsonEvent::Error(_) => ParserState::Finished,
                JsonEvent::ArrayStart => ParserState::Array(true),
                JsonEvent::ObjectStart => ParserState::Object(true),
                _ => ParserState::BeforeFinish,
            };
            val
        }

        fn parse_array(&mut self, first: bool) -> JsonEvent {
            if self.ch_is(']') {
                if !first {
                    self.error_event(ErrorCode::InvalidSyntax)
                } else {
                    self.state = if self.stack.is_empty() {
                        ParserState::BeforeFinish
                    } else if self.stack.last_is_index() {
                        ParserState::ArrayComma
                    } else {
                        ParserState::ObjectComma
                    };
                    self.bump();
                    JsonEvent::ArrayEnd
                }
            } else {
                if first {
                    self.stack.push_index(0);
                }
                let val = self.parse_value();
                self.state = match val {
                    JsonEvent::Error(_) => ParserState::Finished,
                    JsonEvent::ArrayStart => ParserState::Array(true),
                    JsonEvent::ObjectStart => ParserState::Object(true),
                    _ => ParserState::ArrayComma,
                };
                val
            }
        }

        fn parse_array_comma_or_end(&mut self) -> Option<JsonEvent> {
            if self.ch_is(',') {
                self.stack.bump_index();
                self.state = ParserState::Array(false);
                self.bump();
                None
            } else if self.ch_is(']') {
                self.stack.pop();
                self.state = if self.stack.is_empty() {
                    ParserState::BeforeFinish
                } else if self.stack.last_is_index() {
                    ParserState::ArrayComma
                } else {
                    ParserState::ObjectComma
                };
                self.bump();
                Some(JsonEvent::ArrayEnd)
            } else if self.eof() {
                Some(self.error_event(ErrorCode::EOFWhileParsingArray))
            } else {
                Some(self.error_event(ErrorCode::InvalidSyntax))
            }
        }

        fn parse_object(&mut self, first: bool) -> JsonEvent {
            if self.ch_is('}') {
                if !first {
                    if self.stack.is_empty() {
                        return self.error_event(ErrorCode::TrailingComma);
                    } else {
                        self.stack.pop();
                    }
                }
                self.state = if self.stack.is_empty() {
                    ParserState::BeforeFinish
                } else if self.stack.last_is_index() {
                    ParserState::ArrayComma
                } else {
                    ParserState::ObjectComma
                };
                self.bump();
                return JsonEvent::ObjectEnd;
            }
            if self.eof() {
                return self.error_event(ErrorCode::EOFWhileParsingObject);
            }
            if !self.ch_is('"') {
                return self.error_event(ErrorCode::KeyMustBeAString);
            }
            let s = match self.parse_str() {
                Ok(s) => s,
                Err(e) => {
                    self.state = ParserState::Finished;
                    return JsonEvent::Error(e);
                }
            };
            self.parse_whitespace();
            if self.eof() {
                return self.error_event(ErrorCode::EOFWhileParsingObject);
            } else if self.ch_or_null() != ':' {
                return self.error_event(ErrorCode::ExpectedColon);
            }
            self.stack.push_key(s);
            self.bump();
            self.parse_whitespace();

            let val = self.parse_value();

            self.state = match val {
                JsonEvent::Error(_) => ParserState::Finished,
                JsonEvent::ArrayStart => ParserState::Array(true),
                JsonEvent::ObjectStart => ParserState::Object(true),
                _ => ParserState::ObjectComma,
            };
            return val;
        }

        fn parse_object_end(&mut self) -> JsonEvent {
            if self.ch_is('}') {
                self.state = if self.stack.is_empty() {
                    ParserState::BeforeFinish
                } else if self.stack.last_is_index() {
                    ParserState::ArrayComma
                } else {
                    ParserState::ObjectComma
                };
                self.bump();
                JsonEvent::ObjectEnd
            } else if self.eof() {
                self.error_event(ErrorCode::EOFWhileParsingObject)
            } else {
                self.error_event(ErrorCode::InvalidSyntax)
            }
        }

        fn parse_value(&mut self) -> JsonEvent {
            if self.eof() { return self.error_event(ErrorCode::EOFWhileParsingValue); }
            match self.ch_or_null() {
                'n' => { self.parse_ident("ull", JsonEvent::NullValue) }
                't' => { self.parse_ident("rue", JsonEvent::BooleanValue(true)) }
                'f' => { self.parse_ident("alse", JsonEvent::BooleanValue(false)) }
                '0' ... '9' | '-' => self.parse_number(),
                '"' => match self.parse_str() {
                    Ok(s) => JsonEvent::StringValue(s),
                    Err(e) => JsonEvent::Error(e),
                },
                '[' => {
                    self.bump();
                    JsonEvent::ArrayStart
                }
                '{' => {
                    self.bump();
                    JsonEvent::ObjectStart
                }
                _ => { self.error_event(ErrorCode::InvalidSyntax) }
            }
        }

        fn parse_ident(&mut self, ident: &str, value: JsonEvent) -> JsonEvent {
            if ident.chars().all(|c| Some(c) == self.next_char()) {
                self.bump();
                value
            } else {
                JsonEvent::Error(ParserError::SyntaxError(ErrorCode::InvalidSyntax, self.line, self.col))
            }
        }

        fn error_event(&mut self, reason: ErrorCode) -> JsonEvent {
            self.state = ParserState::Finished;
            JsonEvent::Error(ParserError::SyntaxError(reason, self.line, self.col))
        }
    }

    pub struct Builder<T> {
        parser: Parser<T>,
        token: Option<JsonEvent>,
    }

    impl<T: Iterator<char>> Builder<T> {
        /// Create a JSON Builder.
        pub fn new(src: T) -> Builder<T> {
            Builder { parser: Parser::new(src), token: None, }
        }

        // Decode a Json value from a Parser.
        pub fn build(&mut self) -> Result<Json, BuilderError> {
            self.bump();
            let result = self.build_value();
            self.bump();
            match self.token {
                None => {}
                Some(JsonEvent::Error(e)) => { return Err(e); }
                ref tok => { panic!("unexpected token {}", tok.clone()); }
            }
            result
        }

        fn bump(&mut self) {
            self.token = self.parser.next();
        }

        fn build_value(&mut self) -> Result<Json, BuilderError> {
            return match self.token {
                Some(JsonEvent::NullValue) => Ok(Json::Null),
                Some(JsonEvent::I64Value(n)) => Ok(Json::I64(n)),
                Some(JsonEvent::U64Value(n)) => Ok(Json::U64(n)),
                Some(JsonEvent::F64Value(n)) => Ok(Json::F64(n)),
                Some(JsonEvent::BooleanValue(b)) => Ok(Json::Boolean(b)),
                Some(JsonEvent::StringValue(ref mut s)) => {
                    let mut temp = string::String::new();
                    swap(s, &mut temp);
                    Ok(Json::String(temp))
                }
                Some(JsonEvent::Error(e)) => Err(e),
                Some(JsonEvent::ArrayStart) => self.build_array(),
                Some(JsonEvent::ObjectStart) => self.build_object(),
                Some(JsonEvent::ObjectEnd) => self.parser.error(ErrorCode::InvalidSyntax),
                Some(JsonEvent::ArrayEnd) => self.parser.error(ErrorCode::InvalidSyntax),
                None => self.parser.error(ErrorCode::EOFWhileParsingValue),
            }
        }

        fn build_array(&mut self) -> Result<Json, BuilderError> {
            self.bump();
            let mut values = Vec::new();

            loop {
                if self.token == Some(JsonEvent::ArrayEnd) {
                    return Ok(Json::Array(values.into_iter().collect()));
                }
                match self.build_value() {
                    Ok(v) => values.push(v),
                    Err(e) => { return Err(e) }
                }
                self.bump();
            }
        }

        fn build_object(&mut self) -> Result<Json, BuilderError> {
            self.bump();

            let mut values = BTreeMap::new();

            loop {
                match self.token {
                    Some(JsonEvent::ObjectEnd) => { return Ok(Json::Object(values)); }
                    Some(JsonEvent::Error(e)) => { return Err(e); }
                    None => { break; }
                    _ => {}
                }
                let key = match self.parser.stack().top() {
                    Some(StackElement::Key(k)) => { k.to_string() }
                    _ => { panic!("invalid state"); }
                };
                match self.build_value() {
                    Ok(value) => { values.insert(key, value); }
                    Err(e) => { return Err(e); }
                }
                self.bump();
            }
            return self.parser.error(ErrorCode::EOFWhileParsingObject);
        }
    }

    /// Decodes a json value from a string
    pub fn from_str(s: &str) -> Result<Json, BuilderError> {
        let mut builder = Builder::new(s.chars());
        builder.build()
    }
}
