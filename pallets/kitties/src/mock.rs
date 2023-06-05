use crate as pallet_kitties;
use frame_support::{
	traits::{ConstU16, ConstU64},
	PalletId,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use pallet_insecure_randomness_collective_flip;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		KittiesModule: pallet_kitties,
		Randomness: pallet_insecure_randomness_collective_flip,
		// Balances: pallet_balances::{Pallet, Call, RuntimeEvent<T>, Storage},
		Balances: pallet_balances,
	}
);
// #[warn(unused_imports)]
frame_support::parameter_types! {
	pub const SContractPalletId: PalletId = PalletId(*b"py/kitty");
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	// type MaxClaimLength = ConstU32<10>;
	type Randomness = Randomness;
	type Currency = Balances;
	type KittyPrice = ConstU64<256>;
	type PalletId = SContractPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<256>;
	type AccountStore = System;
	type WeightInfo = ();
	// type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

// Build genesis storage according to the mock runtime.
//  模拟链上内存存储
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into(); // 创世初始化
	ext.execute_with(|| System::set_block_number(1));
	ext
}
