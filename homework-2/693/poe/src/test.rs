use super::*;
use crate::{mock::*,Error};

use frame_support::{assert_noop, assert_ok, BoundedVec, pallet_prelude::Get};

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(
            PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
       
    })


}

#[test]
fn create_claim_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(
            PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1),claim.clone()),
            Error::<Test>::ProofAlreadyExist 
        );
       
    })

}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(
            PoeModule::create_claim(
                RuntimeOrigin::signed(1),
                claim.clone()
            )
        );

        assert_ok!(
            PoeModule::revoke_claim(
                RuntimeOrigin::signed(1),
                claim.clone()
            )
        );
        assert_eq!(
            Proofs::<Test>::get(&claim),
            None
        );
    });
}

#[test]
fn revoke_claim_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        assert_noop!(
            PoeModule::revoke_claim(
                RuntimeOrigin::signed(1),
                claim.clone()
            ),
            Error::<Test>::ClaimnotExist
        );
    });
}


#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        assert_ok!(
            PoeModule::create_claim(
                RuntimeOrigin::signed(1),
                claim.clone()
            )
        );

        
        assert_ok!(
            PoeModule::transfer_claim(
                RuntimeOrigin::signed(1),
                claim.clone(),
                2
            )
        );

        
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn transfer_claim_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

        
        assert_noop!(
            PoeModule::transfer_claim(
                RuntimeOrigin::signed(1),
                claim.clone(),
                2
            ),
            Error::<Test>::ClaimnotExist
        );
    });
}

#[test]
fn transfer_claim_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        
        assert_ok!(
            PoeModule::create_claim(
                RuntimeOrigin::signed(1),
                claim.clone()
            )
        );

        
        assert_noop!(
            PoeModule::transfer_claim(
                RuntimeOrigin::signed(2),
                claim.clone(),
                3
            ),
            Error::<Test>::NotClaimOwner
        );
    });
}
