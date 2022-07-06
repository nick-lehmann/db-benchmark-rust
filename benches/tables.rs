#![feature(stdsimd)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use mem::{
    generate_random_data, ColumnTable, Equal, GreaterEqual, RowTable, ScalarFilters, ScalarQuery,
    Table, VectorFilters, VectorisedQuery,
};
use pprof::criterion::{Output, PProfProfiler};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::time::Duration;

fn bench_tables(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tables");
    group.sampling_mode(SamplingMode::Flat);

    let scalar_filters: ScalarFilters<i32, i32> = vec![
        Box::new(Equal::<i32>::new(0, 5)),
        Box::new(GreaterEqual::<i32>::new(1, 3)),
    ];
    let vector_filters: VectorFilters<__m512i, i32, __mmask16> = vec![
        Box::new(Equal::<i32>::new(0, 5)),
        Box::new(GreaterEqual::<i32>::new(1, 3)),
    ];

    let chunk_size = 16;
    let row_counts = [
        // chunk_size,
        // chunk_size * 10,
        chunk_size * 100,
        chunk_size * 1_000,
        chunk_size * 10_000,
    ];

    for rows in row_counts.iter() {
        let data = generate_random_data::<3>(rows);
        let row_table = RowTable::new(data.clone());
        let column_table = ColumnTable::new(data.clone());

        group.bench_with_input(
            BenchmarkId::new("ColumnTable Scalar", rows),
            rows,
            |b, _| {
                b.iter(|| {
                    let indices = ScalarQuery::filter(&column_table, &scalar_filters);
                    return indices;
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("ColumnTable AVX", rows), rows, |b, _| {
            b.iter(|| {
                let indices = unsafe { VectorisedQuery::filter(&column_table, &vector_filters) };
                return indices;
            })
        });
        group.bench_with_input(BenchmarkId::new("RowTable Scalar", rows), rows, |b, _| {
            b.iter(|| {
                let indices = ScalarQuery::filter(&row_table, &scalar_filters);
                return indices;
            })
        });
        group.bench_with_input(BenchmarkId::new("RowTable AVX", rows), rows, |b, _| {
            b.iter(|| {
                let indices = unsafe { VectorisedQuery::filter(&row_table, &vector_filters) };
                return indices;
            })
        });
    }
}

// criterion_group! {
//     name = benches;
//     config = Criterion::default().warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(5)).sample_size(50).with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//     targets = bench_tables
// }
criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(2)).sample_size(20);
    targets = bench_tables
}
criterion_main!(benches);
