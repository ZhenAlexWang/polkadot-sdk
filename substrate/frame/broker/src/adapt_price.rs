// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(missing_docs)]

use crate::CoreIndex;
use sp_arithmetic::{traits::One, FixedU64};
use sp_runtime::Saturating;

/// Type for determining how to set price.
pub trait AdaptPrice {
	/// Return the factor by which the regular price must be multiplied during the leadin period.
	///
	/// - `when`: The amount through the leadin period; between zero and one.
	fn leadin_factor_at(when: FixedU64) -> FixedU64;
	/// Return the correction factor by which the regular price must be multiplied based on market
	/// performance.
	///
	/// - `sold`: The number of cores sold.
	/// - `target`: The target number of cores to be sold (must be larger than zero).
	/// - `limit`: The maximum number of cores to be sold.
	fn adapt_price(sold: CoreIndex, target: CoreIndex, limit: CoreIndex) -> FixedU64;
}

impl AdaptPrice for () {
	fn leadin_factor_at(_: FixedU64) -> FixedU64 {
		FixedU64::one()
	}
	fn adapt_price(_: CoreIndex, _: CoreIndex, _: CoreIndex) -> FixedU64 {
		FixedU64::one()
	}
}

/// Simple implementation of `AdaptPrice` giving a monotonic leadin and a linear price change based
/// on cores sold.
pub struct Linear;
impl AdaptPrice for Linear {
	fn leadin_factor_at(when: FixedU64) -> FixedU64 {
		FixedU64::from(2).saturating_sub(when)
	}
	fn adapt_price(sold: CoreIndex, target: CoreIndex, limit: CoreIndex) -> FixedU64 {
		if sold <= target {
			// Range of [0.5, 1.0].
			FixedU64::from_rational(1, 2).saturating_add(FixedU64::from_rational(
				sold.into(),
				target.saturating_mul(2).into(),
			))
		} else {
			// Range of (1.0, 2].

			// Unchecked math: In this branch we know that sold > target. The limit must be >= sold
			// by construction, and thus target must be < limit.
			FixedU64::one().saturating_add(FixedU64::from_rational(
				(sold - target).into(),
				(limit - target).into(),
			))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn linear_no_panic() {
		for limit in 0..10 {
			for target in 1..10 {
				for sold in 0..=limit {
					let price = Linear::adapt_price(sold, target, limit);

					if sold > target {
						assert!(price > FixedU64::one());
					} else {
						assert!(price <= FixedU64::one());
					}
				}
			}
		}
	}

	#[test]
	fn linear_bound_check() {
		// Using constraints from pallet implementation i.e. `limit >= sold`.
		// Check extremes
		let limit = 10;
		let target = 5;

		// Maximally sold: `sold == limit`
		assert_eq!(Linear::adapt_price(limit, target, limit), FixedU64::from_float(2.0));
		// Ideally sold: `sold == target`
		assert_eq!(Linear::adapt_price(target, target, limit), FixedU64::one());
		// Minimally sold: `sold == 0`
		assert_eq!(Linear::adapt_price(0, target, limit), FixedU64::from_float(0.5));
		// Optimistic target: `target == limit`
		assert_eq!(Linear::adapt_price(limit, limit, limit), FixedU64::one());
		// Pessimistic target: `target == 0`
		assert_eq!(Linear::adapt_price(limit, 0, limit), FixedU64::from_float(2.0));
	}
}
