//! Improved version of Rust's `include_bytes` macro that allows to reinterpret input as differently array.
//!
//! Due to inability to capture current file path in the stable Rust, this macro only accepts paths relative to crate's root.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

extern crate proc_macro;

use proc_macro::TokenStream;

use core::fmt;

#[cold]
#[inline(never)]
fn compile_error<T: core::fmt::Display>(text: T) -> TokenStream {
    format!("core::compile_error!(\"{}\")", text).parse().unwrap()
}

enum Type {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl Type {
    ///returns number of bytes written.
    fn write_bytes<O: fmt::Write>(&self, out: &mut O, bytes: &[u8]) -> usize {
        match self {
            Type::U8 => {
                for byte in bytes {
                    core::fmt::write(out, format_args!("0x{:x}u8, ", byte)).expect("To write string");
                }
                bytes.len()
            },
            Type::U16 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(2) {
                    written += chunk.len();
                    let byte = u16::from_ne_bytes([chunk[0], chunk[1]]);
                    core::fmt::write(out, format_args!("0x{:x}u16, ", byte)).expect("To write string");
                }
                written
            },
            Type::U32 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(4) {
                    written += chunk.len();
                    let byte = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    core::fmt::write(out, format_args!("0x{:x}u32, ", byte)).expect("To write string");
                }
                written
            }
            Type::U64 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(8) {
                    written += chunk.len();
                    let byte = u64::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
                    core::fmt::write(out, format_args!("0x{:x}u64, ", byte)).expect("To write string");
                }
                written
            },
            Type::U128 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(16) {
                    written += chunk.len();
                    let byte = u128::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]]);
                    core::fmt::write(out, format_args!("0x{:x}u128, ", byte)).expect("To write string");
                }
                written
            },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::U8 => fmt.write_str("u8"),
            Type::U16 => fmt.write_str("u16"),
            Type::U32 => fmt.write_str("u32"),
            Type::U64 => fmt.write_str("u64"),
            Type::U128 => fmt.write_str("u128"),
        }
    }
}

struct Input<'a> {
    file: &'a str,
    typ: Type,
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> Result<Self, TokenStream> {
        let (file, input) = if let Some(input) = input.strip_prefix('"') {
            if let Some(end_file_idx) = input.find('"') {
                (&input[..end_file_idx], &input[end_file_idx+1..])
            } else {
                return Err(compile_error("Missing '\"' at the end of file path"));
            }
        } else {
            let mut split = input.split_whitespace();
            let file = split.next().unwrap();
            (file, &input[file.len()..])
        };

        let mut split = input.trim().split_whitespace();

        let typ = match split.next() {
            Some("as") => match split.next() {
                None => return Err(compile_error("'as' is missing type")),
                Some("u8") => Type::U8,
                Some("u16") => Type::U16,
                Some("u32") => Type::U32,
                Some("u64") => Type::U64,
                Some("u128") => Type::U128,
                Some(other) => return Err(compile_error(format_args!("'as' specifies unsupported type '{}'", other))),
            },
            Some(other) => return Err(compile_error(format_args!("Unsupported syntax after file name '{}'", other))),
            None => Type::U8,
        };

        Ok(Self {
            file,
            typ,
        })
    }
}

#[proc_macro]
///Includes a file as a reference to a byte array.
///
///This macro will yield an expression of type [u8; N] by default with content of file.
///
///To reinterpret it as different type add `as <type>` where type can be: `u8`, `u16`, `u32`, `u64` or `u128`.
///
///# NOTE:
///
///Due to `Span::source_file` being unstable, the file is searched relative to crate root.
///
///# Usage:
///
///```
///use include_bytes_plus::include_bytes;
///
///let bytes = include_bytes!("tests/include.in");
///let bytes_u16 = include_bytes!("tests/include.in" as u16);
///let bytes_u16_2 = include_bytes!("tests/include with whitespaces.in" as u16);
///
///assert_eq!(bytes.len(), bytes_u16.len() * 2);
///assert_eq!(bytes.len(), bytes_u16_2.len() * 2);
///```
pub fn include_bytes(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim();

    let args = match Input::parse(input) {
        Ok(args) => args,
        Err(error) => return error,
    };

    if args.file.is_empty() {
        return compile_error("Empty file name");
    }

    let mut file = match std::fs::File::open(args.file) {
        Ok(file) => file,
        Err(error) => return compile_error(format_args!("{}: Cannot open file: {}", args.file, error)),
    };

    let mut cursor = 0;
    let mut file_len = 0;
    let mut buf = [0u8; 4096];
    let mut result = "[".to_owned();

    loop {
        match std::io::Read::read(&mut file, &mut buf[cursor..]) {
            Ok(0) => {
                result.push(']');
                if cursor != 0 {
                    return compile_error(format_args!("File input with size {}b cannot be reinterpret as {}", file_len, args.typ));
                }
                break;
            },
            Ok(size) => {
                file_len += size;
                let buf_len = cursor + size;
                let written = args.typ.write_bytes(&mut result, &buf[..buf_len]);
                unsafe {
                    core::ptr::copy(buf.as_ptr().add(written), buf.as_mut_ptr(), buf_len - written);
                }
                cursor = buf_len - written;
            },
            Err(error) => {
                return compile_error(format_args!("{}: Error reading file: {}", args.file, error))
            },
        }
    }

    result.parse().expect("To parse")
}
