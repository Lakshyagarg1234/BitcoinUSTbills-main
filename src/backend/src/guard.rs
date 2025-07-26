use crate::errors::{BitcoinUSTBillsError, Result};
use candid::Principal;
use std::{cell::RefCell, collections::BTreeSet};

const INITIAL_AUTHORIZED_PRINCIPAL: &str =
    "6lzil-lzkgm-twmv5-rz5xg-a5nnm-togvj-mlu6s-p4xyl-5j3zi-6a6jy-yqe";

thread_local! {
    pub static GUARD: RefCell<BTreeSet<Principal>> = RefCell::new({
        let mut set = BTreeSet::new();
        let initial_principal = Principal::from_text(INITIAL_AUTHORIZED_PRINCIPAL)
        .expect("Invalid initial principal");

        set.insert(initial_principal);
        set
    });
}

/// Adds a principal to the authorized list
pub fn add_to_list(principal: Principal) -> () {
    GUARD.with(|guard| {
        let mut guard_ref = guard.borrow_mut();

        if guard_ref.contains(&principal) {
            return ();
        } else {
            guard_ref.insert(principal);
            return ();
        }
    })
}

/// Removes a principal from the authorized list
pub fn delete_from_list(p: Principal) -> String {
    GUARD.with(|guard| {
        let mut guard_ref = guard.borrow_mut();

        if !guard_ref.contains(&p) {
            return "Principal {} is not in the guard list".to_string();
        } else {
            guard_ref.remove(&p);
            return "removed".to_string();
        }
    })
}

/// Checks if the caller is an authorized developer/admin
pub fn is_dev() -> std::result::Result<(), String> {
        let caller = ic_cdk::api::msg_caller();
    let anonymous = Principal::anonymous();
    if caller == anonymous {
        return Err("AnonymousCaller".to_string());
    }

    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        if !guard_ref.contains(&caller) {
            return Err(format!("Caller {} is not authorized", caller));
        } else {
            return Ok(());
        }
    })
}

/// Checks if the caller is an admin (same as is_dev for now)
pub fn is_admin() -> std::result::Result<(), String> {
    is_dev()
}

/// Assert that the caller is an admin, returning BitcoinUSTBillsError
pub fn assert_admin() -> Result<()> {
        let caller = ic_cdk::api::msg_caller();
    let anonymous = Principal::anonymous();

    if caller == anonymous {
        return Err(BitcoinUSTBillsError::AnonymousCaller);
    }

    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        if !guard_ref.contains(&caller) {
            return Err(BitcoinUSTBillsError::Unauthorized);
        } else {
            return Ok(());
        }
    })
}

/// Assert that the caller is a verified user (not anonymous)
pub fn assert_user() -> Result<()> {
        let caller = ic_cdk::api::msg_caller();
    let anonymous = Principal::anonymous();

    if caller == anonymous {
        return Err(BitcoinUSTBillsError::AnonymousCaller);
    }

    Ok(())
}

/// Checks if a principal is in the authorized list
pub fn is_authorized(principal: &Principal) -> bool {
    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        guard_ref.contains(principal)
    })
}

/// Gets all authorized principals
pub fn get_authorized_principals() -> Vec<Principal> {
    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        guard_ref.iter().cloned().collect()
    })
}

/// Gets the count of authorized principals
pub fn get_authorized_count() -> usize {
    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        guard_ref.len()
    })
}

/// Checks if the caller is the specific principal
pub fn assert_caller_is(expected: &Principal) -> Result<()> {
        let caller = ic_cdk::api::msg_caller();

    if caller != *expected {
        return Err(BitcoinUSTBillsError::Unauthorized);
    }

    Ok(())
}

/// Checks if the caller is either admin or the specific principal
pub fn assert_admin_or_caller(expected: &Principal) -> Result<()> {
        let caller = ic_cdk::api::msg_caller();

    // Check if caller is admin
    if is_authorized(&caller) {
        return Ok(());
    }

    // Check if caller is the expected principal
    if caller == *expected {
        return Ok(());
    }

    Err(BitcoinUSTBillsError::Unauthorized)
}

/// Initializes the guard with a specific principal
pub fn init_guard(principal: Principal) -> Result<()> {
    GUARD.with(|guard| {
        let mut guard_ref = guard.borrow_mut();
        guard_ref.insert(principal);
        Ok(())
    })
}

/// Clears all authorized principals (use with caution)
pub fn clear_guard() -> Result<()> {
    assert_admin()?;

    GUARD.with(|guard| {
        let mut guard_ref = guard.borrow_mut();
        guard_ref.clear();
        Ok(())
    })
}

/// Validates that a principal is not anonymous
pub fn validate_principal(principal: &Principal) -> Result<()> {
    if *principal == Principal::anonymous() {
        return Err(BitcoinUSTBillsError::AnonymousCaller);
    }
    Ok(())
}

/// Checks if the system has any authorized principals
pub fn has_authorized_principals() -> bool {
    GUARD.with(|guard| {
        let guard_ref = guard.borrow();
        !guard_ref.is_empty()
    })
}

/// Gets the initial authorized principal
pub fn get_initial_principal() -> Principal {
    Principal::from_text(INITIAL_AUTHORIZED_PRINCIPAL).expect("Invalid initial principal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_principal() {
        let initial = get_initial_principal();
        assert!(is_authorized(&initial));
    }

    #[test]
    fn test_add_and_remove_principal() {
        let test_principal = Principal::from_text("rdmx6-jaaaa-aaaah-qcaiq-cai").unwrap();

        // Initially should not be authorized
        assert!(!is_authorized(&test_principal));

        // Add to list
        add_to_list(test_principal);
        assert!(is_authorized(&test_principal));

        // Remove from list
        delete_from_list(test_principal);
        assert!(!is_authorized(&test_principal));
    }

    #[test]
    fn test_validate_principal() {
        let anonymous = Principal::anonymous();
        assert!(validate_principal(&anonymous).is_err());

        let valid = get_initial_principal();
        assert!(validate_principal(&valid).is_ok());
    }
}
