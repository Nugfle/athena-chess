use athena_chess::game::create_tables;
use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

/// profiles the speed for creating a attack pattern table
fn bench_table_creation(c: &mut Criterion) {
    c.bench_function("create tables", |b| b.iter(|| create_tables()));
}

/// creates the config to use for the attack pattern table creation benchmark
fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3))
        .nresamples(1000)
}

criterion_group! {name = benches; config = criterion_config(); targets = bench_table_creation}
criterion_main!(benches);
