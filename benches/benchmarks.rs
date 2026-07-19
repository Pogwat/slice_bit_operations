
use slice_bit_operations::{SliceBitOps,MutSliceBitOps};
use criterion::{criterion_group, criterion_main, Criterion};

fn last_zero(c: &mut Criterion) {
    let fend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(!0).take(9000)).collect();
    c.bench_function("last_zero", |b|
        b.iter(|| fend.last_zero(0..) )
    );
}
fn first_one(c: &mut Criterion) {
    let zstar: Vec<u64> = core::iter::repeat(0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("first_one", |b|
        b.iter(|| zstar.first_one(0..) )
    );
}
fn first_zero(c: &mut Criterion) {
    let fstar: Vec<u64> = core::iter::repeat(!0).take(9000).chain((0..1000).rev()).collect();
    c.bench_function("first_zero", |b|
        b.iter(|| fstar.first_zero(0..) )
    );
}
fn last_one(c: &mut Criterion) {
    let zend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(0).take(9000)).collect();
    c.bench_function("last_one", |b|
        b.iter(|| zend.last_one(0..) )
    );
}

fn ctz(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("ctz", |b|
        b.iter(|| bal.ctz(0..) )
    );
}

fn popcnt(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("popcnt", |b|
        b.iter(|| bal.popcnt(0..) )
    );
}

fn bit_iter(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("bit_iter", |b|
        b.iter(|| {
            let mut set_bits=0;
            bal.bit_iter().for_each(|bit| {set_bits +=bit as usize;})
        })
    );
}

fn bit_iter_mut(c: &mut Criterion) {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    c.bench_function("bit_iter_mut", |b|
        b.iter(|| bal.clone().bit_iter_mut().for_each(|mut bit| *bit=false) )
    );
}

criterion_group!(firstlast, last_one,last_zero,first_one,first_zero);
criterion_group!(counters, ctz,popcnt);
criterion_group!(biters, bit_iter,bit_iter_mut);
criterion_main!(firstlast,counters,biters);
