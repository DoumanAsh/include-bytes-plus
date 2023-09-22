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
fn should_include_u8le_bytes() {
    let included = include_bytes!("tests/include.in" as u8le);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u8, expected.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u8be_bytes() {
    let included = include_bytes!("tests/include.in" as u8be);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u8, expected.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
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
fn should_include_array_u8le_single() {
    let included = include_bytes!("tests/include.in" as [u8le; 48]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u8; 48], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u8be_single() {
    let included = include_bytes!("tests/include.in" as [u8be; 48]);
    let expected = read("tests/include.in").expect("To read source code");
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
fn should_include_array_u8le_multiple() {
    let included = include_bytes!("tests/include.in" as [u8le; 8]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u8; 8], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u8be_multiple() {
    let included = include_bytes!("tests/include.in" as [u8be; 8]);
    let expected = read("tests/include.in").expect("To read source code");
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
fn should_include_u16le() {
    let included = include_bytes!("tests/include.in" as u16le);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u16, expected.len() / 2)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u16be() {
    let included = include_bytes!("tests/include.in" as u16be);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 2 {
        expected.swap(i * 2, i * 2 + 1);
    }
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
fn should_include_array_u16le_single() {
    let included = include_bytes!("tests/include.in" as [u16le; 24]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u16; 24], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u16be_single() {
    let included = include_bytes!("tests/include.in" as [u16be; 24]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 2 {
        expected.swap(i * 2, i * 2 + 1);
    }
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
fn should_include_array_u16le_multiple() {
    let included = include_bytes!("tests/include.in" as [u16le; 12]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u16; 12], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u16be_multiple() {
    let included = include_bytes!("tests/include.in" as [u16be; 12]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 2 {
        expected.swap(i * 2, i * 2 + 1);
    }
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
fn should_include_u32le() {
    let included = include_bytes!("tests/include.in" as u32le);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u32, expected.len() / 4)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u32be() {
    let included = include_bytes!("tests/include.in" as u32be);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 4 {
        expected.swap(i * 4, i * 4 + 3);
        expected.swap(i * 4 + 1, i * 4 + 2);
    }
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
fn should_include_array_u32le_single() {
    let included = include_bytes!("tests/include.in" as [u32le; 12]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u32; 12], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u32be_single() {
    let included = include_bytes!("tests/include.in" as [u32be; 12]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 4 {
        expected.swap(i * 4, i * 4 + 3);
        expected.swap(i * 4 + 1, i * 4 + 2);
    }
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
fn should_include_array_u32le_multiple() {
    let included = include_bytes!("tests/include.in" as [u32le; 6]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u32; 6], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u32be_multiple() {
    let included = include_bytes!("tests/include.in" as [u32be; 6]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 4 {
        expected.swap(i * 4, i * 4 + 3);
        expected.swap(i * 4 + 1, i * 4 + 2);
    }
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
fn should_include_u64le() {
    let included = include_bytes!("tests/include.in" as u64le);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u64, expected.len() / 8)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u64be() {
    let included = include_bytes!("tests/include.in" as u64be);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 8 {
        expected.swap(i * 8, i * 8 + 7);
        expected.swap(i * 8 + 1, i * 8 + 6);
        expected.swap(i * 8 + 2, i * 8 + 5);
        expected.swap(i * 8 + 3, i * 8 + 4);
    }
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
fn should_include_array_u64le_single() {
    let included = include_bytes!("tests/include.in" as [u64le; 6]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u64; 6], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u64be_single() {
    let included = include_bytes!("tests/include.in" as [u64be; 6]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 8 {
        expected.swap(i * 8, i * 8 + 7);
        expected.swap(i * 8 + 1, i * 8 + 6);
        expected.swap(i * 8 + 2, i * 8 + 5);
        expected.swap(i * 8 + 3, i * 8 + 4);
    }
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
fn should_include_array_u64le_multiple() {
    let included = include_bytes!("tests/include.in" as [u64le; 3]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u64; 3], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u64be_multiple() {
    let included = include_bytes!("tests/include.in" as [u64be; 3]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 8 {
        expected.swap(i * 8, i * 8 + 7);
        expected.swap(i * 8 + 1, i * 8 + 6);
        expected.swap(i * 8 + 2, i * 8 + 5);
        expected.swap(i * 8 + 3, i * 8 + 4);
    }
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
fn should_include_u128le() {
    let included = include_bytes!("tests/include.in" as u128le);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u128, expected.len() / 16)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u128be() {
    let included = include_bytes!("tests/include.in" as u128be);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 16 {
        expected.swap(i * 16, i * 16 + 15);
        expected.swap(i * 16 + 1, i * 16 + 14);
        expected.swap(i * 16 + 2, i * 16 + 13);
        expected.swap(i * 16 + 3, i * 16 + 12);
        expected.swap(i * 16 + 4, i * 16 + 11);
        expected.swap(i * 16 + 5, i * 16 + 10);
        expected.swap(i * 16 + 6, i * 16 + 9);
        expected.swap(i * 16 + 7, i * 16 + 8);
    }
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

#[test]
fn should_include_array_u128le_single() {
    let included = include_bytes!("tests/include.in" as [u128le; 3]);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u128; 3], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_array_u128be_single() {
    let included = include_bytes!("tests/include.in" as [u128be; 3]);
    let mut expected = read("tests/include.in").expect("To read source code");
    // swap bytes
    for i in 0..expected.len() / 16 {
        expected.swap(i * 16, i * 16 + 15);
        expected.swap(i * 16 + 1, i * 16 + 14);
        expected.swap(i * 16 + 2, i * 16 + 13);
        expected.swap(i * 16 + 3, i * 16 + 12);
        expected.swap(i * 16 + 4, i * 16 + 11);
        expected.swap(i * 16 + 5, i * 16 + 10);
        expected.swap(i * 16 + 6, i * 16 + 9);
        expected.swap(i * 16 + 7, i * 16 + 8);
    }
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const [u128; 3], included.len())
    };
    assert!(included == expected, "included doesn't match expected file output");
}
