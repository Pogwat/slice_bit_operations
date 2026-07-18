use std::hint::black_box;
use slice_bit_operations::{SliceBitOps,MutSliceBitOps};

fn main() {divan::main();}

#[divan::bench]
fn last_one() {
    let zend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(0).take(9000)).collect();
    zend.last_one(0..);
}

#[divan::bench]
fn last_zero() {
    let fend: Vec<u64> = (0..1000).rev().chain(core::iter::repeat(!0).take(9000)).collect();
    fend.last_zero(0..);
}

#[divan::bench]
fn first_one() {
    let zstar: Vec<u64> = core::iter::repeat(0).take(9000).chain((0..1000).rev()).collect();
    zstar.first_one(0..);
}

#[divan::bench]
fn first_zero() {
    let fstar: Vec<u64> = core::iter::repeat(!0).take(9000).chain((0..1000).rev()).collect();
    fstar.first_zero(0..);
}

#[divan::bench]
fn ctz() {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    bal.ctz(0..);
}

#[divan::bench]
fn popcnt() {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    bal.popcnt(0..);
}

#[divan::bench]
fn bit_iter() {
    let bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    let mut set_bits=0;
    bal.clone().bit_iter().for_each(|bit| {set_bits +=bit as usize;})
}

#[divan::bench]
fn bit_iter_mut() {
    let mut bal: Vec<u64> = core::iter::repeat(0).take(5000).chain(core::iter::repeat(!0).take(5000)).collect();
    bal.bit_iter_mut().for_each(|mut bit| *bit=false)
}
