use core::fmt;
use std::{
    error,
    fmt::{Debug, Display},
    str::FromStr,
};


#[derive(Debug, PartialEq)]
pub enum Error {
    MissingMatch,
    MissingClosingBrace,
    MissingOpeningBrace,
    UnexpectedValue(u8, Option<u8>),
    InvalidUtf8(Vec<u8>),
    PartialUtf8(usize, Vec<u8>),
    Parse(String, &'static str),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Error::*;
        use std::str::from_utf8;

        match *self {
            InvalidUtf8(ref raw) => write!(f, "input was not valid utf8: {:?}", raw),
            Parse(ref s, arg) => write!(f, "could not parse {} as target type of {}", s, arg),
            UnexpectedValue(exp, act) => write!(
                f,
                "found value {:?} not matching the pattern value {}",
                act.map(|b| b as char),
                exp as char
            ),
            PartialUtf8(n, ref raw) => write!(
                f,
                "input was only partially valid utf8: \"{}\" followed by {:?}",
                from_utf8(&raw[..n]).unwrap(),
                &raw[n..]
            ),
            MissingMatch => write!(f, "Bad read! format string: did not contain {{}}"),
            MissingClosingBrace => write!(
                f,
                "found single open curly brace at the end of the format string"
            ),
            MissingOpeningBrace => write!(f, "couldn't find opening brace in input"),
        }
    }
}

fn parse_capture<'a, T: FromStr>(
    argument_name: &'static str,
    input: &'a mut dyn Iterator<Item = u8>,
) -> Result<T, Error> {
    match input.next() {
        Some(b'{') => {
            let mut open_counter = 1;
            let s = input
                .take_while(|ch| match ch {
                    b'{' => {
                        open_counter += 1;
                        true
                    }
                    b'}' => {
                        open_counter -= 1;
                        open_counter > 0
                    }
                    _ => true,
                })
                .collect();
            let data = String::from_utf8(s).map_err(|e| Error::InvalidUtf8(e.into_bytes()))?;
            let result: T = T::from_str(&data).map_err(|_| Error::Parse(data, argument_name))?;
            return Ok(result);
        }
        _ => return Err(Error::MissingOpeningBrace),
    }
}

pub fn match_char(input: Option<u8>, expected: u8) -> Result<(), Error> {
    match input {
        Some(input_char) => {
            if expected != input_char {
                return Err(Error::UnexpectedValue(expected, input));
            }
            Ok(())
        }
        None => return Err(Error::UnexpectedValue(expected, input)),
    }
}

pub fn scanf<T: FromStr>(
    input: &mut dyn Iterator<Item = u8>,
    pattern: &mut dyn Iterator<Item = u8>,
) -> Result<T, Error> {
    match pattern.next() {
        Some(b'{') => {
            match pattern.next() {
                Some(b'{') => {
                    // match char with input
                    match_char(input.next(), b'{')?;
                    return scanf(input, pattern);
                }
                Some(b'}') => {
                    return parse_capture(stringify!(T), input);
                }
                _ => return Err(Error::MissingClosingBrace),
            }
        }
        Some(pattern_char) => {
            // match char with input
            match_char(input.next(), pattern_char)?;
            return scanf(input, pattern);
        }
        None => return Err(Error::MissingMatch),
    }
}

pub fn printf<T: Display>(
    out: &mut dyn std::fmt::Write,
    pattern: &mut dyn Iterator<Item = u8>,
    input: T,
) -> Result<(), Error> {
    match pattern.next() {
        Some(b'{') => match pattern.next() {
            Some(b'{') => {
                out.write_char('{').unwrap();
                return printf(out, pattern, input);
            }
            Some(b'}') => {
                out.write_fmt(format_args!("{{{}}}", input)).unwrap();
                return Ok(());
            }
            _ => return Err(Error::MissingClosingBrace),
        },
        Some(c) => {
            out.write_char(c as char).unwrap();
            return printf(out, pattern, input);
        }
        None => return Err(Error::MissingOpeningBrace),
    }
}

#[macro_export]
macro_rules! pscan {
    ($input:expr => $pattern:expr,$($arg:expr), *) => {
        {
            format_args!($pattern,$($arg),*);

            // type checking
            let pattern: &'static str = $pattern;
            let pattern: &mut dyn Iterator<Item = u8> = &mut pattern.bytes();
            let input: &mut dyn Iterator<Item = u8> = $input;

            let mut res: Result<(), $crate::Error> = Ok(());

            // iterate over arguments
            $(
                match $crate::scanf(input, pattern){
                    Ok(val) => {
                        $arg = val;
                    },
                    Err(err) => {
                        if res.is_ok(){
                            res =  Err(err);
                        }
                    }
                }
            )*

            // write remaining characters
            while let Some(ch) = pattern.next(){
                match $crate::match_char(input.next(), ch){
                    Ok(()) => {}
                    Err(err) =>{res = Err(err)}
                }
            }

            res
        }
    };
}

pub struct FmtWriter<W: std::io::Write>(pub W);
impl<W: std::io::Write> std::fmt::Write for FmtWriter<W> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), std::fmt::Error> {
        self.0.write_fmt(args).map_err(|_| std::fmt::Error)
    }
}

#[macro_export]
macro_rules! pwrite {
    ($out:expr, $pattern:expr ,$($arg:expr), *) => {
        {
            let out: &mut dyn std::io::Write = $out;
            let mut writer = $crate::FmtWriter(out);
            $crate::pwrite!($pattern => &mut writer, $($arg),*)
        }
    };

    ($pattern:expr => $out:expr,$($arg:expr), *) => {
        {

            format_args!($pattern,$($arg),*);
            
            // type check
            let pattern: &'static str = $pattern;
            let out: &mut dyn std::fmt::Write = $out;


            let mut bytes = pattern.bytes();
            let mut res: Result<(), $crate::Error> = Ok(());

            // iterate over arguments
            $(
                match $crate::printf(out, &mut bytes, $arg){
                    Err(err)=> {
                        if res.is_ok(){
                            res = Err(err);
                        }
                    }
                    _=>{}
                }
            )*

            // write remaining characters
            while let Some(ch) = bytes.next(){
                out.write_char(ch as char).unwrap();
            }
            res
        }
    };
}