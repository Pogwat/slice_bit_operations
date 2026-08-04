use slice_bit_operations::{SliceBitOps,MutSliceBitOps};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn last_zero(c: &mut Criterion) {
    let fend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(!0).take(9000)).collect();
    c.bench_function("last_zero", |b|
        b.iter(|| fend.bit_iter().last_zero() )
    );
}
fn first_one(c: &mut Criterion) {
    let zstar: Vec<u64> = core::iter::repeat(0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("first_one", |b|
        b.iter(|| zstar.bit_iter().first_one() )
    );
}
fn first_zero(c: &mut Criterion) {
    let fstar: Vec<u64> = core::iter::repeat(!0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("first_zero", |b|
        b.iter(|| fstar.bit_iter().first_zero() )
    );
}
fn last_one(c: &mut Criterion) {
    let zend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(0).take(9000)).collect();
    c.bench_function("last_one", |b|
        b.iter(|| zend.bit_iter().last_one() )
    );
}

fn biter_popcnt(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("biter_popcnt", |b|
        b.iter(|| bal.bit_iter().popcnt() )
    );
}

fn biter_ctz(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("biter_ctz", |b|
        b.iter(|| bal.bit_iter().ctz() )
    );
}

fn biter_first_zero(c: &mut Criterion) {
    let fstar: Vec<u64> = core::iter::repeat(!0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("biter_first_zero", |b|
        b.iter(|| fstar.bit_iter().first_zero() )
    );
}
fn biter_first_one(c: &mut Criterion) {
    let zstar: Vec<u64> = core::iter::repeat(0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("biter_first_one", |b|
        b.iter(|| zstar.bit_iter().first_one() )
    );
}


fn bit_iter(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("bit_iter", |b|
        b.iter(|| {
            let mut set_bits=0;
            bal.bit_iter().for_each(|bit| {set_bits +=bit as usize;});
            black_box(set_bits);
        })
    );
}

fn bit_iter_mut(c: &mut Criterion) {
    let mut bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("bit_iter_mut", |b|
        b.iter(|| {
            bal.bit_iter_mut().for_each(|mut bit| *bit=false);
            black_box(&bal);
        })
    );
}

criterion_group!(firstlast, last_one,last_zero,first_one,first_zero);
criterion_group!(biterfirst, biter_first_one,biter_first_zero);
criterion_group!(bitercounters, biter_ctz,biter_popcnt);
criterion_group!(biters, bit_iter,bit_iter_mut);
criterion_main!(firstlast,biters,bitercounters,biterfirst);
