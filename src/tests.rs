use super::*;

#[test]
#[rustfmt::skip]
fn test_fmt_int_in_decimal() {
    assert_eq!(fmt_int!(i8::MAX),    format!("{}", i8::MAX));
    assert_eq!(fmt_int!(i16::MAX),   format!("{}", i16::MAX));
    assert_eq!(fmt_int!(i32::MAX),   format!("{}", i32::MAX));
    assert_eq!(fmt_int!(i64::MAX),   format!("{}", i64::MAX));
    assert_eq!(fmt_int!(i128::MAX),  format!("{}", i128::MAX));
    assert_eq!(fmt_int!(isize::MAX), format!("{}", isize::MAX));

    assert_eq!(fmt_int!(i8::MIN),    format!("{}", i8::MIN));
    assert_eq!(fmt_int!(i16::MIN),   format!("{}", i16::MIN));
    assert_eq!(fmt_int!(i32::MIN),   format!("{}", i32::MIN));
    assert_eq!(fmt_int!(i64::MIN),   format!("{}", i64::MIN));
    assert_eq!(fmt_int!(i128::MIN),  format!("{}", i128::MIN));
    assert_eq!(fmt_int!(isize::MIN), format!("{}", isize::MIN));

    assert_eq!(fmt_int!(u8::MAX),    format!("{}", u8::MAX));
    assert_eq!(fmt_int!(u16::MAX),   format!("{}", u16::MAX));
    assert_eq!(fmt_int!(u32::MAX),   format!("{}", u32::MAX));
    assert_eq!(fmt_int!(u64::MAX),   format!("{}", u64::MAX));
    assert_eq!(fmt_int!(u128::MAX),  format!("{}", u128::MAX));
    assert_eq!(fmt_int!(usize::MAX), format!("{}", usize::MAX));
}

#[test]
#[rustfmt::skip]
fn test_fmt_int_in_binary() {
    assert_eq!(fmt_int!("{:b}", i8::MAX),    format!("{:b}", i8::MAX));
    assert_eq!(fmt_int!("{:b}", i16::MAX),   format!("{:b}", i16::MAX));
    assert_eq!(fmt_int!("{:b}", i32::MAX),   format!("{:b}", i32::MAX));
    assert_eq!(fmt_int!("{:b}", i64::MAX),   format!("{:b}", i64::MAX));
    assert_eq!(fmt_int!("{:b}", i128::MAX),  format!("{:b}", i128::MAX));
    assert_eq!(fmt_int!("{:b}", isize::MAX), format!("{:b}", isize::MAX));

    assert_eq!(fmt_int!("{:b}", i8::MIN),    format!("{:b}", i8::MIN));
    assert_eq!(fmt_int!("{:b}", i16::MIN),   format!("{:b}", i16::MIN));
    assert_eq!(fmt_int!("{:b}", i32::MIN),   format!("{:b}", i32::MIN));
    assert_eq!(fmt_int!("{:b}", i64::MIN),   format!("{:b}", i64::MIN));
    assert_eq!(fmt_int!("{:b}", i128::MIN),  format!("{:b}", i128::MIN));
    assert_eq!(fmt_int!("{:b}", isize::MIN), format!("{:b}", isize::MIN));

    assert_eq!(fmt_int!("{:b}", u8::MAX),    format!("{:b}", u8::MAX));
    assert_eq!(fmt_int!("{:b}", u16::MAX),   format!("{:b}", u16::MAX));
    assert_eq!(fmt_int!("{:b}", u32::MAX),   format!("{:b}", u32::MAX));
    assert_eq!(fmt_int!("{:b}", u64::MAX),   format!("{:b}", u64::MAX));
    assert_eq!(fmt_int!("{:b}", u128::MAX),  format!("{:b}", u128::MAX));
    assert_eq!(fmt_int!("{:b}", usize::MAX), format!("{:b}", usize::MAX));
}

#[test]
#[rustfmt::skip]
fn test_fmt_int_in_octal() {
    assert_eq!(fmt_int!("{:o}", i8::MAX),    format!("{:o}", i8::MAX));
    assert_eq!(fmt_int!("{:o}", i16::MAX),   format!("{:o}", i16::MAX));
    assert_eq!(fmt_int!("{:o}", i32::MAX),   format!("{:o}", i32::MAX));
    assert_eq!(fmt_int!("{:o}", i64::MAX),   format!("{:o}", i64::MAX));
    assert_eq!(fmt_int!("{:o}", i128::MAX),  format!("{:o}", i128::MAX));
    assert_eq!(fmt_int!("{:o}", isize::MAX), format!("{:o}", isize::MAX));

    assert_eq!(fmt_int!("{:o}", i8::MIN),    format!("{:o}", i8::MIN));
    assert_eq!(fmt_int!("{:o}", i16::MIN),   format!("{:o}", i16::MIN));
    assert_eq!(fmt_int!("{:o}", i32::MIN),   format!("{:o}", i32::MIN));
    assert_eq!(fmt_int!("{:o}", i64::MIN),   format!("{:o}", i64::MIN));
    assert_eq!(fmt_int!("{:o}", i128::MIN),  format!("{:o}", i128::MIN));
    assert_eq!(fmt_int!("{:o}", isize::MIN), format!("{:o}", isize::MIN));

    assert_eq!(fmt_int!("{:o}", u8::MAX),    format!("{:o}", u8::MAX));
    assert_eq!(fmt_int!("{:o}", u16::MAX),   format!("{:o}", u16::MAX));
    assert_eq!(fmt_int!("{:o}", u32::MAX),   format!("{:o}", u32::MAX));
    assert_eq!(fmt_int!("{:o}", u64::MAX),   format!("{:o}", u64::MAX));
    assert_eq!(fmt_int!("{:o}", u128::MAX),  format!("{:o}", u128::MAX));
    assert_eq!(fmt_int!("{:o}", usize::MAX), format!("{:o}", usize::MAX));
}

#[test]
#[rustfmt::skip]
fn test_fmt_int_in_lowerhex() {
    assert_eq!(fmt_int!("{:x}", i8::MAX),    format!("{:x}", i8::MAX));
    assert_eq!(fmt_int!("{:x}", i16::MAX),   format!("{:x}", i16::MAX));
    assert_eq!(fmt_int!("{:x}", i32::MAX),   format!("{:x}", i32::MAX));
    assert_eq!(fmt_int!("{:x}", i64::MAX),   format!("{:x}", i64::MAX));
    assert_eq!(fmt_int!("{:x}", i128::MAX),  format!("{:x}", i128::MAX));
    assert_eq!(fmt_int!("{:x}", isize::MAX), format!("{:x}", isize::MAX));

    assert_eq!(fmt_int!("{:x}", i8::MIN),    format!("{:x}", i8::MIN));
    assert_eq!(fmt_int!("{:x}", i16::MIN),   format!("{:x}", i16::MIN));
    assert_eq!(fmt_int!("{:x}", i32::MIN),   format!("{:x}", i32::MIN));
    assert_eq!(fmt_int!("{:x}", i64::MIN),   format!("{:x}", i64::MIN));
    assert_eq!(fmt_int!("{:x}", i128::MIN),  format!("{:x}", i128::MIN));
    assert_eq!(fmt_int!("{:x}", isize::MIN), format!("{:x}", isize::MIN));

    assert_eq!(fmt_int!("{:x}", u8::MAX),    format!("{:x}", u8::MAX));
    assert_eq!(fmt_int!("{:x}", u16::MAX),   format!("{:x}", u16::MAX));
    assert_eq!(fmt_int!("{:x}", u32::MAX),   format!("{:x}", u32::MAX));
    assert_eq!(fmt_int!("{:x}", u64::MAX),   format!("{:x}", u64::MAX));
    assert_eq!(fmt_int!("{:x}", u128::MAX),  format!("{:x}", u128::MAX));
    assert_eq!(fmt_int!("{:x}", usize::MAX), format!("{:x}", usize::MAX));
}

#[test]
#[rustfmt::skip]
fn test_fmt_int_in_upperhex() {
    assert_eq!(fmt_int!("{:X}", i8::MAX),    format!("{:X}", i8::MAX));
    assert_eq!(fmt_int!("{:X}", i16::MAX),   format!("{:X}", i16::MAX));
    assert_eq!(fmt_int!("{:X}", i32::MAX),   format!("{:X}", i32::MAX));
    assert_eq!(fmt_int!("{:X}", i64::MAX),   format!("{:X}", i64::MAX));
    assert_eq!(fmt_int!("{:X}", i128::MAX),  format!("{:X}", i128::MAX));
    assert_eq!(fmt_int!("{:X}", isize::MAX), format!("{:X}", isize::MAX));

    assert_eq!(fmt_int!("{:X}", i8::MIN),    format!("{:X}", i8::MIN));
    assert_eq!(fmt_int!("{:X}", i16::MIN),   format!("{:X}", i16::MIN));
    assert_eq!(fmt_int!("{:X}", i32::MIN),   format!("{:X}", i32::MIN));
    assert_eq!(fmt_int!("{:X}", i64::MIN),   format!("{:X}", i64::MIN));
    assert_eq!(fmt_int!("{:X}", i128::MIN),  format!("{:X}", i128::MIN));
    assert_eq!(fmt_int!("{:X}", isize::MIN), format!("{:X}", isize::MIN));

    assert_eq!(fmt_int!("{:X}", u8::MAX),    format!("{:X}", u8::MAX));
    assert_eq!(fmt_int!("{:X}", u16::MAX),   format!("{:X}", u16::MAX));
    assert_eq!(fmt_int!("{:X}", u32::MAX),   format!("{:X}", u32::MAX));
    assert_eq!(fmt_int!("{:X}", u64::MAX),   format!("{:X}", u64::MAX));
    assert_eq!(fmt_int!("{:X}", u128::MAX),  format!("{:X}", u128::MAX));
    assert_eq!(fmt_int!("{:X}", usize::MAX), format!("{:X}", usize::MAX));
}
