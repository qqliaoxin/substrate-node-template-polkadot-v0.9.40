use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::{conn::OldKitty, VERSION};
pub type KittyId = u32;
pub type KittyDna = [u8; 16];
pub type KittyName = [u8; 4];

#[derive(
	Clone, PartialEq, Copy, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug,
)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
}

pub fn name_to_up(_v: &[u8; 2], _index: u32) -> KittyName {
	let mut result = [0; 4];
	// u32 to  u8
	let i: u8 = _index as u8;
	let index = [i];
	result[..2].copy_from_slice(_v);
	result[2..].copy_from_slice(&index);
	result
}

pub fn upgrade_v0<T: Config>(current_version: StorageVersion) -> Weight {
	if current_version != VERSION {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	// 旧kitty 存储的数据
	let items = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, items).drain()
	{
		let new_kitty = Kitty { name: name_to_up(b"v0", index), dna: kitty.0 };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
