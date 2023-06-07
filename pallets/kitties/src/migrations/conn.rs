use crate::{Config, Pallet};
use frame_support::{pallet_prelude::*, traits::GetStorageVersion, weights::Weight};

use super::{
	v0::upgrade_v0,
	v1::{self, upgrade_v1},
	v2::{self, upgrade_v2},
	VERSION,
};

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct OldKitty(pub [u8; 16]);

fn name_to_up(name_v1: &v1::KittyName, append: &[u8; 4]) -> v2::KittyName {
	let mut result = [0; 8];
	result[..4].copy_from_slice(name_v1);
	result[4..].copy_from_slice(append);
	result
}

pub fn migrate<T: Config>() -> Weight {
	// 链版本号
	let chain_version = Pallet::<T>::on_chain_storage_version();
	if chain_version != 0 {
		return Weight::zero()
	}

	// 获取当前版本 Version
	let current_version = Pallet::<T>::current_storage_version();

	// 输入要升级的版本
	match VERSION {
		1 => upgrade_v0::<T>(current_version),
		2 => upgrade_v1::<T>(current_version),
		3 => upgrade_v2::<T>(current_version),
		_ => return Weight::zero(),
	}
}
