use super::*; // 引入模块的所用存储项

use crate::{mock::*, Error, Event, Kitty, KittyId, NextKittyId};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchResultWithPostInfo};

#[test]
fn test_module_works() {
	assert_eq!(1, 1);
}

// unit-testing Even Test Checks
// https://docs.substrate.io/test/unit-testing/

//构建三个address,开发链有一系列测试账户，其中前两个账户有一定数额的测试余额
const ALICE: AccountId = 1;
const BOB: AccountId = 2;
// const CHARLIE: AccountId = 3;
const KITTY_ID_0: KittyId = 0;

fn init_balance(account: AccountId, new_free: Balance) -> DispatchResultWithPostInfo {
	Balances::set_balance(RuntimeOrigin::root(), account, new_free, 0)
}

// 测试创建一个 Kitty Events
#[test]
fn created_checks_events() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);

		assert_ok!(init_balance(ALICE, 10_000_000));

		let signer = RuntimeOrigin::signed(ALICE);

		//获取当前kitty的id,创建时会默认将该id给新的kitty
		let kitty_id = NextKittyId::<Test>::get();
		//断言是否创建成功
		assert_ok!(KittiesModule::create(signer.clone()));
		assert_ne!(Kitties::<Test>::get(KITTY_ID_0), None);
		//测试event
		let kitty: Kitty = Kitties::<Test>::get(kitty_id).unwrap();

		System::assert_has_event(
			Event::KittyCreated { who: ALICE, kitty_id: KITTY_ID_0, kitty }.into(),
		);
		// assert_eq!(System::events().len(), 1);
	})
}

#[test]
fn create_kitties_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ALICE, 10_000_000));

		let signer = RuntimeOrigin::signed(ALICE);

		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID_0);
		assert_ok!(KittiesModule::create(signer.clone()));

		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID_0 + 1);
		assert_eq!(KittiesModule::kitties(KITTY_ID_0).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ALICE));
		assert_eq!(KittiesModule::kitty_parents(KITTY_ID_0), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());

		// 断言储存值 是否相等
		assert_noop!(KittiesModule::create(signer.clone()), Error::<Test>::InvalidKittyId);
		let kitty: Kitty = Kitties::<Test>::get(KITTY_ID_0).unwrap();

		System::assert_has_event(
			Event::KittyCreated { who: ALICE, kitty_id: KITTY_ID_0, kitty }.into(),
		);
	});
}

#[test]
fn breed_is_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ALICE, 10_000_000));

		let signer = RuntimeOrigin::signed(ALICE);

		let breed_kitty_id = KITTY_ID_0 + 2;

		// 一样的 parent kitty_id
		assert_noop!(
			KittiesModule::breed(signer.clone(), KITTY_ID_0, KITTY_ID_0),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(signer.clone(), KITTY_ID_0, KITTY_ID_0 + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(signer.clone()));
		assert_ok!(KittiesModule::create(signer.clone()));

		assert_eq!(KittiesModule::next_kitty_id(), KITTY_ID_0 + 2);

		assert_ok!(KittiesModule::breed(signer.clone(), KITTY_ID_0, KITTY_ID_0 + 1));

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(ALICE));
		assert_eq!(
			KittiesModule::kitty_parents(breed_kitty_id),
			Some((KITTY_ID_0, KITTY_ID_0 + 1))
		);

		let breed_kitty: Kitty = KittiesModule::kitties(breed_kitty_id).expect("Kitty Breed");
		System::assert_last_event(
			Event::KittyBreed { who: ALICE, kitty_id: breed_kitty_id, kitty: breed_kitty }.into(),
		);
	});
}

#[test]
fn transfer_is_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ALICE, 10_000_000));

		let signer = RuntimeOrigin::signed(ALICE);

		assert_ok!(KittiesModule::create(signer.clone()));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ALICE));

		assert_ok!(KittiesModule::transfer(signer.clone(), BOB, KITTY_ID_0));

		System::assert_last_event(
			Event::KittyTransfered { who: ALICE, kitty_id: KITTY_ID_0, recipient: BOB }.into(),
		);
	});
}

#[test]
fn sale_is_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ALICE, 10_000_000));
		let signer = RuntimeOrigin::signed(ALICE);

		assert_ok!(KittiesModule::create(signer.clone()));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ALICE));

		// 上架 Kitty
		assert_ok!(KittiesModule::sale(signer.clone(), KITTY_ID_0));

		System::assert_last_event(Event::KittyOnSale { who: ALICE, kitty_id: KITTY_ID_0 }.into());

		// 重复上架
		assert_noop!(KittiesModule::sale(signer.clone(), KITTY_ID_0), Error::<Test>::AlreadyOnSale);
	});
}

#[test]
fn buy_is_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ALICE, 10_000_000));
		assert_ok!(init_balance(BOB, 10_000_000));
		let signer_alice = RuntimeOrigin::signed(ALICE);
		let signer = RuntimeOrigin::signed(BOB);

		assert_ok!(KittiesModule::create(signer_alice.clone()));
		assert_eq!(KittiesModule::kitty_owner(KITTY_ID_0), Some(ALICE));
		// 上架 Kitty
		assert_ok!(KittiesModule::sale(signer_alice.clone(), KITTY_ID_0));

		// 购买 Kitty
		assert_ok!(KittiesModule::buy(signer.clone(), KITTY_ID_0));

		System::assert_last_event(Event::KittyBought { who: BOB, kitty_id: KITTY_ID_0 }.into());
	});
}
