use super::*; // 引入模块的所用存储项

// use crate::{mock::*, Error}; // 引入 mock 的一些定义
use frame_support::{assert_noop, assert_ok};
// use mock::{new_test_ext, KittiesModule, RuntimeEvent as TestEvent, RuntimeOrigin, System, Test};
use crate::{
	mock::{new_test_ext, KittiesModule, RuntimeEvent as TestEvent, RuntimeOrigin, System, Test},
	Error, Event,
};
#[test]
fn test_module_works() {
	assert_eq!(1, 1);
}

// unit-testing Even Test Checks
// https://docs.substrate.io/test/unit-testing/

//构建三个address,开发链有一系列测试账户，其中前两个账户有一定数额的测试余额
const ALICE: u64 = 0; //100
const BOB: u64 = 1; //25
const CHARLIE: u64 = 2; //1

// 测试创建一个 Kitty Events
#[test]
fn created_checks_events() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);

		let alice: u64 = ALICE;
		//获取当前kitty的id,创建时会默认将该id给新的kitty
		let kitty_id = NextKittyId::<Test>::get();
		//断言是否创建成功
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));
		assert_ne!(Kitties::<Test>::get(kitty_id), None);
		//测试event
		let kitty: Kitty = Kitties::<Test>::get(kitty_id).unwrap();

		System::assert_has_event(Event::KittyCreated { who: alice, kitty_id, kitty }.into());
		assert_eq!(System::events().len(), 1);
	})
}

#[test]
fn create_kitties_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());

		// 断言储存值 是否相等
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
		let kitty: Kitty = Kitties::<Test>::get(kitty_id).unwrap();

		System::assert_has_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());
	});
}

#[test]
fn breed_is_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let breed_kitty_id = 2;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

		let breed_kitty: Kitty = KittiesModule::kitties(breed_kitty_id).expect("Kitty Breed");
		System::assert_last_event(
			// Event::KittyBreed { who: account_id, kitty_id, kitty: Kitty::default() }.into(),
			Event::KittyBreed { who: account_id, kitty_id: breed_kitty_id, kitty: breed_kitty }
				.into(),
		);
	});
}

#[test]
fn transfer_is_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));

		System::assert_has_event(
			Event::KittyTransfered { who: account_id, kitty_id, recipient }.into(),
		);
	});
}

#[test]
fn sale_is_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

		assert_noop!(
			KittiesModule::kitty_on_sale(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::AlreadyOnSale
		);

		System::assert_has_event(Event::KittyOnSale { who: account_id, kitty_id }.into());
	});
}
