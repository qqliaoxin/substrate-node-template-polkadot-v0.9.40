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
// https://github.com/shiyivei/substrate-advanced-course/blob/fdd14d4a6f307667842898f84abbc8b532eee19d/lesson4/backend/pallets/kitties/src/tests.rs
#[test]
fn breed_checks_events() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);

		let alice: u64 = ALICE;
		let kitty_id_1 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));

		let kitty_id_2 = NextKittyId::<Test>::get();
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));

		let new_kitty_id = NextKittyId::<Test>::get();

		//断言是否创建成功
		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(alice), kitty_id_1, kitty_id_2));

		//断言新创建的kitty持有人是当前创建者
		assert_eq!(KittyOwner::<Test>::get(new_kitty_id), Some(alice));

		//断言kitty已经存储,assert_ne 和 None
		// 双重否定（即断言成功），为什么要使用这个，因为我们无法判断生成的kitty的具体值，
		// 但是可以确定它是有值还是无值，所以就可以使用这种方式来对存在性进行判断
		assert_ne!(Kitties::<Test>::get(new_kitty_id), None);

		//断言下一个kitty的id
		// assert_eq!(NextKittyId::<Test>::get(), new_kitty_id.add(&1));

		// //断言被资金被锁定
		// assert_eq!(
		// 	<Test as Config>::Currency::reserved_balance(&alice),
		// 	<Test as Config>::KittyPrice::get().checked_mul(3).unwrap()
		// );

		//测试event
		let kitty = Kitties::<Test>::get(new_kitty_id).unwrap();

		System::assert_has_event(Event::KittyBreed { who: alice, kitty_id_1, kitty_id_2 }.into());
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
	});
}

#[test]
fn breed_is_works() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

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

		let breed_kitty_id = 2;

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
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
	});
}
