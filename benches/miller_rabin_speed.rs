use criterion::{criterion_group, criterion_main, Criterion};
use rsa_algorithm::prime_num_gen;

fn bench_miller_rabin_speed(c: &mut Criterion) {
    c.bench_function("miller_rabin_speed", |b| {
        b.iter(|| prime_num_gen::find_prime(2048))
    });
}

criterion_group!(benches, bench_miller_rabin_speed);
criterion_main!(benches);
