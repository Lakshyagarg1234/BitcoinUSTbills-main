use crate::guard::GUARD;
use crate::store::DATA;
use candid::{CandidType, Principal};
use ic_cdk::{post_upgrade, pre_upgrade, storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;

// StableStore struct is used for serializing and deserializing the data during upgrades
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct StableStore {
    pub data: HashMap<Principal, String>,
    pub guard: Vec<Principal>,
}

#[pre_upgrade]
pub fn pre_upgrade_handler() {
    let stable_store = DATA.with(|data| {
        GUARD.with(|guard| {
            let data_ref = data.borrow().clone();
            let guard_ref = guard.borrow().iter().cloned().collect();
            StableStore {
                data: data_ref,
                guard: guard_ref,
            }
        })
    });
    storage::stable_save((stable_store,)).unwrap();
}

// Post-upgrade hook: Restores the state from stable storage after an upgrade
#[post_upgrade]
pub fn post_upgrade_handler() {
    let (stable_store,): (StableStore,) = storage::stable_restore().unwrap();
    DATA.with(|data| {
        *data.borrow_mut() = stable_store.data;
    });

    GUARD.with(|guard| {
        *guard.borrow_mut() = BTreeSet::from_iter(stable_store.guard);
    })
}
