use candid::Principal;
use ic_cdk::api::caller;
use ic_cdk_macros::{query, update, init};
use std::cell::RefCell;

thread_local! {
    static OWNER: RefCell<Option<Principal>> = RefCell::new(None);
}

#[init]
fn init() {
    OWNER.with(|owner| {
        *owner.borrow_mut() = Some(caller());
    });
}

#[update]
fn whoami() -> Principal {
    caller()
}

#[query]
fn get_owner() -> Option<Principal> {
    OWNER.with(|owner| *owner.borrow())
}

#[update]
fn set_owner(new_owner: Principal) {
    OWNER.with(|owner| {
        *owner.borrow_mut() = Some(new_owner);
    });
}