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

#[test]
fn test_const() {
    {
        const N: u8 = u8::MAX;
        assert_eq!(const_fmt_int!(N, u8), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, u8), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, u8), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, u8), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, u8), format!("{:X}", N));
    }
    {
        const N: u16 = u16::MAX;
        assert_eq!(const_fmt_int!(N, u16), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, u16), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, u16), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, u16), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, u16), format!("{:X}", N));
    }
    {
        const N: u32 = u32::MAX;
        assert_eq!(const_fmt_int!(N, u32), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, u32), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, u32), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, u32), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, u32), format!("{:X}", N));
    }
    {
        const N: u64 = u64::MAX;
        assert_eq!(const_fmt_int!(N, u64), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, u64), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, u64), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, u64), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, u64), format!("{:X}", N));
    }
    {
        const N: u128 = u128::MAX;
        assert_eq!(const_fmt_int!(N, u128), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, u128), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, u128), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, u128), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, u128), format!("{:X}", N));
    }
    {
        const N: usize = usize::MAX;
        assert_eq!(const_fmt_int!(N, usize), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, usize), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, usize), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, usize), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, usize), format!("{:X}", N));
    }
    {
        const N: i8 = i8::MAX;
        assert_eq!(const_fmt_int!(N, i8), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i8), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i8), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i8), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i8), format!("{:X}", N));
    }
    {
        const N: i16 = i16::MAX;
        assert_eq!(const_fmt_int!(N, i16), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i16), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i16), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i16), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i16), format!("{:X}", N));
    }
    {
        const N: i32 = i32::MAX;
        assert_eq!(const_fmt_int!(N, i32), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i32), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i32), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i32), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i32), format!("{:X}", N));
    }
    {
        const N: i64 = i64::MAX;
        assert_eq!(const_fmt_int!(N, i64), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i64), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i64), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i64), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i64), format!("{:X}", N));
    }
    {
        const N: i128 = i128::MAX;
        assert_eq!(const_fmt_int!(N, i128), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i128), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i128), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i128), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i128), format!("{:X}", N));
    }
    {
        const N: isize = isize::MAX;
        assert_eq!(const_fmt_int!(N, isize), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, isize), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, isize), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, isize), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, isize), format!("{:X}", N));
    }
    {
        const N: i8 = i8::MIN;
        assert_eq!(const_fmt_int!(N, i8), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i8), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i8), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i8), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i8), format!("{:X}", N));
    }
    {
        const N: i16 = i16::MIN;
        assert_eq!(const_fmt_int!(N, i16), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i16), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i16), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i16), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i16), format!("{:X}", N));
    }
    {
        const N: i32 = i32::MIN;
        assert_eq!(const_fmt_int!(N, i32), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i32), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i32), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i32), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i32), format!("{:X}", N));
    }
    {
        const N: i64 = i64::MIN;
        assert_eq!(const_fmt_int!(N, i64), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i64), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i64), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i64), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i64), format!("{:X}", N));
    }
    {
        const N: i128 = i128::MIN;
        assert_eq!(const_fmt_int!(N, i128), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, i128), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, i128), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, i128), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, i128), format!("{:X}", N));
    }
    {
        const N: isize = isize::MIN;
        assert_eq!(const_fmt_int!(N, isize), format!("{}", N));
        assert_eq!(const_fmt_int!("{:b}", N, isize), format!("{:b}", N));
        assert_eq!(const_fmt_int!("{:o}", N, isize), format!("{:o}", N));
        assert_eq!(const_fmt_int!("{:x}", N, isize), format!("{:x}", N));
        assert_eq!(const_fmt_int!("{:X}", N, isize), format!("{:X}", N));
    }
}
