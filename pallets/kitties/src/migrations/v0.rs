use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::{conn::OldKitty, VERSION};
pub type KittyId = u32;
pub type KittyDna = [u8; 16];
pub type KittyName = [u8; 13];

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
		// let mut _name = [0; 13];
		// _name[..12].copy_from_slice(b"v0_ketty_id_");
		// _name[13] = index as u8;
		let _name = *b"v0_ketty_id_0";
		let new_kitty = Kitty { name: _name, dna: kitty.0 };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
