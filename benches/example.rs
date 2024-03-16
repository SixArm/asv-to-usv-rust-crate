use criterion::{criterion_group, criterion_main, Criterion};
use rand::*;
use rand::distributions::*;
use asv_to_usv::asv_to_usv;

/// Generate a random string of given length
#[inline]
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate random USV data
pub fn random_asv_data(unit_length: usize, unit_count: usize, record_count: usize) -> String {
    let mut s = String::new();
    for _ in 0..record_count {
        for _ in 0..unit_count {
            s += &random_string(unit_length);
            s += "\u{001F}";
        }
        s += "\u{001D}";
    }
    s
}

fn bench_example(c: &mut Criterion){
    let unit_length = 10;
    let unit_count = 1000;
    let record_count = 1000;

    let asv_data = random_asv_data(unit_length, unit_count, record_count);

    let mut group = c.benchmark_group(
        &format!("benchmark group unit_length: {}, unit_count: {}, record_count: {}", unit_length, unit_count, record_count)
    );
    group.sample_size(10);
    group.bench_function("asv_to_usv", |b| b.iter(|| asv_to_usv(&asv_data)));
    group.finish();
}

criterion_group!(benches, bench_example);
criterion_main!(benches);
