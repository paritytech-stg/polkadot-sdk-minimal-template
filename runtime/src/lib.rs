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

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use frame_system;
use frame_support;
use frame_support::sp_runtime;

use frame_support::{
	genesis_builder_helper::{build_config, create_default_config},
	weights::{FixedFee, NoFee}
};

use frame_system::pallet_prelude::*;
use frame_support::pallet_prelude::*;
use frame_system_rpc_runtime_api;

use sp_core;
use sp_core::*;

use sp_std::prelude::*;
use sp_runtime::generic;
use sp_runtime::OpaqueExtrinsic;

use sp_runtime::traits;

pub use frame_executive::*;

/// Macro to amalgamate the runtime into `struct Runtime`.
pub use frame_support::construct_runtime;

/// Macro to easily derive the `Config` trait of various pallet for `Runtime`.
pub use frame_support::derive_impl;

/// Macros to easily impl traits such as `Get` for types.
// TODO: using linking in the Get in the line above triggers an ICE :/
pub use frame_support::{ord_parameter_types, parameter_types};

/// Const types that can easily be used in conjuncture with `Get`.
pub use frame_support::traits::{
	ConstBool, ConstI128, ConstI16, ConstI32, ConstI64, ConstI8, ConstU128, ConstU16,
	ConstU32, ConstU64, ConstU8,
};

/// Primary types used to parameterize `EnsureOrigin` and `EnsureRootWithArg`.
pub use frame_system::{
	EnsureNever, EnsureNone, EnsureRoot, EnsureRootWithSuccess, EnsureSigned,
	EnsureSignedBy,
};

/// Types to define your runtime version.
pub use sp_version::{create_runtime_str, runtime_version, RuntimeVersion};

/// Macro to implement runtime APIs.
pub use sp_api::impl_runtime_apis;

#[cfg(feature = "std")]
pub use sp_version::NativeVersion;

pub use sp_core::OpaqueMetadata;
pub use sp_inherents::{CheckInherentsResult, InherentData};
pub use sp_runtime::{ApplyExtrinsicResult, ExtrinsicInclusionMode};

pub use frame_system_rpc_runtime_api::*;
pub use sp_api::{self, *};
pub use sp_block_builder::*;


mod apis {
	pub use frame_system_rpc_runtime_api::*;
	pub use sp_api::{self, *};
	pub use sp_block_builder::*;
	pub use sp_offchain::*;
	pub use sp_session::runtime_api::*;
	pub use sp_transaction_pool::runtime_api::*;
}

use apis::*;

#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("minimal-template-runtime"),
	impl_name: create_runtime_str!("minimal-template-runtime"),
	authoring_version: 1,
	spec_version: 0,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Timestamp: pallet_timestamp,

		Balances: pallet_balances,
		Sudo: pallet_sudo,
		TransactionPayment: pallet_transaction_payment,

		// our local pallet
		Template: pallet_minimal_template,
	}
);

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type Version = Version;
	type BlockHashCount = ConstU32<1024>;
	type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Runtime {
	type AccountStore = System;
}

#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig as pallet_sudo::DefaultConfig)]
impl pallet_sudo::Config for Runtime {}

#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig as pallet_timestamp::DefaultConfig)]
impl pallet_timestamp::Config for Runtime {}

#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig as pallet_transaction_payment::DefaultConfig)]
impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
	type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
	type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;
}

impl pallet_minimal_template::Config for Runtime {}

use frame_system::Config as SysConfig;
type Signature = sp_runtime::MultiSignature;
type AccountId = <<Signature as traits::Verify>::Signer as traits::IdentifyAccount>::AccountId;
type ExtrinsicInner<T, Extra, AccountIndex = ()> = generic::UncheckedExtrinsic<
	sp_runtime::MultiAddress<AccountId, AccountIndex>,
	<T as SysConfig>::RuntimeCall,
	Signature,
	Extra,
>;
type BlockNumber = u32;
pub type OpaqueBlock = generic::Block<HeaderInner, OpaqueExtrinsic>;
type HeaderInner = generic::Header<BlockNumber, traits::BlakeTwo256>;
type BlockOf<T, Extra = ()> = generic::Block<HeaderInner, ExtrinsicInner<T, Extra>>;
type Block = BlockOf<Runtime, SignedExtra>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
	Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};

impl_runtime_apis! {
	impl apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			RuntimeExecutive::execute_block(block)
		}

		fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
			RuntimeExecutive::initialize_block(header)
		}
	}
	impl apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
			RuntimeExecutive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> HeaderFor<Runtime> {
			RuntimeExecutive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicFor<Runtime>,
			block_hash: <Runtime as frame_system::Config>::Hash,
		) -> TransactionValidity {
			RuntimeExecutive::validate_transaction(source, tx, block_hash)
		}
	}

	impl apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderFor<Runtime>) {
			RuntimeExecutive::offchain_worker(header)
		}
	}

	impl apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
			Default::default()
		}
	}

	impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
		fn account_nonce(account: interface::AccountId) -> interface::Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		interface::Balance,
	> for Runtime {
		fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> interface::Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> interface::Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
		fn create_default_config() -> Vec<u8> {
			create_default_config::<RuntimeGenesisConfig>()
		}

		fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
			build_config::<RuntimeGenesisConfig>(config)
		}
	}
}

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
	use super::Runtime;
	use frame_system;

	pub type Block = super::Block;
	pub type OpaqueBlock = super::OpaqueBlock;
	pub type AccountId = <Runtime as frame_system::Config>::AccountId;
	pub type Nonce = <Runtime as frame_system::Config>::Nonce;
	pub type Hash = <Runtime as frame_system::Config>::Hash;
	pub type Balance = <Runtime as pallet_balances::Config>::Balance;
	pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
