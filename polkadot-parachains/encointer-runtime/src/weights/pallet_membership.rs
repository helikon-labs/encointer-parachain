
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-fresh"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=encointer-rococo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(35_155_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((144_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(41_616_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((127_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(41_459_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((162_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(41_378_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((325_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:1)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(44_099_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((139_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership Members (r:1 w:0)
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(10_973_000 as Weight)
			// Standard Error: 0
			.saturating_add((103_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn clear_prime(m: u32, ) -> Weight {
		(3_660_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}