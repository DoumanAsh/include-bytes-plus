use include_bytes_plus::include_bytes;

use std::fs::read;
use core::{mem, slice};

#[test]
fn should_include_u8_bytes() {
    let included = include_bytes!("tests/include.in");
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    assert!(included == expected.as_slice(), "included doesn't match expected file output");
}

#[test]
fn should_include_array_u8_single() {
    let included = include_bytes!("tests/include.in" as [u8; 48]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u8; 48], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u8_multiple() {
    let included = include_bytes!("tests/include.in" as [u8; 8]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u8; 8], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u16() {
    let included = include_bytes!("tests/include.in" as u16);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u16, expected.len() / 2)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u16_single() {
    let included = include_bytes!("tests/include.in" as [u16; 24]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u16; 24], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u16_multiple() {
    let included = include_bytes!("tests/include.in" as [u16; 12]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u16; 12], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u32() {
    let included = include_bytes!("tests/include.in" as u32);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u32, expected.len() / 4)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u32_single() {
    let included = include_bytes!("tests/include.in" as [u32; 12]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u32; 12], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u32_multiple() {
    let included = include_bytes!("tests/include.in" as [u32; 6]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u32; 6], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u64() {
    let included = include_bytes!("tests/include.in" as u64);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u64, expected.len() / 8)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u64_single() {
    let included = include_bytes!("tests/include.in" as [u64; 6]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u64; 6], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u64_multiple() {
    let included = include_bytes!("tests/include.in" as [u64; 3]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u64; 3], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u128() {
    let included = include_bytes!("tests/include.in" as u128);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u128, expected.len() / 16)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u128_single() {
    let included = include_bytes!("tests/include.in" as [u128; 3]);
    let expected = read("tests/include.in").expect("To read source code");
    assert_eq!(mem::size_of_val(&included), expected.len());
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u128; 3], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}
