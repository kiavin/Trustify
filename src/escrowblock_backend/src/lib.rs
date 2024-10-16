use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::canister_balance;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// ====================
// Data Structures
// ====================

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub enum EscrowStatus {
    Created,
    Agreed,
    Funded,
    GoodsShipped,
    FundsReleased,
    Disputed,
    Resolved,
    Refunded,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub enum ReleaseMethod {
    Mpesa,
    Icp,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub enum DisputeDecision {
    RefundBuyer,
    ReleaseFunds,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct MpesaTransaction {
    pub transaction_id: String,
    pub escrow_id: u64,
    pub buyer_principal: Principal,
    pub seller_principal: Principal,
    pub amount_e8s: u64, // Changed from u128 to u64
    pub timestamp: u64,  // Unix timestamp in nanoseconds
    pub status: String,   // e.g., "Completed", "Failed"
    pub receipt_number: String,
    pub payer_phone: String,
    pub receiver_phone: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct EscrowAgreement {
    pub escrow_id: u64,
    pub buyer: Principal,
    pub seller: Principal,
    pub amount: u64, // Changed from u128 to u64
    pub terms: String,
    pub status: EscrowStatus,
    pub created_at: u64,          // Timestamp in nanoseconds
    pub agreed_at: Option<u64>,   // Timestamp when seller agreed
    pub funded_at: Option<u64>,   // Timestamp when funds were added
    pub shipped_at: Option<u64>,  // Timestamp when goods were shipped
    pub released_at: Option<u64>, // Timestamp when funds were released
    pub disputed_at: Option<u64>, // Timestamp when dispute was opened
    pub resolved_at: Option<u64>, // Timestamp when dispute was resolved
    pub release_method: Option<ReleaseMethod>, // Method of fund release
    pub mpesa_transactions: Vec<MpesaTransaction>, // List of associated M-Pesa transactions
}

// ====================
// Canister State
// ====================

thread_local! {
    static ESCROWS: RefCell<HashMap<u64, EscrowAgreement>> = RefCell::new(HashMap::new());
    static NEXT_ESCROW_ID: RefCell<u64> = RefCell::new(1);
    static ADMIN: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static OFF_CHAIN_SERVER_PRINCIPAL: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

// ====================
// Initialization
// ====================

#[ic_cdk::init]
fn init() {
    let caller = ic_cdk::caller();
    ADMIN.with(|admin| {
        *admin.borrow_mut() = caller;
    });
}

// ====================
// Admin Management
// ====================

/// Sets a new admin. Only the current admin can perform this action.
#[ic_cdk::update]
fn set_admin(new_admin: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();
    ADMIN.with(|admin| {
        if caller != *admin.borrow() {
            return Err("Only current admin can set a new admin.".to_string());
        }
        *admin.borrow_mut() = new_admin;
        Ok(())
    })
}

/// Sets the Off-Chain Server's Principal. Only admin can perform this.
#[ic_cdk::update]
fn set_off_chain_server(server_principal: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();
    ADMIN.with(|admin| {
        if caller != *admin.borrow() {
            return Err("Only admin can set the Off-Chain Server.".to_string());
        }
        OFF_CHAIN_SERVER_PRINCIPAL.with(|server| {
            *server.borrow_mut() = server_principal;
        });
        Ok(())
    })
}

// ====================
// Escrow Functions
// ====================

/// Initiates a new escrow.
///
/// # Arguments
///
/// * `seller` - Principal of the seller
/// * `buyer` - Principal of the buyer
/// * `amount_e8s` - Amount in e8s (ICP)
/// * `terms` - Terms of the escrow agreement
#[ic_cdk::update]
fn initiate_escrow(
    seller: Principal,
    buyer: Principal,
    amount_e8s: u64,
    terms: String,
) -> Result<u64, String> {
    // Validate amount
    if amount_e8s == 0 {
        return Err("Amount must be greater than zero.".to_string());
    }

    // Generate unique escrow_id
    let escrow_id = NEXT_ESCROW_ID.with(|id| {
        let current = *id.borrow();
        *id.borrow_mut() += 1;
        current
    });

    // Create EscrowAgreement
    let agreement = EscrowAgreement {
        escrow_id,
        buyer,
        seller,
        amount: amount_e8s,
        terms,
        status: EscrowStatus::Created,
        created_at: ic_cdk::api::time(),
        agreed_at: None,
        funded_at: None,
        shipped_at: None,
        released_at: None,
        disputed_at: None,
        resolved_at: None,
        release_method: None,
        mpesa_transactions: Vec::new(),
    };

    // Store EscrowAgreement
    ESCROWS.with(|escrows| {
        escrows.borrow_mut().insert(escrow_id, agreement);
    });

    Ok(escrow_id)
}

/// Seller agrees to the escrow terms.
#[ic_cdk::update]
fn agree_escrow(escrow_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.seller != caller {
                return Err("Only the designated seller can agree to the escrow.".to_string());
            }
            if agreement.status != EscrowStatus::Created {
                return Err("Escrow is not in Created status.".to_string());
            }
            agreement.status = EscrowStatus::Agreed;
            agreement.agreed_at = Some(ic_cdk::api::time());
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Funds the escrow.
/// **Note**: Actual ICP transfer is handled via the frontend and Off-Chain Server.
#[ic_cdk::update]
fn fund_escrow(escrow_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.buyer != caller {
                return Err("Only the buyer can fund the escrow.".to_string());
            }
            if agreement.status != EscrowStatus::Agreed {
                return Err("Escrow is not in Agreed status.".to_string());
            }
            // Check if the canister has received the required funds
            let balance = canister_balance();
            if balance < agreement.amount {
                return Err("Insufficient funds in escrow canister.".to_string());
            }
            agreement.status = EscrowStatus::Funded;
            agreement.funded_at = Some(ic_cdk::api::time());
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Seller confirms goods have been shipped.
#[ic_cdk::update]
fn confirm_goods_shipped(escrow_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.seller != caller {
                return Err("Only the designated seller can confirm shipment.".to_string());
            }
            if agreement.status != EscrowStatus::Funded {
                return Err("Escrow is not in Funded status.".to_string());
            }
            agreement.status = EscrowStatus::GoodsShipped;
            agreement.shipped_at = Some(ic_cdk::api::time());
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Buyer confirms receipt of goods, specifying the release method (M-Pesa or ICP).
/// **Note**: Actual ICP transfer is handled via the frontend or Off-Chain Server.
#[ic_cdk::update]
fn confirm_goods_received(escrow_id: u64, method: ReleaseMethod) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.buyer != caller {
                return Err("Only the buyer can confirm receipt of goods.".to_string());
            }
            if agreement.status != EscrowStatus::GoodsShipped {
                return Err("Escrow is not in GoodsShipped status.".to_string());
            }
            agreement.status = EscrowStatus::FundsReleased;
            agreement.released_at = Some(ic_cdk::api::time());
            agreement.release_method = Some(method);
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Confirms that ICP funds have been successfully transferred to the seller.
/// **Note**: This should be called by the frontend after successful ICP transfer.
#[ic_cdk::update]
fn confirm_icp_transfer(escrow_id: u64) -> Result<(), String> {
    let _caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            // Ensure that the release method was set to ICP
            match agreement.release_method {
                Some(ReleaseMethod::Icp) => {}
                _ => return Err("Escrow is not set to release funds via ICP.".to_string()),
            }

            // Ensure the escrow is in FundsReleased status
            if agreement.status != EscrowStatus::FundsReleased {
                return Err("Escrow is not in FundsReleased status.".to_string());
            }

            // Optionally, add a field to mark ICP transfer completion
            // For demonstration, we'll just acknowledge the confirmation.
            // You might want to log the transfer details or update the escrow agreement accordingly.

            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Either Buyer or Seller can open a dispute.
#[ic_cdk::update]
fn open_dispute(escrow_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.buyer != caller && agreement.seller != caller {
                return Err("Only buyer or seller can open a dispute.".to_string());
            }
            if agreement.status == EscrowStatus::FundsReleased || agreement.status == EscrowStatus::Refunded {
                return Err("Cannot dispute a completed or refunded escrow.".to_string());
            }
            agreement.status = EscrowStatus::Disputed;
            agreement.disputed_at = Some(ic_cdk::api::time());
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Admin resolves a dispute by choosing to refund the buyer or release funds to the seller.
/// **Note**: Actual M-Pesa transfer or ICP transfer is handled via the Off-Chain Server or frontend.
#[ic_cdk::update]
fn resolve_dispute(escrow_id: u64, decision: DisputeDecision) -> Result<(), String> {
    let caller = ic_cdk::caller();

    // Check if caller is admin
    ADMIN.with(|admin| {
        if caller != *admin.borrow() {
            return Err("Only admin can resolve disputes.".to_string());
        }
        Ok(())
    })?;

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&escrow_id) {
            if agreement.status != EscrowStatus::Disputed {
                return Err("Escrow is not in Disputed status.".to_string());
            }
            match decision {
                DisputeDecision::RefundBuyer => {
                    agreement.status = EscrowStatus::Refunded;
                    // Trigger Off-Chain Server or frontend to refund buyer via M-Pesa or ICP
                }
                DisputeDecision::ReleaseFunds => {
                    agreement.status = EscrowStatus::FundsReleased;
                    // Trigger Off-Chain Server or frontend to release funds to seller via M-Pesa or ICP
                }
            }
            agreement.resolved_at = Some(ic_cdk::api::time());
            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Cancels an existing escrow. Can only be done if it's not funded.
#[ic_cdk::update]
fn cancel_escrow(escrow_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get(&escrow_id) {
            if agreement.buyer != caller && agreement.seller != caller {
                return Err("Only the buyer or seller can cancel the escrow.".to_string());
            }
            if agreement.status != EscrowStatus::Created && agreement.status != EscrowStatus::Agreed {
                return Err("Can only cancel escrows in Created or Agreed status.".to_string());
            }
        } else {
            return Err("Escrow ID not found.".to_string());
        }
        escrows.remove(&escrow_id);
        Ok(())
    })
}

// ====================
// Query Functions
// ====================

/// Retrieves details of a specific escrow.
#[ic_cdk::query]
fn get_escrow(escrow_id: u64) -> Result<EscrowAgreement, String> {
    ESCROWS.with(|escrows| {
        let escrows = escrows.borrow();
        escrows
            .get(&escrow_id)
            .cloned()
            .ok_or("Escrow ID not found.".to_string())
    })
}

/// Lists all escrows associated with the caller (as Buyer or Seller).
#[ic_cdk::query]
fn list_my_escrows() -> Vec<EscrowAgreement> {
    let caller = ic_cdk::caller();
    ESCROWS.with(|escrows| {
        escrows
            .borrow()
            .values()
            .filter(|e| e.buyer == caller || e.seller == caller)
            .cloned()
            .collect()
    })
}

// ====================
// M-Pesa Transaction Recording
// ====================

/// Records an M-Pesa transaction. Only the Off-Chain Server can call this.
/// Ensures immutability by only allowing appending transactions.
#[ic_cdk::update]
fn record_mpesa_transaction(transaction: MpesaTransaction) -> Result<(), String> {
    let caller = ic_cdk::caller();

    // Ensure only the Off-Chain Server can call this function
    OFF_CHAIN_SERVER_PRINCIPAL.with(|server| {
        if caller != *server.borrow() {
            return Err("Unauthorized caller.".to_string());
        }
        Ok(())
    })?;

    ESCROWS.with(|escrows| {
        let mut escrows = escrows.borrow_mut();
        if let Some(agreement) = escrows.get_mut(&transaction.escrow_id) {
            // Append the transaction to the escrow agreement
            agreement.mpesa_transactions.push(transaction.clone());

            // Update escrow status based on transaction status
            match transaction.status.as_str() {
                "Completed" => {
                    agreement.status = EscrowStatus::Funded;
                    agreement.funded_at = Some(transaction.timestamp);
                }
                "Failed" => {
                    // Optionally handle failed transactions
                    // For example, notify buyer or update escrow status
                    return Err("M-Pesa transaction failed.".to_string());
                }
                _ => {
                    // Handle other statuses if necessary
                }
            }

            Ok(())
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}

/// Retrieves all M-Pesa transactions for a specific escrow.
#[ic_cdk::query]
fn get_mpesa_transactions(escrow_id: u64) -> Result<Vec<MpesaTransaction>, String> {
    ESCROWS.with(|escrows| {
        let escrows = escrows.borrow();
        escrows
            .get(&escrow_id)
            .map(|e| e.mpesa_transactions.clone())
            .ok_or("Escrow ID not found.".to_string())
    })
}

// ====================
// Additional Utility Functions
// ====================

/// (Optional) Retrieve Buyer and Seller Principals based on Escrow ID.
/// Useful for Off-Chain Server or frontend when initiating M-Pesa or ICP transfers.
#[ic_cdk::query]
fn get_participants(escrow_id: u64) -> Result<(Principal, Principal), String> {
    ESCROWS.with(|escrows| {
        let escrows = escrows.borrow();
        if let Some(agreement) = escrows.get(&escrow_id) {
            Ok((agreement.buyer, agreement.seller))
        } else {
            Err("Escrow ID not found.".to_string())
        }
    })
}
