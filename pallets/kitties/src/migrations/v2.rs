use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::VERSION;

pub type KittyId = u32;
pub type KittyDna = [u8; 16];
pub type KittyName = [u8; 13];

#[derive(Clone, PartialEq, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
}

pub fn upgrade_v2<T: Config>(current_version: StorageVersion) -> Weight {
	if current_version != VERSION {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	// 旧kitty 存储的数据
	let items = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, Kitty, Blake2_128Concat>(module, items).drain()
	{
		// let mut _name = kitty.name;
		// _name[..1].copy_from_slice(b"v2");
		// _name[12] = index as u8;
		let _name = *b"v2_ketty_id_2";
		let new_kitty = Kitty { name: _name, dna: kitty.dna };
		// Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
