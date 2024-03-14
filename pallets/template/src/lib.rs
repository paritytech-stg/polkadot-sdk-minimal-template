//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;
use frame_system;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}
