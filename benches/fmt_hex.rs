use core::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn std_bench() {
    use core::fmt::Write;

    struct Sink;
    impl Write for Sink {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            black_box(s);
            Ok(())
        }
    }

    let mut s = Sink;
    macro_rules! sink {
        ($n:expr) => {
            _ = write!(s, "{:x}", $n);
        };
    }
    sink!(i8::MAX);
    sink!(i16::MAX);
    sink!(i32::MAX);
    sink!(i64::MAX);
    sink!(i128::MAX);
    sink!(i8::MIN);
    sink!(i16::MIN);
    sink!(i32::MIN);
    sink!(i64::MIN);
    sink!(i128::MIN);

    sink!(u8::MAX);
    sink!(u16::MAX);
    sink!(u32::MAX);
    sink!(u64::MAX);
    sink!(u128::MAX);
}

fn numtoa_bench() {
    use numtoa::*;

    const BASE: i8 = 16;
    black_box(numtoa_i8_str(
        i8::MAX,
        BASE,
        &mut [0; required_space(BASE as _, i8::MAX as _, true).unwrap()],
    ));
    black_box(numtoa_i8_str(
        i8::MIN,
        BASE,
        &mut [0; required_space(BASE as _, i8::MIN as _, true).unwrap()],
    ));
    black_box(numtoa_i16_str(
        i16::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, i16::MAX as _, true).unwrap()],
    ));
    black_box(numtoa_i16_str(
        i16::MIN,
        BASE as _,
        &mut [0; required_space(BASE as _, i16::MIN as _, true).unwrap()],
    ));
    black_box(numtoa_i32_str(
        i32::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, i32::MAX as _, true).unwrap()],
    ));
    black_box(numtoa_i32_str(
        i32::MIN,
        BASE as _,
        &mut [0; required_space(BASE as _, i32::MIN as _, true).unwrap()],
    ));
    black_box(numtoa_i64_str(
        i64::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, i64::MAX as _, true).unwrap()],
    ));
    black_box(numtoa_i64_str(
        i64::MIN,
        BASE as _,
        &mut [0; required_space(BASE as _, i64::MIN as _, true).unwrap()],
    ));
    black_box(numtoa_i128_str(
        i128::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, i128::MAX as _, true).unwrap()],
    ));
    black_box(numtoa_i128_str(
        i128::MIN,
        BASE as _,
        &mut [0; required_space(BASE as _, i128::MIN as _, true).unwrap()],
    ));

    black_box(numtoa_u8_str(
        u8::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, u8::MAX as _, false).unwrap()],
    ));
    black_box(numtoa_u16_str(
        u16::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, u16::MAX as _, false).unwrap()],
    ));
    black_box(numtoa_u32_str(
        u32::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, u32::MAX as _, false).unwrap()],
    ));
    black_box(numtoa_u64_str(
        u64::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, u64::MAX as _, false).unwrap()],
    ));
    black_box(numtoa_u128_str(
        u128::MAX,
        BASE as _,
        &mut [0; required_space(BASE as _, u128::MAX as _, false).unwrap()],
    ));
}

fn fmt_int_bench() {
    use fmt_int::*;

    black_box(fmt_int!("{:x}", i8::MAX));
    black_box(fmt_int!("{:x}", i16::MAX));
    black_box(fmt_int!("{:x}", i32::MAX));
    black_box(fmt_int!("{:x}", i64::MAX));
    black_box(fmt_int!("{:x}", i128::MAX));
    black_box(fmt_int!("{:x}", isize::MAX));
    black_box(fmt_int!("{:x}", i8::MIN));
    black_box(fmt_int!("{:x}", i16::MIN));
    black_box(fmt_int!("{:x}", i32::MIN));
    black_box(fmt_int!("{:x}", i64::MIN));
    black_box(fmt_int!("{:x}", i128::MIN));

    black_box(fmt_int!("{:x}", u8::MAX));
    black_box(fmt_int!("{:x}", u16::MAX));
    black_box(fmt_int!("{:x}", u32::MAX));
    black_box(fmt_int!("{:x}", u64::MAX));
    black_box(fmt_int!("{:x}", u128::MAX));
}

fn bench(c: &mut Criterion) {
    c.benchmark_group("fmt_hex")
        .bench_function("std", |b| b.iter(std_bench))
        .bench_function("numtoa", |b| b.iter(numtoa_bench))
        .bench_function("fmt_int", |b| b.iter(fmt_int_bench));
}

criterion_group!(benches, bench);
criterion_main!(benches);
