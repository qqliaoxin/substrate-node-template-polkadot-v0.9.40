use crate::{Config, Pallet};
use frame_support::{pallet_prelude::*, traits::GetStorageVersion, weights::Weight};

use super::{v0::upgrade_v0, v1::upgrade_v1, v2::upgrade_v2, VERSION};
// use super::{v0::upgrade_v0, VERSION};

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	// 链版本号
	let chain_version = Pallet::<T>::on_chain_storage_version();
	if chain_version != 0 {
		return Weight::zero()
	}

	// 获取当前版本 Version
	let current_version = Pallet::<T>::current_storage_version();

	// VERSION.put::<Pallet>();

	// 输入要升级的版本
	match VERSION {
		1 => upgrade_v0::<T>(current_version),
		2 => upgrade_v1::<T>(current_version),
		5 => upgrade_v2::<T>(current_version),
		_ => return Weight::zero(),
	}
}

// 自动推导类型 可将以0填充buf空间
// let mut _name = [0_u8; 32];
// _name[..10].copy_from_slice(&kitty.name[..10]);
// _name[10] = index as u8;
// _name[10..14].copy_from_slice(b"_v5_");
// let new_kitty = Kitty { name: _name, dna: kitty.dna };
// let new_kitty = Kitty { name: *b"ketty_id_00000000000000000000000", dna: kitty.dna };
