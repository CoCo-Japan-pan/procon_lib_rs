use bitvec::BitVec;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_pcg::Pcg32;

pub fn rank1(c: &mut Criterion) {
    let mut rng = Pcg32::seed_from_u64(0);
    const SIZE: usize = 100000;
    let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();
    let bit_vec = BitVec::from(&bool_vec[..]);
    c.bench_function("rank1_all", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                black_box(bit_vec.rank1(i));
            }
        });
    });
}

pub fn rank0(c: &mut Criterion) {
    let mut rng = Pcg32::seed_from_u64(0);
    const SIZE: usize = 100000;
    let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();
    let bit_vec = BitVec::from(&bool_vec[..]);
    c.bench_function("rank0_all", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                black_box(bit_vec.rank0(i));
            }
        });
    });
}

pub fn select1(c: &mut Criterion) {
    let mut rng = Pcg32::seed_from_u64(0);
    const SIZE: usize = 100000;
    let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();
    let bit_vec = BitVec::from(&bool_vec[..]);
    c.bench_function("select1_all", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                black_box(bit_vec.select1(i));
            }
        });
    });
}

pub fn select0(c: &mut Criterion) {
    let mut rng = Pcg32::seed_from_u64(0);
    const SIZE: usize = 100000;
    let bool_vec: Vec<bool> = (0..SIZE).map(|_| rng.gen()).collect();
    let bit_vec = BitVec::from(&bool_vec[..]);
    c.bench_function("select0_all", |b| {
        b.iter(|| {
            for i in 0..SIZE {
                black_box(bit_vec.select0(i));
            }
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = rank1, rank0, select1, select0
}
criterion_main!(benches);
