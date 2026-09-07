use core::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn format(num: u32) -> String {
    format!("bytes: {:b}", num)
}

fn concat(num: u32) -> String {
    use fmt_int::*;

    ["bytes: ", fmt_int!("{:b}", num)].concat()
}

fn bench(c: &mut Criterion) {
    let num = u32::MAX;
    assert_eq!(format(num), concat(num));

    c.benchmark_group("str_concat")
        .bench_function("format", |b| b.iter(|| black_box(format(num))))
        .bench_function("concat", |b| b.iter(|| black_box(concat(num))));
}

criterion_group!(benches, bench);
criterion_main!(benches);
