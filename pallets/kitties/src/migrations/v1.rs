use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::{v0, VERSION};
pub type KittyId = v0::KittyId;
pub type KittyDna = v0::KittyDna;
pub type KittyName = [u8; 8];

#[derive(Clone, PartialEq, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
}
pub fn name_to_up(_v: &[u8; 3], _index: u32, _name: v0::KittyName) -> KittyName {
	let mut result = [0; 8];
	// u32 to  u8
	let i: u8 = _index as u8;
	let index = [i];
	result[..3].copy_from_slice(_v);
	result[3..4].copy_from_slice(&index);
	result[4..].copy_from_slice(&_name);
	result
}
pub fn upgrade_v1<T: Config>(current_version: StorageVersion) -> Weight {
	if current_version != VERSION {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	// 旧kitty 存储的数据
	let items = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, v0::Kitty, Blake2_128Concat>(module, items).drain()
	{
		let new_kitty = Kitty { name: name_to_up(b"v1_", index, kitty.name), dna: kitty.dna };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
