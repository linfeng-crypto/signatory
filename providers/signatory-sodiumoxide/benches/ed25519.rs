//! sodiumoxide provider benchmarks

#![allow(unused_imports)]
#![deny(warnings)]

#[macro_use]
extern crate criterion;
extern crate signatory;
extern crate signatory_sodiumoxide;

use criterion::Criterion;
use signatory::{
    ed25519::TEST_VECTORS, test_vector::TestVector, Ed25519PublicKey, Ed25519Seed,
    Ed25519Signature, Signature, Verifier,
};
use signatory_sodiumoxide::{Ed25519Signer, Ed25519Verifier};

/// Test vector to use for benchmarking
const TEST_VECTOR: &TestVector = &TEST_VECTORS[4];

fn sign_ed25519(c: &mut Criterion) {
    let signer = Ed25519Signer::from(&Ed25519Seed::from_bytes(TEST_VECTOR.sk).unwrap());

    c.bench_function("sodiumoxide: Ed25519 signer", move |b| {
        b.iter(|| signatory::sign(&signer, TEST_VECTOR.msg).unwrap())
    });
}

fn verify_ed25519(c: &mut Criterion) {
    let signature = Ed25519Signature::from_bytes(TEST_VECTOR.sig).unwrap();
    let verifier = Ed25519Verifier::from(&Ed25519PublicKey::from_bytes(TEST_VECTOR.pk).unwrap());

    c.bench_function("sodiumoxide: Ed25519 verifier", move |b| {
        b.iter(|| verifier.verify(TEST_VECTOR.msg, &signature).unwrap())
    });
}

criterion_group! {
    name = ed25519;
    config = Criterion::default();
    targets = sign_ed25519, verify_ed25519
}

criterion_main!(ed25519);
