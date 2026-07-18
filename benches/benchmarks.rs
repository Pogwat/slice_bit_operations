use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use slice_bit_operations::{SliceBitOps,MutSliceBitOps};
fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<i32> = (0..1000).rev().collect();  // Reversed array of 1000 elements

    //last one is about 9000 elements from the end
    let zend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(0).take(9000)).collect();

    //4.4153 µs per iter
    c.bench_function("last_one", |b| {
        b.iter(|| zend.last_one(0..))
    });

    //last zero is about 9000 elements from the end
    let fend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(!0).take(9000)).collect();

    //6.2775 µs per iter
    c.bench_function("last_zero", |b| {
        b.iter(|| fend.last_zero(0..))
    });

    //first one is about 9k elements from start
    let zstar: Vec<u64> = core::iter::repeat(0).take(9000).chain((0..1000).rev()).collect();

    //3.2012 µs per iter
    c.bench_function("first_one", |b| {
        b.iter(|| zstar.first_one(0..))
    });

    //first zero is about 9k elements from start
    let fstar: Vec<u64> = core::iter::repeat(!0).take(9000).chain((0..1000).rev()).collect();

    //3.0670 µs per iter
    c.bench_function("first_zero", |b| {
        b.iter(|| fstar.first_zero(0..))
    });

    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    //2.3030 µs per iter
    c.bench_function("ctz", |b| {
        b.iter(|| bal.ctz(0..))
    });
    //1.9825 µs per iter
    c.bench_function("popcnt", |b| {
        b.iter(|| bal.popcnt(0..))
    });

     //765.92 µs per iter //28.833 µs
    c.bench_function("bit_iter_mut", |b| {
        b.iter(|| bal.clone().bit_iter_mut().for_each(|mut bit| *bit=false))
    });
    ;
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
