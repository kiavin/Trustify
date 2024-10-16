// Define User structure
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
struct User {
    principal: Principal,
    username: String,
    is_seller: bool,
    license_agreed: bool,
}

// Using a RefCell to allow interior mutability
thread_local! {
    static USERS: RefCell<Vec<User>> = RefCell::new(Vec::new());
}

// Register User
#[ic_cdk::update]
async fn register_user(username: String, is_seller: bool) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut users = USERS.with(|u| u.borrow_mut());

    // Check if user already exists
    if users.iter().any(|user| user.principal == caller) {
        return Err("User already registered.".to_string());
    }

    let user = User {
        principal: caller,
        username,
        is_seller,
        license_agreed: false,
    };
    users.push(user);
    Ok(())
}

// Agree to License
#[ic_cdk::update]
async fn agree_license() -> Result<(), String> {
    let caller = ic_cdk::caller();
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(user) = users.iter_mut().find(|u| u.principal == caller) {
            user.license_agreed = true;
            Ok(())
        } else {
            Err("User not found. Please register first.".to_string())
        }
    })
}

// Authenticate User
#[ic_cdk::query]
fn authenticate_user() -> Result<User, String> {
    let caller = ic_cdk::caller();
    USERS.with(|users| {
        let users = users.borrow();
        users
            .iter()
            .find(|u| u.principal == caller)
            .cloned()
            .ok_or("User not authenticated.".to_string())
    })
}

// Check License Agreement
#[ic_cdk::query]
fn has_agreed_license() -> bool {
    let caller = ic_cdk::caller();
    USERS.with(|users| {
        users
            .borrow()
            .iter()
            .find(|u| u.principal == caller)
            .map(|u| u.license_agreed)
            .unwrap_or(false)
    })
}

// List All Users (Admin Function)
#[ic_cdk::query]
fn list_users() -> Vec<User> {
    USERS.with(|users| users.borrow().clone())
}

#[ic_cdk::init]
fn init() {
    // Initialization logic if needed
}