// 1. Core Data Structures

// Your smart contract should define all necessary entities and their relationships. Here's a detailed approach:
// Escrow Contract Data

// Each escrow contract must include:

//     Unique ID: To identify the contract.
//     Participants: Addresses or IDs of the payer and payee.
//     Amount: Funds locked in the escrow.
//     Conditions: Any text or structured data describing the release/refund conditions.
//     Status: Tracks the current state (Pending, Active, Released, Refunded, Disputed).
//     Timestamps: Created and updated times.

// Rust Implementation:

        use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct EscrowContract {
    id: u64,                     // Unique contract ID
    payer: String,               // Payer's address or ID
    payee: String,               // Payee's address or ID
    amount: u64,                 // Amount in ICP tokens
    conditions: String,          // Conditions for release/refund
    status: ContractStatus,      // Enum: Pending, Active, etc.
    created_at: u64,             // Timestamp
    updated_at: u64,             // Timestamp
}

#[derive(CandidType, Deserialize, Clone, PartialEq)]
pub enum ContractStatus {
    Pending,
    Active,
    Released,
    Refunded,
    Disputed,
}

// Store all escrow contracts
type EscrowDB = HashMap<u64, EscrowContract>;



// 2. Core Functions
// a. Create Escrow

// This function initializes a new escrow contract:

//     Generates a unique ID.
//     Stores the contract in the state.

use std::time::{SystemTime, UNIX_EPOCH};

static mut ESCROWS: Option<EscrowDB> = None; // Global storage

#[update]
fn create_escrow(payer: String, payee: String, amount: u64, conditions: String) -> u64 {
    // Generate a unique ID
    let contract_id = get_unique_id();
    let now = get_current_timestamp();

    let escrow = EscrowContract {
        id: contract_id,
        payer,
        payee,
        amount,
        conditions,
        status: ContractStatus::Pending,
        created_at: now,
        updated_at: now,
    };

    unsafe {
        ESCROWS.get_or_insert_with(HashMap::new).insert(contract_id, escrow);
    }

    contract_id
}

fn get_unique_id() -> u64 {
    // Replace this with a better unique ID generator if needed
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}




// b. Release Funds

// Allows the payer to release funds to the payee when conditions are met.

#[update]
fn release_funds(contract_id: u64) -> Result<String, String> {
    unsafe {
        let escrows = ESCROWS.get_or_insert_with(HashMap::new);
        match escrows.get_mut(&contract_id) {
            Some(contract) if contract.status == ContractStatus::Active => {
                // Perform fund transfer logic (ICP Ledger integration)
                transfer_funds(&contract.payer, &contract.payee, contract.amount)?;

                // Update contract status
                contract.status = ContractStatus::Released;
                contract.updated_at = get_current_timestamp();
                Ok(format!("Funds released for contract {}", contract_id))
            }
            Some(_) => Err("Contract is not active.".to_string()),
            None => Err("Contract not found.".to_string()),
        }
    }
}


// c. Refund Funds

// Refunds the funds back to the payer if conditions fail or the payer decides to cancel.

#[update]
fn refund_funds(contract_id: u64) -> Result<String, String> {
    unsafe {
        let escrows = ESCROWS.get_or_insert_with(HashMap::new);
        match escrows.get_mut(&contract_id) {
            Some(contract) if contract.status == ContractStatus::Pending => {
                // Perform fund refund logic
                transfer_funds(&contract.payee, &contract.payer, contract.amount)?;

                // Update contract status
                contract.status = ContractStatus::Refunded;
                contract.updated_at = get_current_timestamp();
                Ok(format!("Funds refunded for contract {}", contract_id))
            }
            Some(_) => Err("Contract cannot be refunded.".to_string()),
            None => Err("Contract not found.".to_string()),
        }
    }
}





// d. Dispute Handling

// Allow disputes where a neutral party (e.g., an arbitrator) resolves conflicts.

#[update]
fn dispute_contract(contract_id: u64) -> Result<String, String> {
    unsafe {
        let escrows = ESCROWS.get_or_insert_with(HashMap::new);
        match escrows.get_mut(&contract_id) {
            Some(contract) => {
                contract.status = ContractStatus::Disputed;
                contract.updated_at = get_current_timestamp();
                Ok(format!("Contract {} marked as disputed.", contract_id))
            }
            None => Err("Contract not found.".to_string()),
        }
    }
}





// 3. ICP Ledger Integration

// Integrate with the ICP Ledger for token transfers. You can use the ICP ledger canister for transferring funds between users.
// Transfer Funds Function



use ic_cdk::call;

async fn transfer_funds(from: &str, to: &str, amount: u64) -> Result<(), String> {
    let result: Result<(), String> = call(
        Principal::from_text("aaaaa-aa").unwrap(), // ICP Ledger canister ID
        "transfer",
        (from, to, amount),
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e));

    result
}





// 4. Query Functions
// Get Contract Details

// Fetch details of a specific escrow contract:

#[query]
fn get_contract(contract_id: u64) -> Option<EscrowContract> {
    unsafe { ESCROWS.as_ref()?.get(&contract_id).cloned() }
}


// List All Contracts

// Fetch all contracts for a user:
#[query]
fn list_user_contracts(user_id: String) -> Vec<EscrowContract> {
    unsafe {
        ESCROWS
            .as_ref()
            .map(|escrows| {
                escrows
                    .values()
                    .filter(|c| c.payer == user_id || c.payee == user_id)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}




// Types of Notifications:

//     Contract-Based Events:
//         Contract Created
//         Contract Activated
//         Funds Released
//         Refund Processed
//         Dispute Raised or Resolved
//     System-Wide Notifications:
//         Updates from the escrow platform (e.g., changes in policies).

// Delivery Channels:

//     In-App Notifications: Delivered within the frontend (Vue.js).
//     Push Notifications: For real-time updates (optional, requires additional integration).
//     Email or SMS Notifications: Using external services like SendGrid or Twilio.


// Notification Data Model

// Define the structure for notifications in Rust to store and manage them efficiently.

use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct Notification {
    id: u64,                 // Unique notification ID
    user_id: String,         // User who receives the notification
    message: String,         // Notification message
    contract_id: Option<u64>, // Associated escrow contract (if applicable)
    timestamp: u64,          // Time of notification
    read: bool,              // Status: Read or Unread
}

// Global storage for notifications
type NotificationDB = HashMap<String, Vec<Notification>>; // User-specific notifications
static mut NOTIFICATIONS: Option<NotificationDB> = None;




// 3. Notification Logic
// a. Create Notification

// Function to generate a notification for a specific user:

#[update]
fn create_notification(user_id: String, message: String, contract_id: Option<u64>) {
    let now = get_current_timestamp();
    let notification = Notification {
        id: get_unique_id(),
        user_id: user_id.clone(),
        message,
        contract_id,
        timestamp: now,
        read: false,
    };

    unsafe {
        let notifications = NOTIFICATIONS.get_or_insert_with(HashMap::new);
        notifications.entry(user_id).or_insert_with(Vec::new).push(notification);
    }
}


// b. Mark Notification as Read

// Allow users to mark notifications as read:

#[update]
fn mark_notification_as_read(user_id: String, notification_id: u64) -> Result<String, String> {
    unsafe {
        if let Some(notifications) = NOTIFICATIONS.get_mut() {
            if let Some(user_notifications) = notifications.get_mut(&user_id) {
                if let Some(notification) = user_notifications.iter_mut().find(|n| n.id == notification_id) {
                    notification.read = true;
                    return Ok("Notification marked as read.".to_string());
                }
            }
        }
    }
    Err("Notification not found.".to_string())
}





// . Fetch Notifications

// Query notifications for a specific user:
#[query]
fn get_user_notifications(user_id: String) -> Vec<Notification> {
    unsafe {
        NOTIFICATIONS
            .as_ref()
            .and_then(|notifications| notifications.get(&user_id).cloned())
            .unwrap_or_default()
    }
}



// Trigger Notifications

// Notifications should be triggered at key events within your escrow system. For example:
// On Contract Creation:

fn create_escrow(payer: String, payee: String, amount: u64, conditions: String) -> u64 {
    // Contract creation logic...

    // Trigger notifications
    create_notification(
        payer.clone(),
        format!("Escrow contract created. Payee: {}, Amount: {}", payee, amount),
        Some(contract_id),
    );
    create_notification(
        payee.clone(),
        format!("You have been added as the payee for an escrow contract. Amount: {}", amount),
        Some(contract_id),
    );

    contract_id
}




// On Funds Release


fn release_funds(contract_id: u64) -> Result<String, String> {
    // Fund release logic...

    // Trigger notifications
    create_notification(
        contract.payer.clone(),
        format!("Funds successfully released for contract {}", contract_id),
        Some(contract_id),
    );
    create_notification(
        contract.payee.clone(),
        format!("You have received funds for contract {}", contract_id),
        Some(contract_id),
    );

    Ok("Funds released.")
}



// On Dispute Raised:

fn dispute_contract(contract_id: u64) -> Result<String, String> {
    // Dispute logic...

    // Trigger notifications
    create_notification(
        contract.payer.clone(),
        format!("Dispute raised for contract {}", contract_id),
        Some(contract_id),
    );
    create_notification(
        contract.payee.clone(),
        format!("Dispute raised for contract {}", contract_id),
        Some(contract_id),
    );

    Ok("Dispute raised.")
}


// . Advanced Enhancements
// a. Real-Time Notifications:

// Integrate real-time updates using WebSockets or SignalR to push notifications instantly to users.
// WebSocket Example:

const socket = new WebSocket('ws://your-backend-url');

socket.onmessage = (event) => {
  const notification = JSON.parse(event.data);
  notifications.value.push(notification); // Append to current list
};




// Email Notifications:

// Use a service like SendGrid to send email notifications for critical updates.
// Email Example in Rust:


async fn send_email_notification(to: &str, subject: &str, body: &str) -> Result<(), String> {
    // Use an email service API (e.g., SendGrid, SMTP)
    // Example using reqwest for an HTTP request
    let client = reqwest::Client::new();
    let result = client.post("https://api.sendgrid.com/v3/mail/send")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .json(&{
            "personalizations": [{"to": [{"email": to}]}],
            "subject": subject,
            "content": [{"type": "text/plain", "value": body}]
        })
        .send()
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to send email: {:?}", e)),
    }
}



