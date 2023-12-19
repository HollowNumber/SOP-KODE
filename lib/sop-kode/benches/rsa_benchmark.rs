// std
use std::fs;

use num_bigint::{BigInt, BigUint};
use num_traits::FromPrimitive;

use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;

use sop_kode::rsa::*;

lazy_static! {
    static ref USER_1: RSA = RSA::new(2048).expect("Failed to create RSA");
    static ref USER_2: RSA = RSA::new(2048).expect("Failed to create RSA");
    static ref LONG_MESSAGE: String = fs::read_to_string("benches/long_message.txt").expect("Failed to read long message");
    static ref LONG_MESSAGE_9501: String = fs::read_to_string("benches/text files/file_9501.txt").expect("Failed to read long message");
    static ref ENCRYPTED_LONG_MESSAGE_9501: Vec<BigUint> = USER_1.encrypt_message(LONG_MESSAGE_9501.as_str(), USER_2.public_key.clone());
    static ref ENCRYPTED_LONG_MESSAGE: Vec<BigUint> = USER_1.encrypt_message(LONG_MESSAGE.as_str(), USER_2.public_key.clone());
    static ref ENCRYPTED_MESSAGE: Vec<BigUint> = USER_1.encrypt_message(MESSAGE, USER_2.public_key.clone());
}
const MESSAGE: &str = "This is a test message.";

fn generate_rsa_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_rsa");
    for bits in [64, 256, 512, 1024, 2048, 4096].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(bits),
            bits,
            |b, &bits| {
                b.iter(|| {
                    RSA::new(bits).unwrap();
                })
            },
        );
    }
    group.finish();
}


fn encrypt_message_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("encrypt_message");

    for messages in [MESSAGE, &LONG_MESSAGE_9501 ,&LONG_MESSAGE].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(messages.len()),
            messages,
            |b, &messages| {
                b.iter(|| {
                    USER_1.encrypt_message(&messages, USER_2.public_key.clone());
                })
            },
        );
    }
    group.finish();
}


fn decrypt_message_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("decrypt_message");
    for messages in [&*ENCRYPTED_MESSAGE,&*ENCRYPTED_LONG_MESSAGE_9501 ,&*ENCRYPTED_LONG_MESSAGE].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(messages.len()),
            messages,
            |b, &messages| {
                b.iter(|| {
                    USER_2.decrypt_message(messages.to_vec());
                })
            },
        );
    }
}

criterion_group! {
    name = rsa_bench;
    config = Criterion::default();
    targets = generate_rsa_bench,
        encrypt_message_bench,
        decrypt_message_bench
}

criterion_main!(rsa_bench);
