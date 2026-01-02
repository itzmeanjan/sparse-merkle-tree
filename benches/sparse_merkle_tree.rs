#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use fast_sparse_merkle_tree::{
    H256, Hash, blake3_hasher::Blake3Hasher, default_store::DefaultStore, tree::SparseMerkleTree, turboshake_hasher::TurboShake128Hasher,
};
use rand::Rng;
use std::time::Duration;

const TARGET_LEAVES_COUNT: usize = 32;
const NUM_LEAVES_IN_SMT: [usize; 3] = [1usize << 8, 1usize << 12, 1usize << 16];

type Blake3SMT = SparseMerkleTree<Blake3Hasher, Hash, H256, DefaultStore<Hash, H256, 32>, 32>;
type TurboShake128SMT = SparseMerkleTree<TurboShake128Hasher, Hash, H256, DefaultStore<Hash, H256, 32>, 32>;

fn random_h256<R: Rng + ?Sized>(rng: &mut R) -> H256 {
    rng.random::<[u8; std::mem::size_of::<H256>()]>().into()
}

fn random_blake3_smt<R: Rng + ?Sized>(update_count: usize, rng: &mut R) -> (Blake3SMT, Vec<Hash>) {
    let mut smt = Blake3SMT::default();
    let mut keys = Vec::with_capacity(update_count);

    for _ in 0..update_count {
        let key = random_h256(rng);
        let value = random_h256(rng);

        smt.update(key.into(), value).unwrap();
        keys.push(key.into());
    }

    (smt, keys)
}

fn random_turboshake128_smt<R: Rng + ?Sized>(update_count: usize, rng: &mut R) -> (TurboShake128SMT, Vec<Hash>) {
    let mut smt = TurboShake128SMT::default();
    let mut keys = Vec::with_capacity(update_count);

    for _ in 0..update_count {
        let key = random_h256(rng);
        let value = random_h256(rng);

        smt.update(key.into(), value).unwrap();
        keys.push(key.into());
    }

    (smt, keys)
}

fn bench_smt_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("smt_update");

    for size in NUM_LEAVES_IN_SMT.iter() {
        group.bench_with_input(format!("blake3/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            b.iter(|| random_blake3_smt(size, &mut rng));
        });

        group.bench_with_input(format!("turboshake128/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            b.iter(|| random_turboshake128_smt(size, &mut rng));
        });
    }

    group.finish();
}

fn bench_smt_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("smt_get");

    for size in NUM_LEAVES_IN_SMT.iter() {
        group.bench_with_input(format!("blake3/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_blake3_smt(size, &mut rng);

            b.iter(|| {
                let key = random_h256(&mut rng).into();
                smt.get(&key).unwrap();
            });
        });

        group.bench_with_input(format!("turboshake128/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_turboshake128_smt(size, &mut rng);

            b.iter(|| {
                let key = random_h256(&mut rng).into();
                smt.get(&key).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_smt_gen_merkle_proof(c: &mut Criterion) {
    let mut group = c.benchmark_group("smt_gen_merkle_proof");

    for size in NUM_LEAVES_IN_SMT.iter() {
        group.bench_with_input(format!("blake3/{}", size), size, |b, &size| {
            let mut rng = rand::rng();

            let (smt, mut keys) = random_blake3_smt(size, &mut rng);
            keys.dedup();

            let keys: Vec<_> = keys.into_iter().take(TARGET_LEAVES_COUNT).collect();
            b.iter(|| smt.merkle_proof(keys.clone()).unwrap());
        });

        group.bench_with_input(format!("turboshake128/{}", size), size, |b, &size| {
            let mut rng = rand::rng();

            let (smt, mut keys) = random_turboshake128_smt(size, &mut rng);
            keys.dedup();

            let keys: Vec<_> = keys.into_iter().take(TARGET_LEAVES_COUNT).collect();
            b.iter(|| smt.merkle_proof(keys.clone()).unwrap());
        });
    }

    group.finish();
}

fn bench_smt_verify_merkle_proof(c: &mut Criterion) {
    let mut group = c.benchmark_group("smt_verify_merkle_proof");

    for size in NUM_LEAVES_IN_SMT.iter() {
        group.bench_with_input(format!("blake3/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, mut keys) = random_blake3_smt(size, &mut rng);
            keys.dedup();

            let leaves: Vec<_> = keys.iter().take(TARGET_LEAVES_COUNT).map(|k| (*k, smt.get(k).unwrap())).collect();
            let proof = smt.merkle_proof(keys.into_iter().take(TARGET_LEAVES_COUNT).collect()).unwrap();

            let root = smt.root();
            b.iter(|| {
                assert!(
                    proof
                        .clone()
                        .verify::<Blake3Hasher, Hash, H256, 32>(root, leaves.clone())
                        .expect("must pass verification")
                );
            });
        });

        group.bench_with_input(format!("turboshake128/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, mut keys) = random_turboshake128_smt(size, &mut rng);
            keys.dedup();

            let leaves: Vec<_> = keys.iter().take(TARGET_LEAVES_COUNT).map(|k| (*k, smt.get(k).unwrap())).collect();
            let proof = smt.merkle_proof(keys.into_iter().take(TARGET_LEAVES_COUNT).collect()).unwrap();

            let root = smt.root();
            b.iter(|| {
                assert!(
                    proof
                        .clone()
                        .verify::<TurboShake128Hasher, Hash, H256, 32>(root, leaves.clone())
                        .expect("must pass verification")
                );
            });
        });
    }

    group.finish();
}

fn bench_smt_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("smt_validate");

    for size in NUM_LEAVES_IN_SMT.iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(format!("blake3/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_blake3_smt(size, &mut rng);

            b.iter(|| assert!(smt.validate()));
        });

        group.bench_with_input(format!("turboshake128/{}", size), size, |b, &size| {
            let mut rng = rand::rng();
            let (smt, _) = random_turboshake128_smt(size, &mut rng);

            b.iter(|| assert!(smt.validate()));
        });
    }

    group.finish();
}

criterion_group!(
    name = smt;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(60));
    targets = bench_smt_update, bench_smt_get, bench_smt_gen_merkle_proof, bench_smt_verify_merkle_proof, bench_smt_validate);
criterion_main!(smt);
