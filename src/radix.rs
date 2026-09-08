use core::mem::MaybeUninit;

// Based on [standard library](https://doc.rust-lang.org/1.98.0/src/core/fmt/num.rs.html).
macro_rules! radix_integer {
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

        impl FmtInto<$radix> for $unsigned {
            fn fmt_into(self, buf: &mut NumBuffer<$radix, Self>) -> &str {
                // ASCII digits in ascending order are used as a lookup table.
                const DIG_TAB: &[u8] = $dig_tab;
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf = &mut buf.0;
                // Count the number of bytes in `buf` that are not initialized.
                let mut offset = buf.len();

                // Accumulate each digit of the number from the least
                // significant to the most significant figure.
                let mut remain = self;
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

                // SAFETY: `offset` is always included between 0 and `buf`'s length.
                let written = unsafe { buf.get_unchecked(offset..) };
                // SAFETY: (`assume_init_ref`) All `buf` content since offset is set.
                // SAFETY: (`from_utf8_unchecked`) Writes use ASCII from the lookup table exclusively.
                unsafe { str::from_utf8_unchecked(written.assume_init_ref()) }
            }
        }

        impl FmtInto<$radix> for $signed {
            // Format signed integers in the two's-complement form.
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

macro_rules! radix_integers {
    ($signed:ident, $unsigned:ident) => {
        radix_integer! { Binary,   $signed and $unsigned, b"01" }
        radix_integer! { Octal,    $signed and $unsigned, b"01234567" }
        radix_integer! { LowerHex, $signed and $unsigned, b"0123456789abcdef" }
        radix_integer! { UpperHex, $signed and $unsigned, b"0123456789ABCDEF" }
    };
}

radix_integers! { i8, u8 }
radix_integers! { i16, u16 }
radix_integers! { i32, u32 }
radix_integers! { i64, u64 }
radix_integers! { i128, u128 }
radix_integers! { isize, usize }

pub struct NumBuffer<Radix: RadixMarker, N: NumBufferTrait<Radix>>(N::Buf);

impl<Radix: RadixMarker, N: NumBufferTrait<Radix>> NumBuffer<Radix, N> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
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

/// Import this before you use [`fmt_int`](super::fmt_int).
pub trait FmtInto<Radix: RadixMarker>: NumBufferTrait<Radix> + Sized {
    fn fmt_into(self, buf: &mut NumBuffer<Radix, Self>) -> &str;
}
