// Copyright 2022 Webb Technologies Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use pallet_evm_precompile_batch::BatchPrecompile;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_call_permit::CallPermitPrecompile;
use pallet_evm_precompile_democracy::DemocracyPrecompile;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_preimage::PreimagePrecompile;
use pallet_evm_precompile_registry::PrecompileRegistry;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evm_precompile_staking::StakingPrecompile;
use precompile_utils::precompile_set::{
	AcceptDelegateCall, AddressU64, CallableByContract, CallableByPrecompile, OnlyFrom,
	PrecompileAt, PrecompileSetBuilder, SubcallWithMaxNesting,
};

type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);

#[precompile_utils::precompile_name_from_address]
pub type WebbPrecompilesAt<R> = (
	// Ethereum precompiles:
	PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<6>, Bn128Add, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<7>, Bn128Mul, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<8>, Bn128Pairing, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<9>, Blake2F, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<1024>, Sha3FIPS256, (CallableByContract, CallableByPrecompile)>,
	PrecompileAt<AddressU64<1026>, ECRecoverPublicKey, (CallableByContract, CallableByPrecompile)>,
	PrecompileAt<
		AddressU64<2048>,
		StakingPrecompile<R>,
		(CallableByContract, CallableByPrecompile),
	>,
	// Moonbeam precompiles
	PrecompileAt<
		AddressU64<2051>,
		DemocracyPrecompile<R>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<2056>,
		BatchPrecompile<R>,
		(
			SubcallWithMaxNesting<2>,
			// Batch is the only precompile allowed to call Batch.
			CallableByPrecompile<OnlyFrom<AddressU64<2056>>>,
		),
	>,
	PrecompileAt<
		AddressU64<2058>,
		CallPermitPrecompile<R>,
		(SubcallWithMaxNesting<0>, CallableByContract),
	>,
	PrecompileAt<
		AddressU64<2067>,
		PreimagePrecompile<R>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<2069>,
		PrecompileRegistry<R>,
		(CallableByContract, CallableByPrecompile),
	>,
);

pub type WebbPrecompiles<R> = PrecompileSetBuilder<R, WebbPrecompilesAt<R>>;
