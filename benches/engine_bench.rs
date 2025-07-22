use athena_chess::engine::create_tables;
use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn bench_table_creation(c: &mut Criterion) { c.bench_function("create tables", |b| b.iter(|| create_tables())); }

// Configure globally
fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(100) // Default is 100
        .measurement_time(Duration::from_secs(10)) // Default is 5 seconds
        .warm_up_time(Duration::from_secs(3)) // Default is 3 seconds
        .nresamples(100000) // Default is 100000
}

criterion_group! {name = benches; config = criterion_config(); targets = bench_table_creation}
criterion_main!(benches);
