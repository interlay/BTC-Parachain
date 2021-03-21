//! Autogenerated weights for btc_relay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-23, STEPS: [100, ], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ../target/release/btc-parachain
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// btc_relay
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// ../crates/btc-relay/src/weights.rs
// --template
// ../.deploy/weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for btc_relay.
pub trait WeightInfo {
    fn verify_and_validate_transaction() -> Weight;
    fn verify_transaction_inclusion() -> Weight;
    fn validate_transaction() -> Weight;
}

/// Weights for btc_relay using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn verify_and_validate_transaction() -> Weight {
        (99_474_000 as Weight).saturating_add(T::DbWeight::get().reads(9 as Weight))
    }
    fn verify_transaction_inclusion() -> Weight {
        (55_622_000 as Weight).saturating_add(T::DbWeight::get().reads(8 as Weight))
    }
    fn validate_transaction() -> Weight {
        (15_739_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn verify_and_validate_transaction() -> Weight {
        (99_474_000 as Weight).saturating_add(RocksDbWeight::get().reads(9 as Weight))
    }
    fn verify_transaction_inclusion() -> Weight {
        (55_622_000 as Weight).saturating_add(RocksDbWeight::get().reads(8 as Weight))
    }
    fn validate_transaction() -> Weight {
        (15_739_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
    }
}
