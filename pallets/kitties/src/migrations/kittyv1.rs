use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};
use frame_system::{pallet_prelude::*, Config};
pub use pallet::*;
#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	// 获取旧版本 Version
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	if on_chain_version != 0 && current_version != 1 {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	// 旧kitty 存储的数据
	let items = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, items).drain()
	{
		let new_kitty = Kitty { name: *b"kitty" + &index, dna: kitty.0 };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero();
}
