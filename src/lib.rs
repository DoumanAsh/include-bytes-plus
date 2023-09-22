//! Improved version of Rust's `include_bytes` macro that allows to reinterpret input as differently array.
//!
//! Due to inability to capture current file path in the stable Rust, this macro only accepts paths relative to crate's root.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

extern crate proc_macro;

use proc_macro::TokenStream;

use core::{fmt, mem};

#[cold]
#[inline(never)]
fn compile_error<T: core::fmt::Display>(text: T) -> TokenStream {
    format!("core::compile_error!(\"{}\")", text).parse().unwrap()
}

enum Primitive {
    U8,
    U16,
    U32,
    U64,
    U128,
    U8LE,
    U16LE,
    U32LE,
    U64LE,
    U128LE,
    U8BE,
    U16BE,
    U32BE,
    U64BE,
    U128BE,
}

impl Primitive {
    const fn size(&self) -> usize {
        match self {
            Primitive::U8 => mem::size_of::<u8>(),
            Primitive::U16 => mem::size_of::<u16>(),
            Primitive::U32 => mem::size_of::<u32>(),
            Primitive::U64 => mem::size_of::<u64>(),
            Primitive::U128 => mem::size_of::<u128>(),
            Primitive::U8LE => mem::size_of::<u8>(),
            Primitive::U16LE => mem::size_of::<u16>(),  
            Primitive::U32LE => mem::size_of::<u32>(),
            Primitive::U64LE => mem::size_of::<u64>(),
            Primitive::U128LE => mem::size_of::<u128>(),
            Primitive::U8BE => mem::size_of::<u8>(),
            Primitive::U16BE => mem::size_of::<u16>(),
            Primitive::U32BE => mem::size_of::<u32>(),
            Primitive::U64BE => mem::size_of::<u64>(),
            Primitive::U128BE => mem::size_of::<u128>(),
        }
    }
}

enum Type {
    Primitive(Primitive),
    Array(Primitive, usize),
}

impl Type {
    ///returns number of bytes written.
    fn parse(input: &str) -> Result<Self, TokenStream> {
        match input {
            "" => Err(compile_error("'as' is missing type")),
            "u8" => Ok(Type::Primitive(Primitive::U8)),
            "u16" => Ok(Type::Primitive(Primitive::U16)),
            "u32" => Ok(Type::Primitive(Primitive::U32)),
            "u64" => Ok(Type::Primitive(Primitive::U64)),
            "u128" => Ok(Type::Primitive(Primitive::U128)),
            "u8le" => Ok(Type::Primitive(Primitive::U8LE)),
            "u16le" => Ok(Type::Primitive(Primitive::U16LE)),
            "u32le" => Ok(Type::Primitive(Primitive::U32LE)),
            "u64le" => Ok(Type::Primitive(Primitive::U64LE)),
            "u128le" => Ok(Type::Primitive(Primitive::U128LE)),
            "u8be" => Ok(Type::Primitive(Primitive::U8BE)),
            "u16be" => Ok(Type::Primitive(Primitive::U16BE)),
            "u32be" => Ok(Type::Primitive(Primitive::U32BE)),
            "u64be" => Ok(Type::Primitive(Primitive::U64BE)),
            "u128be" => Ok(Type::Primitive(Primitive::U128BE)),
            other => {
                if let Some(arg) = input.strip_prefix('[') {
                    if let Some(arg) = arg.strip_suffix(']') {
                        let mut arg_split = arg.split(';');
                        let arr_type = arg_split.next().unwrap();
                        let arr_size = match arg_split.next() {
                            Some(size) => size,
                            None => return Err(compile_error(format_args!("'as' array expression '{}' is missing size", other))),
                        };

                        if let Some(_) = arg_split.next() {
                            return Err(compile_error(format_args!("'as' array expression '{}' has superfluous ';'", other)));
                        }
                        let arr_type = match arr_type {
                            "u8" => Primitive::U8,
                            "u16" => Primitive::U16,
                            "u32" => Primitive::U32,
                            "u64" => Primitive::U64,
                            "u128" => Primitive::U128,
                            "u8le" => Primitive::U8LE,
                            "u16le" => Primitive::U16LE,
                            "u32le" => Primitive::U32LE,
                            "u64le" => Primitive::U64LE,
                            "u128le" => Primitive::U128LE,
                            "u8be" => Primitive::U8BE,
                            "u16be" => Primitive::U16BE,
                            "u32be" => Primitive::U32BE,
                            "u64be" => Primitive::U64BE,
                            "u128be" => Primitive::U128BE,
                            invalid => return Err(compile_error(format_args!("'as' array expression '{}' has invalid type '{}'", other, invalid))),
                        };
                        match arr_size.parse() {
                            Ok(0) => Err(compile_error(format_args!("'as' array expression '{}' has zero size, which makes no sense", other))),
                            Ok(arr_size) => Ok(Type::Array(arr_type, arr_size)),
                            Err(err) => Err(compile_error(format_args!("'as' array expression '{}' has invalid size: {}", other, err))),
                        }
                    } else {
                        Err(compile_error(format_args!("'as' array expression '{}' is missing closing brackets", other)))
                    }
                } else {
                    Err(compile_error(format_args!("'as' specifies unsupported type '{}'", other)))
                }
            },
        }
    }

    ///returns number of bytes written.
    fn write_bytes<O: fmt::Write>(&self, out: &mut O, bytes: &[u8]) -> usize {
        match self {
            Type::Primitive(primitive) => primitive.write_bytes(out, bytes),
            Type::Array(primitive, size) => {
                let mut written = 0;
                let required_size = primitive.size() * size;

                if required_size > bytes.len() {
                    return written;
                }

                for chunk in bytes.chunks_exact(required_size) {
                    out.write_str("[").expect("to write string");
                    written += primitive.write_bytes(out, chunk);
                    out.write_str("],").expect("to write string");
                }

                written
            },
        }
    }
}

impl Primitive {
    ///returns number of bytes written.
    fn write_bytes<O: fmt::Write>(&self, out: &mut O, bytes: &[u8]) -> usize {
        match self {
            Primitive::U8 => {
                for byte in bytes {
                    core::fmt::write(out, format_args!("0x{:x}u8, ", byte)).expect("To write string");
                }
                bytes.len()
            },
            Primitive::U16 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(2) {
                    written += chunk.len();
                    let byte = u16::from_ne_bytes([chunk[0], chunk[1]]);
                    core::fmt::write(out, format_args!("0x{:x}u16, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U32 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(4) {
                    written += chunk.len();
                    let byte = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    core::fmt::write(out, format_args!("0x{:x}u32, ", byte)).expect("To write string");
                }
                written
            }
            Primitive::U64 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(8) {
                    written += chunk.len();
                    let byte = u64::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
                    core::fmt::write(out, format_args!("0x{:x}u64, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U128 => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(16) {
                    written += chunk.len();
                    let byte = u128::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]]);
                    core::fmt::write(out, format_args!("0x{:x}u128, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U8LE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(1) {
                    written += chunk.len();
                    let byte = u8::from_le_bytes([chunk[0]]);
                    core::fmt::write(out, format_args!("0x{:x}u8, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U16LE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(2) {
                    written += chunk.len();
                    let byte = u16::from_le_bytes([chunk[0], chunk[1]]);
                    core::fmt::write(out, format_args!("0x{:x}u16, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U32LE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(4) {
                    written += chunk.len();
                    let byte = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    core::fmt::write(out, format_args!("0x{:x}u32, ", byte)).expect("To write string");
                }
                written
            }
            Primitive::U64LE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(8) {
                    written += chunk.len();
                    let byte = u64::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
                    core::fmt::write(out, format_args!("0x{:x}u64, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U128LE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(16) {
                    written += chunk.len();
                    let byte = u128::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]]);
                    core::fmt::write(out, format_args!("0x{:x}u128, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U8BE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(1) {
                    written += chunk.len();
                    let byte = u8::from_be_bytes([chunk[0]]);
                    core::fmt::write(out, format_args!("0x{:x}u8, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U16BE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(2) {
                    written += chunk.len();
                    let byte = u16::from_be_bytes([chunk[0], chunk[1]]);
                    core::fmt::write(out, format_args!("0x{:x}u16, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U32BE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(4) {
                    written += chunk.len();
                    let byte = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    core::fmt::write(out, format_args!("0x{:x}u32, ", byte)).expect("To write string");
                }
                written
            }
            Primitive::U64BE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(8) {
                    written += chunk.len();
                    let byte = u64::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7]]);
                    core::fmt::write(out, format_args!("0x{:x}u64, ", byte)).expect("To write string");
                }
                written
            },
            Primitive::U128BE => {
                let mut written = 0;
                for chunk in bytes.chunks_exact(16) {
                    written += chunk.len();
                    let byte = u128::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]]);
                    core::fmt::write(out, format_args!("0x{:x}u128, ", byte)).expect("To write string");
                }
                written
            },
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Primitive::U8 => fmt.write_str("u8"),
            Primitive::U16 => fmt.write_str("u16"),
            Primitive::U32 => fmt.write_str("u32"),
            Primitive::U64 => fmt.write_str("u64"),
            Primitive::U128 => fmt.write_str("u128"),
            Primitive::U8LE => fmt.write_str("u8le"),
            Primitive::U16LE => fmt.write_str("u16le"),
            Primitive::U32LE => fmt.write_str("u32le"),
            Primitive::U64LE => fmt.write_str("u64le"),
            Primitive::U128LE => fmt.write_str("u128le"),
            Primitive::U8BE => fmt.write_str("u8be"),
            Primitive::U16BE => fmt.write_str("u16be"),
            Primitive::U32BE => fmt.write_str("u32be"),
            Primitive::U64BE => fmt.write_str("u64be"),
            Primitive::U128BE => fmt.write_str("u128be"),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Primitive(primitive) => fmt::Display::fmt(primitive, fmt),
            Type::Array(primitive, size) => fmt.write_fmt(format_args!("[{}; {}]", primitive, size)),
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

        let input = input.trim();
        let mut split = input.split_whitespace();

        let typ = match split.next() {
            Some("as") => {
                let arg = split.fold(String::new(), |mut acc, part| {
                    acc.push_str(part);
                    acc
                });
                let arg = arg.trim();

                Type::parse(arg)?
            },
            Some(other) => return Err(compile_error(format_args!("Unsupported syntax after file name '{}'", other))),
            None => Type::Primitive(Primitive::U8),
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
///Change the endianness of the type by adding `le` or `be` suffix, e.g. `u16le` or `u32be`.
///The default endianness is native.
///
///# NOTE:
///
///Due to `Span::source_file` being unstable, the file is searched relative to crate root.
///
///# Supported types:
///
///- Primitive fixed sized unsigned integers;
///
///- Arrays with unsigned integers;
///
///# Usage:
///
///```
///use include_bytes_plus::include_bytes;
///
///let bytes = include_bytes!("tests/include.in");
///let bytes_u16 = include_bytes!("tests/include.in" as u16);
///let bytes_u16_2 = include_bytes!("tests/include with whitespaces.in" as u16);
///let bytes_u16_3 = include_bytes!("tests/include with whitespaces.in" as [u8; 48]);
///let bytes_u16_4 = include_bytes!("tests/include with whitespaces.in" as [u16; 12]);
///let bytes_u16be = include_bytes!("tests/include.in" as u16be);
///
///assert_eq!(bytes.len(), bytes_u16.len() * 2);
///assert_eq!(bytes.len(), bytes_u16_2.len() * 2);
///assert_eq!(bytes_u16_3.len(), 1);
///assert_eq!(bytes_u16_3[0].len(), 48);
///assert_eq!(bytes_u16_4.len(), 2);
///assert_eq!(bytes_u16_4[0].len(), 12);
///assert_eq!(bytes_u16_4[1].len(), 12);
///assert_eq!(bytes_u16be.len(), bytes_u16.len());
///```
///
///# Debugging timings:
///
///Set env variable `RUST_INCLUDE_BYTES_LOG=1` to enable logging of each parsed file statistics
pub fn include_bytes(input: TokenStream) -> TokenStream {
    let is_log = match std::env::var("RUST_INCLUDE_BYTES_LOG") {
        Ok(res) => match res.as_str() {
            "1" | "true" => true,
            _ => false,
        },
        _ => false,
    };

    let now = std::time::Instant::now();

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

                if written > 0 {
                    unsafe {
                        core::ptr::copy(buf.as_ptr().add(written), buf.as_mut_ptr(), buf_len - written);
                    }
                    cursor = buf_len - written;
                } else {
                    //not enough data to write expression
                    //wait another read.
                    cursor = buf_len;
                }
            },
            Err(error) => {
                return compile_error(format_args!("{}: Error reading file: {}", args.file, error))
            },
        }
    }

    if is_log {
        let elapsed = now.elapsed();
        let secs = elapsed.as_secs();
        let ms = elapsed.subsec_millis();

        if secs > 0 {
            println!("{}: parsed {}b in {}.{} seconds", args.file, file_len, secs, ms);
        } else {
            println!("{}: parsed {}b in {} ms", args.file, file_len, ms);
        }
    }

    result.parse().expect("To parse")
}
