use num_bigint::BigUint;

use criterion::{criterion_group, criterion_main, Criterion};
use sop_kode::rsa::*;

fn miller_rabin_returns_true_for_prime_number(c: &mut Criterion) {
    c.bench_function("miller_rabin prime", |b| {
        b.iter(|| {
            let prime = BigUint::from(13u64);
            assert!(miller_rabin(&prime, 5));
        })
    });
}

fn miller_rabin_returns_false_for_composite_number(c: &mut Criterion) {
    c.bench_function("miller_rabin composite", |b| {
        b.iter(|| {
            let composite = BigUint::from(15u64);
            assert!(!miller_rabin(&composite, 5));
        })
    });
}

fn generate_prime_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_prime");
    for bits in [32, 64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(bits),
            bits,
            |b, &bits| {
                b.iter(|| generate_prime(bits));
            },
        );
    }
    group.finish();
}

fn mod_inverse_bench(c: &mut Criterion) {
    c.bench_function("mod_inverse", |b| {
        b.iter(|| {
            mod_inverse(7, 26);
        })
    });
}

fn base_n_to_base10_bench(c: &mut Criterion) {
    c.bench_function("base_n_to_base10", |b| {
        b.iter(|| {
            base_n_to_base10(&vec![1, 0, 1], 2);
        })
    });
}

fn encrypt_bench(c: &mut Criterion) {
    c.bench_function("encrypt", |b| {
        b.iter(|| {
            let message = BigUint::from(7u64);
            let public_key = (BigUint::from(33u64), BigUint::from(3u64));
            encrypt(message, public_key);
        })
    });
}

fn decrypt_bench(c: &mut Criterion) {
    c.bench_function("decrypt", |b| {
        b.iter(|| {
            let ciphertext = BigUint::from(13u64);
            let private_key = (BigUint::from(33u64), BigUint::from(7u64));
            decrypt(ciphertext, private_key);
        })
    });
}

criterion_group! {
    name = rsa_bench;
    config = Criterion::default();
    targets = miller_rabin_returns_true_for_prime_number,
    miller_rabin_returns_false_for_composite_number,
        generate_prime_bench,
        mod_inverse_bench,
        base_n_to_base10_bench,
        encrypt_bench,
        decrypt_bench
}

criterion_main!(rsa_bench);
