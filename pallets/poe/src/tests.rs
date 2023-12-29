#![feature(let_chains)]

use super::*;
use crate::{mock::*,Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_system::ensure_signed;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		// 断言error结果 并且不修改链上状态
		assert_noop!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist)
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		// 断言error结果 并且不修改链上状态
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		// 断言error结果 并且不修改链上状态
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		)
	});
}

#[test]
fn revoke_claim_failed_when_claim_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言error结果 并且不修改链上状态
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		)
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let receive = ensure_signed(RuntimeOrigin::signed(2))?;
		// 断言error结果 并且不修改链上状态
		assert_ok!(PoeModule::trans_claim(RuntimeOrigin::signed(1), claim.clone(), receive));
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		// let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let receive = ensure_signed(RuntimeOrigin::signed(2))?;
		// 断言error结果 并且不修改链上状态
		assert_noop!(PoeModule::trans_claim(RuntimeOrigin::signed(1), claim, receive), Error::<Test>::ClaimNotExist);
	});
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		// let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let receive = ensure_signed(RuntimeOrigin::signed(2))?;
		// 断言error结果 并且不修改链上状态
		assert_noop!(PoeModule::trans_claim(RuntimeOrigin::signed(3), claim, receive), Error::<Test>::NotClaimOwner);
	});
}

#[test]
fn transfer_claim_failed_with_not_transfer_self() {
	new_test_ext().execute_with(|| {
		// 生成claim
		let claim = BoundedVec::try_from(vec![0,1]).unwrap();
		// let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let receive = ensure_signed(RuntimeOrigin::signed(1))?;
		// 断言error结果 并且不修改链上状态
		assert_noop!(PoeModule::trans_claim(RuntimeOrigin::signed(1), claim, receive), Error::<Test>::NotTransferSelf);
	});
}