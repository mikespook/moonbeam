// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("moonbase-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 8_943_000 picoseconds.
		Weight::from_parts(9_593_622, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 1_459
			.saturating_add(Weight::from_parts(99_156, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + r * (5 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 32_324_000 picoseconds.
		Weight::from_parts(30_424_080, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_785
			.saturating_add(Weight::from_parts(150_588, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89`
		//  Estimated: `10991 + s * (2565 ±0)`
		// Minimum execution time: 9_515_000 picoseconds.
		Weight::from_parts(23_106_874, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 4_246
			.saturating_add(Weight::from_parts(3_589_907, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2565).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169 + p * (20 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 9_420_000 picoseconds.
		Weight::from_parts(23_856_903, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_447
			.saturating_add(Weight::from_parts(1_497_213, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445 + r * (5 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 59_657_000 picoseconds.
		Weight::from_parts(29_331_865, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 7_077
			.saturating_add(Weight::from_parts(275_001, 0).saturating_mul(r.into()))
			// Standard Error: 1_382
			.saturating_add(Weight::from_parts(1_492_955, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `355 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 32_035_000 picoseconds.
		Weight::from_parts(31_277_555, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_421
			.saturating_add(Weight::from_parts(109_577, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `386 + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 29_698_000 picoseconds.
		Weight::from_parts(28_787_784, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 3_232
			.saturating_add(Weight::from_parts(84_735, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 6_433_000 picoseconds.
		Weight::from_parts(6_763_771, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 1_060
			.saturating_add(Weight::from_parts(76_031, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 6_530_000 picoseconds.
		Weight::from_parts(6_764_139, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 829
			.saturating_add(Weight::from_parts(75_980, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 6_422_000 picoseconds.
		Weight::from_parts(6_845_806, 0)
			.saturating_add(Weight::from_parts(0, 2386))
			// Standard Error: 913
			.saturating_add(Weight::from_parts(69_740, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 21_684_000 picoseconds.
		Weight::from_parts(21_175_027, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 4_037
			.saturating_add(Weight::from_parts(77_912, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `712 + r * (5 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 82_142_000 picoseconds.
		Weight::from_parts(52_763_268, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 7_326
			.saturating_add(Weight::from_parts(283_613, 0).saturating_mul(r.into()))
			// Standard Error: 1_430
			.saturating_add(Weight::from_parts(1_493_205, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `536 + s * (23 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 28_438_000 picoseconds.
		Weight::from_parts(32_896_151, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_176
			.saturating_add(Weight::from_parts(74_450, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `615 + s * (4 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 13_006_000 picoseconds.
		Weight::from_parts(16_462_133, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_374
			.saturating_add(Weight::from_parts(58_282, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647 + s * (24 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 31_845_000 picoseconds.
		Weight::from_parts(36_646_202, 0)
			.saturating_add(Weight::from_parts(0, 10991))
			// Standard Error: 1_281
			.saturating_add(Weight::from_parts(89_740, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(2046), added: 4521, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `620 + s * (24 ±0)`
		//  Estimated: `5511`
		// Minimum execution time: 23_322_000 picoseconds.
		Weight::from_parts(25_422_652, 0)
			.saturating_add(Weight::from_parts(0, 5511))
			// Standard Error: 1_320
			.saturating_add(Weight::from_parts(111_345, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:0 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3505`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3505))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7560), added: 10035, mode: `MaxEncodedLen`)
	fn set_username_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `11025`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(23_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11025))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7560), added: 10035, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:0 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `11025`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11025))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn remove_expired_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `3538`
		// Minimum execution time: 27_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3538))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:0)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7560), added: 10035, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `11025`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11025))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7560), added: 10035, mode: `MaxEncodedLen`)
	fn remove_dangling_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `11025`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11025))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}