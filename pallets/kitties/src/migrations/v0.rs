use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::{conn::OldKitty, VERSION};
pub type KittyId = u32;
pub type KittyDna = [u8; 16];
pub type KittyName = [u8; 0];

#[derive(Clone, PartialEq, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
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
		let i: u8 = index as u8;
		// let array = [0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, i];
		let _n = *b"kettyV0";
		let new_kitty = Kitty { name: [u8; 0], dna: kitty.0 };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
