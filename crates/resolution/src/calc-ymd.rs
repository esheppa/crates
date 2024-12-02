use chrono::{Datelike, Days, NaiveDate};
use criterion::{criterion_group, criterion_main, Criterion};
use resolution::Day;
use std::hint::black_box;

const CHRONO_BASE: NaiveDate = match NaiveDate::from_ymd_opt(1900, 1, 1) {
    Some(d) => d,
    None => panic!(""),
};

fn date_impl_narrow_benchmark(c: &mut Criterion) {
    c.bench_function("date-impl-narrow", |b| {
        b.iter(|| {
            for i in 36524..58439_i32 {
                let date = Day::new(black_box(i));
                _ = date.year();
                _ = date.month();
                _ = date.day();
            }
        })
    });
}

fn chrono_narrow_benchmark(c: &mut Criterion) {
    c.bench_function("chrono-narrow", |b| {
        b.iter(|| {
            for i in 36524..58439_u32 {
                let chrono_adj = CHRONO_BASE + Days::new(black_box(i) as u64);
                _ = chrono_adj.year();
                _ = chrono_adj.month();
                _ = chrono_adj.day();
            }
        })
    });
}

fn date_impl_pre_benchmark(c: &mut Criterion) {
    c.bench_function("date-impl-pre", |b| {
        b.iter(|| {
            let date = Day::new(black_box(11255));
            _ = date.year();
            _ = date.month();
            _ = date.day();
        })
    });
}

fn date_impl_1_benchmark(c: &mut Criterion) {
    c.bench_function("date-impl-1", |b| {
        b.iter(|| {
            let date = Day::new(black_box(36557));
            _ = date.year();
            _ = date.month();
            _ = date.day();
        })
    });
}

fn chrono_pre_benchmark(c: &mut Criterion) {
    c.bench_function("chrono-pre", |b| {
        b.iter(|| {
            let chrono_adj = CHRONO_BASE + Days::new(black_box(11255));
            _ = chrono_adj.year();
            _ = chrono_adj.month();
            _ = chrono_adj.day();
        })
    });
}

fn chrono_1_benchmark(c: &mut Criterion) {
    c.bench_function("chrono-1", |b| {
        b.iter(|| {
            let chrono_adj = CHRONO_BASE + Days::new(black_box(36557));
            _ = chrono_adj.year();
            _ = chrono_adj.month();
            _ = chrono_adj.day();
        })
    });
}

criterion_group!(
    benches,
    chrono_1_benchmark,
    date_impl_1_benchmark,
    chrono_pre_benchmark,
    date_impl_pre_benchmark,
    date_impl_narrow_benchmark,
    chrono_narrow_benchmark
);
criterion_main!(benches);
