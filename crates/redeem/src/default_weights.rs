//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn request_redeem() -> Weight;
    fn liquidation_redeem() -> Weight;
    fn execute_redeem() -> Weight;
    fn cancel_redeem_reimburse() -> Weight;
    fn cancel_redeem_retry() -> Weight;
    fn set_redeem_period() -> Weight;
    fn mint_tokens_for_reimbursed_redeem() -> Weight;
}

impl crate::WeightInfo for () {
    fn request_redeem() -> Weight {
        179_175_000_u64
            .saturating_add(DbWeight::get().reads(12_u64))
            .saturating_add(DbWeight::get().writes(5_u64))
    }
    fn liquidation_redeem() -> Weight {
        179_175_000_u64
            .saturating_add(DbWeight::get().reads(12_u64))
            .saturating_add(DbWeight::get().writes(5_u64))
    }
    fn execute_redeem() -> Weight {
        188_681_000_u64
            .saturating_add(DbWeight::get().reads(14_u64))
            .saturating_add(DbWeight::get().writes(4_u64))
    }
    fn cancel_redeem_reimburse() -> Weight {
        168_952_000_u64
            .saturating_add(DbWeight::get().reads(14_u64))
            .saturating_add(DbWeight::get().writes(5_u64))
    }
    fn cancel_redeem_retry() -> Weight {
        168_952_000_u64
            .saturating_add(DbWeight::get().reads(14_u64))
            .saturating_add(DbWeight::get().writes(5_u64))
    }
    fn set_redeem_period() -> Weight {
        3_376_000_u64.saturating_add(DbWeight::get().writes(1_u64))
    }
    // note: placeholder value
    fn mint_tokens_for_reimbursed_redeem() -> Weight {
        168_952_000_u64
            .saturating_add(DbWeight::get().reads(14_u64))
            .saturating_add(DbWeight::get().writes(5_u64))
    }
}
