#![feature(stdsimd)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mem::{
    generate_data, ColumnTable, Equal, GreaterEqual, RowTable, ScalarFilters, ScalarQuery, Table,
    VectorFilters, VectorisedQuery,
};
use pprof::criterion::{Output, PProfProfiler};
use rand::prelude::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::time::Duration;

// fn generate_random_data<T: Sized + Copy + From<i32>, const ATTRS: usize>(
//     rows: u32,
// ) -> Vec<[T; ATTRS]> {
//     let mut data: Vec<[T; ATTRS]> = Vec::new();

//     for _ in 0..=rows - 1 {
//         let mut row: [T; ATTRS] = [0.into(); ATTRS];
//         for i in 0..ATTRS {
//             row[i] = rand::random::<i32>().into();
//         }
//         data.push(row);
//     }

//     data
// }

fn bench_tables(c: &mut Criterion) {
    let mut group = c.benchmark_group("ColumnTable");

    let scalar_filters: ScalarFilters<i32, i32> = vec![
        Box::new(Equal::<i32>::new(0, 5)),
        Box::new(GreaterEqual::<i32>::new(1, 3)),
    ];
    let vector_filters: VectorFilters<__m512i, i32, __mmask16> = vec![
        Box::new(Equal::<i32>::new(0, 5)),
        Box::new(GreaterEqual::<i32>::new(1, 3)),
    ];

    let row_counts = [100_000, 1_000_000];

    for rows in row_counts.iter() {
        group.bench_with_input(
            BenchmarkId::new("ColumnTable Scalar", rows),
            rows,
            |b, rows| {
                let data = generate_data::<i32, 3>(rows.clone());
                let table = ColumnTable::new(data);

                b.iter(|| {
                    let indices = ScalarQuery::filter(&table, &scalar_filters);
                    return indices;
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("ColumnTable AVX", rows),
            rows,
            |b, rows| {
                let data = generate_data::<i32, 3>(rows.clone());
                let table = ColumnTable::new(data);

                b.iter(|| {
                    let indices = unsafe { VectorisedQuery::filter(&table, &vector_filters) };
                    return indices;
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("RowTable Scalar", rows),
            rows,
            |b, rows| {
                let data = generate_data::<i32, 3>(rows.clone());
                let table = RowTable::new(data);

                b.iter(|| {
                    let indices = ScalarQuery::filter(&table, &scalar_filters);
                    return indices;
                })
            },
        );
    }
}

// criterion_group! {
//     name = benches;
//     config = Criterion::default().warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(10)).sample_size(50).with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//     targets = bench_tables
// }
criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(5)).sample_size(50);
    targets = bench_tables
}
criterion_main!(benches);
