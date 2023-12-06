
use num_bigint::{BigInt, BigUint};
use num_traits::FromPrimitive;

use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;

use sop_kode::rsa::*;

lazy_static! {
    static ref KEYS: ((BigUint, BigUint), (BigUint, BigUint)) = generate_keys(1024).unwrap();
    static ref ENCRYPTED_MESSAGE: Vec<BigUint> = encrypt_message(MESSAGE, &KEYS.0.clone());
}

const MESSAGE: &str = "This is a test message.";

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
            mod_inverse(BigInt::from_i32(7).unwrap(), BigInt::from_i32(26).unwrap());
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


fn encrypt_message_bench(c: &mut Criterion) {
    c.bench_function("encrypt_message", |b| {
        b.iter(|| {
            let (public_key, _) = KEYS.clone();
            encrypt_message(MESSAGE, &public_key);
        })
    });
}

fn decrypt_message_bench(c: &mut Criterion) {
    c.bench_function("decrypt_message", |b| {
        b.iter(|| {
            let (_, private_key) = KEYS.clone();
            decrypt_message(ENCRYPTED_MESSAGE.clone(), private_key);
        })
    });
}

fn calculate_chunk_size_bench(c: &mut Criterion) {
    c.bench_function("calculate_chunk_size", |b| {
        b.iter(|| {
            let n = BigUint::from(33u64);
            calculate_chunk_size(&n);
        })
    });
}

fn generate_keys_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_keys");
    for bits in [32, 64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(bits),
            bits,
            |b, &bits| {
                b.iter(|| generate_keys(bits));
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = rsa_bench;
    config = Criterion::default();
    targets = miller_rabin_returns_true_for_prime_number,
    miller_rabin_returns_false_for_composite_number,
        generate_prime_bench,
        mod_inverse_bench,
        base_n_to_base10_bench,
        encrypt_message_bench,
        decrypt_message_bench,
        calculate_chunk_size_bench
}

criterion_main!(rsa_bench);
