use core::{marker::PhantomData, mem::MaybeUninit};

#[doc(hidden)]
// A namespace to avoid using `paste::paste!`.
pub struct Const<Radix: RadixMarker, Int>(PhantomData<(Radix, Int)>);

// Based on [standard library](https://doc.rust-lang.org/1.98.0/src/core/fmt/num.rs.html).
macro_rules! radix {
    ($radix:ty, $signed:ident and $unsigned:ident, $dig_tab:literal) => {
        impl NumBufferTrait<$radix> for $unsigned {
            type Buf =
                [MaybeUninit<u8>; $unsigned::MAX.ilog($dig_tab.len() as $unsigned) as usize + 1];
            const DEFAULT: Self::Buf = [MaybeUninit::uninit(); _];
        }

        impl NumBufferTrait<$radix> for $signed {
            type Buf = <$unsigned as NumBufferTrait<$radix>>::Buf;
            const DEFAULT: Self::Buf = <$unsigned as NumBufferTrait<$radix>>::DEFAULT;
        }

        impl Const<$radix, $unsigned> {
            pub const fn fmt(this: $unsigned, buf: &mut NumBuffer<$radix, $unsigned>) -> usize {
                // ASCII digits in ascending order are used as a lookup table.
                const DIG_TAB: &[u8] = $dig_tab;
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf = &mut buf.0;
                // Count the number of bytes in `buf` that are not initialized.
                let mut offset = buf.len();
                // Accumulate each digit of the number from the least
                // significant to the most significant figure.
                let mut remain = this;
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf.len()) }
                    buf[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                offset
            }
        }

        impl Const<$radix, $signed> {
            // Format signed integers in the two's-complement form.
            pub const fn fmt(this: $signed, buf: &mut NumBuffer<$radix, $signed>) -> usize {
                // SAFETY:
                // `NumBuffer<$radix, $signed>` and `NumBuffer<$radix, $unsigned>`
                // are the same things, see above.
                let buf: &mut NumBuffer<$radix, $unsigned> = unsafe { core::mem::transmute(buf) };
                let offset = Const::<$radix, $unsigned>::fmt(this.cast_unsigned(), buf);
                offset
            }
        }

        impl FmtInto<$radix> for $unsigned {
            #[inline]
            fn fmt_into(self, buf: &mut NumBuffer<$radix, Self>) -> &str {
                let offset = crate::Const::<$radix, $unsigned>::fmt(self, buf);
                // SAFETY: `offset` is always included between 0 and `buf`'s length.
                let written = unsafe { buf.0.get_unchecked(offset..) };
                // SAFETY: (`assume_init_ref`) All `buf` content since offset is set.
                // SAFETY: (`from_utf8_unchecked`) Writes use ASCII from the lookup table exclusively.
                unsafe { str::from_utf8_unchecked(written.assume_init_ref()) }
            }
        }

        impl FmtInto<$radix> for $signed {
            // Format signed integers in the two's-complement form.
            #[inline]
            fn fmt_into(self, buf: &mut NumBuffer<$radix, Self>) -> &str {
                // SAFETY:
                // `NumBuffer<$radix, $signed>` and `NumBuffer<$radix, $unsigned>`
                // are the same things, see above.
                let buf: &mut NumBuffer<$radix, $unsigned> = unsafe { core::mem::transmute(buf) };
                self.cast_unsigned().fmt_into(buf)
            }
        }
    };
}

macro_rules! radixes {
    ($signed:ident, $unsigned:ident) => {
        radix! { Binary,   $signed and $unsigned, b"01" }
        radix! { Octal,    $signed and $unsigned, b"01234567" }
        radix! { LowerHex, $signed and $unsigned, b"0123456789abcdef" }
        radix! { UpperHex, $signed and $unsigned, b"0123456789ABCDEF" }
    };
}

radixes! { i8, u8 }
radixes! { i16, u16 }
radixes! { i32, u32 }
radixes! { i64, u64 }
radixes! { i128, u128 }
radixes! { isize, usize }

pub struct NumBuffer<Radix: RadixMarker, N: NumBufferTrait<Radix>>(#[doc(hidden)] pub N::Buf);

impl<Radix: RadixMarker, N: NumBufferTrait<Radix>> NumBuffer<Radix, N> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(N::DEFAULT)
    }
}

pub trait NumBufferTrait<Radix: RadixMarker> {
    #[doc(hidden)]
    type Buf;
    #[doc(hidden)]
    const DEFAULT: Self::Buf;
}

/// Radix marker.
pub struct Binary;
/// Radix marker.
pub struct Octal;
/// Radix marker.
pub struct LowerHex;
/// Radix marker.
pub struct UpperHex;

pub trait RadixMarker {}

impl RadixMarker for Binary {}
impl RadixMarker for Octal {}
impl RadixMarker for LowerHex {}
impl RadixMarker for UpperHex {}

/// Import this before you use [`fmt_int!`](super::fmt_int).
pub trait FmtInto<Radix: RadixMarker>: NumBufferTrait<Radix> + Sized {
    fn fmt_into(self, buf: &mut NumBuffer<Radix, Self>) -> &str;
}

// The followings should only be used in `const` context via `const_fmt_int!`.

/// Do not use this at run-time because the `std` way is more optimal.
#[doc(hidden)]
pub struct Decimal;

impl RadixMarker for Decimal {}

macro_rules! decimal {
    ($signed:ident, $unsigned:ident) => {
        impl NumBufferTrait<Decimal> for $unsigned {
            type Buf = [MaybeUninit<u8>; $unsigned::MAX.ilog10() as usize + 1];
            const DEFAULT: Self::Buf = [MaybeUninit::uninit(); _];
        }

        impl NumBufferTrait<Decimal> for $signed {
            type Buf = [MaybeUninit<u8>; $signed::MAX.ilog10() as usize + 1 + 1];
            const DEFAULT: Self::Buf = [MaybeUninit::uninit(); _];
        }

        impl Const<Decimal, $unsigned> {
            /// Do not use this at run-time because the `std` way is more optimal.
            pub const fn fmt(this: $unsigned, buf: &mut NumBuffer<Decimal, $unsigned>) -> usize {
                const DIG_TAB: &[u8] = b"0123456789";
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf = &mut buf.0;
                let mut offset = buf.len();
                let mut remain = this;
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf.len()) }
                    buf[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                offset
            }
        }

        impl Const<Decimal, $signed> {
            /// Do not use this at run-time because the `std` way is more optimal.
            pub const fn fmt(this: $signed, buf: &mut NumBuffer<Decimal, $signed>) -> usize {
                const DIG_TAB: &[u8] = b"0123456789";
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf = &mut buf.0;
                let mut offset = buf.len();
                let mut remain = this.unsigned_abs();
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf.len()) }
                    buf[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                if this < 0 {
                    offset -= 1;
                    buf[offset].write(b'-');
                }
                offset
            }
        }
    };
}

decimal! { i8, u8 }
decimal! { i16, u16 }
decimal! { i32, u32 }
decimal! { i64, u64 }
decimal! { i128, u128 }
decimal! { isize, usize }
