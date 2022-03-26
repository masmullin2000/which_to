use criterion::{black_box, criterion_group, criterion_main, Criterion};

use which_to::RawData;

fn test_normal_16(c: &mut Criterion) {
    let ch = 'a';
    let len = 16;
    let raw = RawData::new(ch, len);
    let s = raw.to_str();

    let mut group = c.benchmark_group("normal 16");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(15));
    group.sample_size(1_000);
    group.bench_function("to_string", |b| b.iter(|| black_box(s.to_string())));
    group.bench_function("to_owned", |b| b.iter(|| black_box(s.to_owned())));
    group.finish();
}

fn test_normal_4096(c: &mut Criterion) {
    let ch = 'a';
    let len = 4096;
    let raw = RawData::new(ch, len);
    let s = raw.to_str();

    let mut group = c.benchmark_group("normal 4096");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(15));
    group.sample_size(1_000);
    group.bench_function("to_string", |b| b.iter(|| black_box(s.to_string())));
    group.bench_function("to_owned", |b| b.iter(|| black_box(s.to_owned())));
    group.finish();
}

fn test_utf8_4096(c: &mut Criterion) {
    let ch = '香';
    let len = 1364;
    let raw = RawData::new(ch, len);
    let s = raw.to_str();

    let mut group = c.benchmark_group("utf8 4096");
    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(45));
    group.sample_size(2_000);
    group.bench_function("to_string", |b| b.iter(|| black_box(s.to_string())));
    group.bench_function("to_owned", |b| b.iter(|| black_box(s.to_owned())));
    group.finish();
}

criterion_group!(benches, test_normal_16, test_normal_4096, test_utf8_4096);
criterion_main!(benches);
