use criterion::{criterion_group, criterion_main, Criterion};
use rust_kzg_bn254::kzg::Kzg;
use std::time::Duration;

fn bench_kzg_setup(c: &mut Criterion) {
    c.bench_function("bench_kzg_setup", |b| {
        b.iter(|| {
            Kzg::setup(
                "src/test-files/g1.point",
                "src/test-files/g2.point",
                "src/test-files/g2.point.powerOf2",
                3000,
                3000,
            )
            .unwrap()
        });

        b.iter(|| {
            Kzg::setup(
                "src/test-files/mainnet-data/g1.131072.point",
                "",
                "src/test-files/mainnet-data/g2.point.powerOf2",
                268435456,
                131072,
            )
            .unwrap()
        });
    });
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(5))  // Warm-up time
        .measurement_time(Duration::from_secs(10))  // Measurement time
        .sample_size(10) // Number of samples to take
}

criterion_group!(
    name = benches;
    config = criterion_config();
    targets = bench_kzg_setup
);
criterion_main!(benches);
