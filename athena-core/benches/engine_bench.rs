//! Performance benchmarks for the Athena chess engine.
//!
//! This module provides benchmarks for critical engine operations:
//! - Attack table generation
//! - Move generation performance
//! - Position evaluation speed
//!
//! # Running Benchmarks
//!
//! ```bash
//! cargo bench --features benchmark
//! ```
//!
//! # Configuration
//!
//! Benchmarks use the following settings:
//! - 10 samples per benchmark
//! - 10 second measurement time
//! - 3 second warm-up period
//! - 1000 bootstrap resamples for statistics
//!
//! The benchmarks help identify performance regressions and
//! optimization opportunities in the engine's core functionality.

#![cfg(feature = "benchmark")]
use athena_core::game::create_tables;
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
