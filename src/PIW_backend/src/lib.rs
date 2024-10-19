use candid::{CandidType, Principal, Deserialize};
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

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    // Check if the request is for Internet Identity authentication
    if req.url.contains("authenticate") {
        // Handle the Internet Identity authentication request
        HttpResponse {
            status_code: 302,
            headers: vec![("Location".to_string(), "https://identity.ic0.app/".to_string())],
            body: vec![],
        }
    } else {
        // Handle other requests
        HttpResponse {
            status_code: 200,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: b"Hello, World!".to_vec(),
        }
    }
}