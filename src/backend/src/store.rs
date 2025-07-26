use candid::Principal;
use std::{cell::RefCell, collections::HashMap};

// Legacy Store implementation for backward compatibility
pub struct Store;

// Thread-local storage for the main data structure (legacy)
thread_local! {
   pub static DATA: RefCell<HashMap<Principal, String>> = RefCell::new(HashMap::new());
}

impl Store {
    // Retrieves a value from the store for the current caller
    pub fn get() -> Option<String> {
        DATA.with(|data| data.borrow().get(&ic_cdk::api::msg_caller()).cloned())
    }

    // Inserts a value into the store for the current caller
    pub fn insert(s: String) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.insert(ic_cdk::api::msg_caller(), s);
        })
    }

    // Gets data for a specific principal
    pub fn get_for_principal(principal: &Principal) -> Option<String> {
        DATA.with(|data| data.borrow().get(principal).cloned())
    }

    // Inserts data for a specific principal
    pub fn insert_for_principal(principal: Principal, s: String) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.insert(principal, s);
        })
    }

    // Removes data for a specific principal
    pub fn remove_for_principal(principal: &Principal) -> Option<String> {
        DATA.with(|data| data.borrow_mut().remove(principal))
    }

    // Gets all stored data
    pub fn get_all() -> HashMap<Principal, String> {
        DATA.with(|data| data.borrow().clone())
    }

    // Clears all stored data
    pub fn clear() {
        DATA.with(|data| data.borrow_mut().clear())
    }

    // Gets the count of stored items
    pub fn count() -> usize {
        DATA.with(|data| data.borrow().len())
    }

    // Checks if a principal has data stored
    pub fn contains_principal(principal: &Principal) -> bool {
        DATA.with(|data| data.borrow().contains_key(principal))
    }
}
