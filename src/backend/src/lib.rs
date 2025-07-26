#![allow(dead_code)]
#![warn(unused_variables)]

// Module declarations
mod errors;
mod guard;
mod handlers;
mod storage;
mod store;
mod types;
mod utils;

// Re-export types for easier access
pub use errors::*;
pub use storage::*;
pub use types::*;

use crate::utils::get_current_timestamp;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk::{query, update};
use std::collections::HashMap;
use crate::storage::VerifiedPurchasesLedgerStorage;
pub use storage::*;

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  VERIFIED BROKER PURCHASE FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

#[update]
pub async fn admin_add_broker_purchase_record(
    amount: u64,
    price: u64,
    broker_txn_id: String,
    ustbill_type: String,
) -> Result<()> {
    guard::assert_admin()?;

    let purchase = VerifiedBrokerPurchase {
        amount,
        price,
        timestamp: get_current_timestamp(),
        broker_txn_id,
        ustbill_type,
    };

    VerifiedPurchasesLedgerStorage::insert(purchase)
}

#[query]
pub fn get_all_verified_broker_purchases() -> Vec<VerifiedBrokerPurchase> {
    VerifiedPurchasesLedgerStorage::get_all()
}


// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  USTBILLS CANISTER FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Creates a new US Treasury Bill offering
#[update]
pub async fn create_ustbill(ustbill_data: USTBillCreateRequest) -> Result<USTBill> {
    // Validate admin access
    guard::assert_admin()?;

    // Validate input data
    validate_ustbill_data(&ustbill_data)?;

    let current_time = get_current_timestamp();

    let ustbill = USTBill {
        id: generate_id(),
        cusip: ustbill_data.cusip,
        face_value: ustbill_data.face_value,
        purchase_price: ustbill_data.purchase_price,
        maturity_date: ustbill_data.maturity_date,
        annual_yield: ustbill_data.annual_yield,
        total_tokens: ustbill_data.total_tokens,
        tokens_sold: 0,
        status: USTBillStatus::Active,
        created_at: current_time,
        updated_at: current_time,
        issuer: ustbill_data.issuer,
        bill_type: ustbill_data.bill_type,
    };

    // Store in database
    USTBillStorage::insert(ustbill.clone())?;

    Ok(ustbill)
}

/// Retrieves a US Treasury Bill by ID
#[query]
pub fn get_ustbill(ustbill_id: String) -> Result<USTBill> {
    USTBillStorage::get(&ustbill_id)
}

/// Retrieves all active US Treasury Bills
#[query]
pub fn get_active_ustbills() -> Vec<USTBill> {
    USTBillStorage::get_active()
}

/// Gets available token count for a specific US Treasury Bill
#[query]
pub fn get_ustbill_availability(ustbill_id: String) -> Result<u64> {
    let ustbill = USTBillStorage::get(&ustbill_id)?;
    Ok(ustbill.available_tokens())
}

/// Retrieves paginated list of US Treasury Bills
#[query]
pub fn get_ustbills_paginated(page: usize, per_page: usize) -> Result<PaginatedResponse<USTBill>> {
    let ustbills = USTBillStorage::get_all();
    let total = ustbills.len();

    let start = page * per_page;
    let end = std::cmp::min(start + per_page, total);

    let data = if start < total {
        ustbills[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(PaginatedResponse {
        data,
        total,
        page,
        per_page,
        has_next: end < total,
    })
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  USER CANISTER FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Registers a new user
#[update]
pub async fn register_user(user_data: UserRegistrationRequest) -> Result<User> {
    let principal = ic_cdk::api::msg_caller();

    // Check if user already exists
    if UserStorage::get(&principal).is_ok() {
        return Err(BitcoinUSTBillsError::UserAlreadyExists);
    }

    // Validate user data
    validate_user_data(&user_data)?;

    let current_time = get_current_timestamp();

    let user = User {
        principal,
        email: user_data.email,
        kyc_status: KYCStatus::Pending,
        wallet_balance: 0,
        total_invested: 0,
        total_yield_earned: 0,
        created_at: current_time,
        updated_at: current_time,
        is_active: true,
        phone_number: user_data.phone_number,
        country: user_data.country,
    };

    UserStorage::insert(user.clone())?;

    Ok(user)
}

/// Updates KYC status for a user
#[update]
pub async fn update_kyc_status(principal: Principal, status: KYCStatus) -> Result<()> {
    // Validate admin access
    guard::assert_admin()?;

    let mut user = UserStorage::get(&principal)?;
    user.kyc_status = status;
    user.updated_at = get_current_timestamp();

    UserStorage::update(user)?;

    Ok(())
}

/// Retrieves user profile
#[query]
pub fn get_user_profile(principal: Principal) -> Result<User> {
    UserStorage::get(&principal)
}

/// Deposits funds to user wallet
#[update]
pub async fn deposit_funds(amount: u64) -> Result<u64> {
    let principal = ic_cdk::api::msg_caller();
    let mut user = UserStorage::get(&principal)?;

    // Validate amount
    if amount == 0 {
        return Err(BitcoinUSTBillsError::InvalidAmount);
    }

    // Update user balance
    user.wallet_balance += amount;
    user.updated_at = get_current_timestamp();

    UserStorage::update(user.clone())?;

    // Record transaction
    let transaction = Transaction {
        id: generate_id(),
        user_principal: principal,
        transaction_type: TransactionType::Deposit,
        amount,
        ustbill_id: None,
        holding_id: None,
        timestamp: get_current_timestamp(),
        status: TransactionStatus::Completed,
        fees: 0,
        description: "Wallet deposit".to_string(),
    };

    TransactionStorage::insert(transaction)?;

    Ok(user.wallet_balance)
}

/// Withdraws funds from user wallet
#[update]
pub async fn withdraw_funds(amount: u64) -> Result<u64> {
    let principal = ic_cdk::api::msg_caller();
    let mut user = UserStorage::get(&principal)?;

    // Validate amount
    if amount == 0 {
        return Err(BitcoinUSTBillsError::InvalidAmount);
    }

    if user.wallet_balance < amount {
        return Err(BitcoinUSTBillsError::InsufficientFunds);
    }

    // Update user balance
    user.wallet_balance -= amount;
    user.updated_at = get_current_timestamp();

    UserStorage::update(user.clone())?;

    // Record transaction
    let transaction = Transaction {
        id: generate_id(),
        user_principal: principal,
        transaction_type: TransactionType::Withdrawal,
        amount,
        ustbill_id: None,
        holding_id: None,
        timestamp: get_current_timestamp(),
        status: TransactionStatus::Completed,
        fees: 0,
        description: "Wallet withdrawal".to_string(),
    };

    TransactionStorage::insert(transaction)?;

    Ok(user.wallet_balance)
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  TRADING CANISTER FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Buys US Treasury Bill tokens
#[update]
pub async fn buy_ustbill_tokens(ustbill_id: String, token_amount: u64) -> Result<TokenHolding> {
    let principal = ic_cdk::api::msg_caller();
    let mut user = UserStorage::get(&principal)?;

    // Validate user eligibility
    if !user.is_eligible_for_trading() {
        return Err(BitcoinUSTBillsError::TradingNotAllowed);
    }

    // Get UST Bill
    let mut ustbill = USTBillStorage::get(&ustbill_id)?;

    // Validate UST Bill availability
    if !ustbill.is_available_for_purchase() {
        return Err(BitcoinUSTBillsError::USTBillSoldOut);
    }

    if ustbill.available_tokens() < token_amount {
        return Err(BitcoinUSTBillsError::InsufficientTokens);
    }

    // Calculate purchase cost
    let cost = calculate_purchase_cost_internal(&ustbill, token_amount)?;
    let config = PlatformConfigStorage::get();

    // Validate investment limits
    if cost < config.minimum_investment {
        return Err(BitcoinUSTBillsError::MinimumInvestmentNotMet);
    }

    if cost > config.maximum_investment {
        return Err(BitcoinUSTBillsError::MaximumInvestmentExceeded);
    }

    // Check user balance
    if user.wallet_balance < cost {
        return Err(BitcoinUSTBillsError::InsufficientFunds);
    }

    // Calculate fees
    let fees = (cost as f64 * config.platform_fee_percentage) as u64;
    let total_cost = cost + fees;

    // Update user balance
    user.wallet_balance -= total_cost;
    user.total_invested += cost;
    user.updated_at = get_current_timestamp();

    // Update UST Bill
    ustbill.tokens_sold += token_amount;
    if ustbill.tokens_sold >= ustbill.total_tokens {
        ustbill.status = USTBillStatus::SoldOut;
    }
    ustbill.updated_at = get_current_timestamp();

    // Create holding
    let holding_id = generate_id();
    let holding = TokenHolding {
        id: holding_id.clone(),
        user_principal: principal,
        ustbill_id: ustbill_id.clone(),
        tokens_owned: token_amount,
        purchase_price_per_token: cost / token_amount,
        purchase_date: get_current_timestamp(),
        yield_option: YieldOption::Maturity,
        status: HoldingStatus::Active,
        current_value: cost,
        projected_yield: calculate_projected_yield(&ustbill, cost),
    };

    // Record transaction
    let transaction = Transaction {
        id: generate_id(),
        user_principal: principal,
        transaction_type: TransactionType::Purchase,
        amount: cost,
        ustbill_id: Some(ustbill_id.clone()),
        holding_id: Some(holding_id.clone()),
        timestamp: get_current_timestamp(),
        status: TransactionStatus::Completed,
        fees,
        description: format!(
            "Purchase of {} tokens from UST Bill {}",
            token_amount, ustbill_id
        ),
    };

    // Record fees transaction
    let fee_transaction = Transaction {
        id: generate_id(),
        user_principal: principal,
        transaction_type: TransactionType::Fee,
        amount: fees,
        ustbill_id: Some(ustbill_id.clone()),
        holding_id: Some(holding_id.clone()),
        timestamp: get_current_timestamp(),
        status: TransactionStatus::Completed,
        fees: 0,
        description: "Platform fee".to_string(),
    };

    // Save all updates
    UserStorage::update(user)?;
    USTBillStorage::update(ustbill)?;
    HoldingStorage::insert(holding.clone())?;
    TransactionStorage::insert(transaction)?;
    TransactionStorage::insert(fee_transaction)?;

    // Update trading metrics
    TradingMetricsStorage::update_volume(cost)?;
    TradingMetricsStorage::update_price(cost / token_amount)?;

    Ok(holding)
}

/// Calculates purchase cost for tokens
#[query]
pub fn calculate_purchase_cost(ustbill_id: String, token_amount: u64) -> Result<u64> {
    let ustbill = USTBillStorage::get(&ustbill_id)?;
    calculate_purchase_cost_internal(&ustbill, token_amount)
}

/// Calculates current value of a holding
#[query]
pub fn calculate_current_value(holding_id: String) -> Result<u64> {
    let holding = HoldingStorage::get(&holding_id)?;
    let ustbill = USTBillStorage::get(&holding.ustbill_id)?;

    // Calculate based on current market conditions and time to maturity
    let _days_to_maturity = ustbill.days_to_maturity();
    let purchase_date = holding.purchase_date;
    let current_time = get_current_timestamp();
    let days_held = (current_time - purchase_date) / 86400; // Convert to days

    // Calculate accrued yield
    let accrued_yield = holding.calculate_current_yield(ustbill.annual_yield, days_held);

    Ok(holding.current_value + accrued_yield)
}

/// Retrieves user holdings
#[query]
pub fn get_user_holdings(principal: Principal) -> Vec<TokenHolding> {
    HoldingStorage::get_by_user(&principal)
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  YIELD CANISTER FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Calculates maturity yield for a holding
#[update]
pub async fn calculate_maturity_yield(holding_id: String) -> Result<u64> {
    let holding = HoldingStorage::get(&holding_id)?;
    let ustbill = USTBillStorage::get(&holding.ustbill_id)?;

    // Check if UST Bill has matured
    let current_time = get_current_timestamp();
    if ustbill.maturity_date <= current_time {
        // Calculate full yield
        let purchase_value = holding.tokens_owned * holding.purchase_price_per_token;
        let face_value = (ustbill.face_value * holding.tokens_owned) / ustbill.total_tokens;
        let yield_amount = face_value - purchase_value;

        Ok(yield_amount)
    } else {
        Err(BitcoinUSTBillsError::MaturityDatePassed)
    }
}

/// Gets yield projection for a holding
#[query]
pub fn get_yield_projection(holding_id: String) -> Result<YieldProjection> {
    let holding = HoldingStorage::get(&holding_id)?;
    let ustbill = USTBillStorage::get(&holding.ustbill_id)?;

    let current_value = holding.current_value;
    let days_to_maturity = ustbill.days_to_maturity();
    let annual_yield_rate = ustbill.annual_yield;

    // Calculate projected yield
    let projected_yield =
        (current_value as f64 * annual_yield_rate * days_to_maturity as f64 / 365.0) as u64;
    let yield_percentage = if current_value > 0 {
        (projected_yield as f64 / current_value as f64) * 100.0
    } else {
        0.0
    };

    Ok(YieldProjection {
        holding_id,
        current_value,
        projected_yield,
        yield_percentage,
        days_to_maturity,
        annual_yield_rate,
    })
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  EXTERNAL API INTEGRATION                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Fetches current Treasury rates from external API
#[update]
pub async fn fetch_treasury_rates() -> Result<Vec<TreasuryRate>> {
    // For now, return a mock response until we can properly configure HTTP requests
    // This is a placeholder implementation
    let mock_rates = vec![TreasuryRate {
        record_date: "2024-01-01".to_string(),
        security_type: "T-Bill".to_string(),
        security_desc: "13-Week Treasury Bill".to_string(),
        rate_date: "2024-01-01".to_string(),
        rate: 5.26,
        cusip: "912796RF6".to_string(),
    }];

    // Store rates in database
    TreasuryRateStorage::clear()?;
    for rate in &mock_rates {
        TreasuryRateStorage::insert(rate.clone())?;
    }

    Ok(mock_rates)

    // TODO: Implement actual HTTP request once we have proper configuration
    // let url = "https://api.fiscaldata.treasury.gov/services/api/v1/accounting/od/rates_of_exchange";
    //
    // let request = HttpRequestType {
    //     url: url.to_string(),
    //     method: HttpMethod::GET,
    //     headers: vec![],
    //     body: None,
    //     transform: Some(TransformContext::from_name("transform_treasury_response".to_string(), vec![])),
    // };
    //
    // match http_request(request, 10_000_000_000).await {
    //     Ok((response,)) => {
    //         let rates = parse_treasury_response(response)?;
    //
    //         // Store rates in database
    //         TreasuryRateStorage::clear()?;
    //         for rate in &rates {
    //             TreasuryRateStorage::insert(rate.clone())?;
    //         }
    //
    //         Ok(rates)
    //     }
    //     Err(e) => Err(BitcoinUSTBillsError::http_request_error(format!("HTTP request failed: {:?}", e))),
    // }
}

/// Updates UST Bill market data
#[update]
pub async fn update_ustbill_market_data() -> Result<()> {
    let _rates = fetch_treasury_rates().await?;
    // Update logic would go here to sync with external data
    Ok(())
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  PLATFORM MANAGEMENT                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

/// Gets platform configuration
#[query]
pub fn get_platform_config() -> PlatformConfig {
    PlatformConfigStorage::get()
}

/// Updates platform configuration (admin only)
#[update]
pub async fn update_platform_config(config: PlatformConfig) -> Result<()> {
    guard::assert_admin()?;
    PlatformConfigStorage::update(config)
}

/// Gets trading metrics
#[query]
pub fn get_trading_metrics() -> TradingMetrics {
    TradingMetricsStorage::get()
}

/// Gets storage statistics
#[query]
pub fn get_storage_stats() -> HashMap<String, u64> {
    storage::get_storage_stats()
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  HELPER FUNCTIONS                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

pub fn validate_ustbill_data(data: &USTBillCreateRequest) -> Result<()> {
    if data.cusip.is_empty() {
        return Err(BitcoinUSTBillsError::InvalidCUSIP);
    }

    if data.annual_yield < 0.0 || data.annual_yield > 1.0 {
        return Err(BitcoinUSTBillsError::InvalidYieldRate);
    }

    if data.total_tokens == 0 {
        return Err(BitcoinUSTBillsError::InvalidTokenAmount);
    }

    let current_time = get_current_timestamp();
    if data.maturity_date <= current_time {
        return Err(BitcoinUSTBillsError::InvalidDate);
    }

    Ok(())
}

pub fn validate_user_data(data: &UserRegistrationRequest) -> Result<()> {
    if data.email.is_empty() || !data.email.contains('@') {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid email format",
        ));
    }

    if data.country.is_empty() {
        return Err(BitcoinUSTBillsError::validation_error(
            "Country is required",
        ));
    }

    Ok(())
}

pub fn calculate_purchase_cost_internal(ustbill: &USTBill, token_amount: u64) -> Result<u64> {
    if token_amount == 0 {
        return Err(BitcoinUSTBillsError::InvalidTokenAmount);
    }

    let cost_per_token = ustbill.purchase_price / ustbill.total_tokens;
    Ok(cost_per_token * token_amount)
}

pub fn calculate_projected_yield(ustbill: &USTBill, investment: u64) -> u64 {
    let days_to_maturity = ustbill.days_to_maturity();
    (investment as f64 * ustbill.annual_yield * days_to_maturity as f64 / 365.0) as u64
}

fn parse_treasury_response(response: HttpResponse) -> Result<Vec<TreasuryRate>> {
    let body = String::from_utf8(response.body)
        .map_err(|_| BitcoinUSTBillsError::external_api_error("Invalid response body"))?;

    let api_response: TreasuryApiResponse = serde_json::from_str(&body).map_err(|_| {
        BitcoinUSTBillsError::external_api_error("Failed to parse Treasury API response")
    })?;

    Ok(api_response.data)
}

#[query]
fn transform_treasury_response(response: TransformArgs) -> HttpResponse {
    let mut res = response.response;
    res.headers.clear();
    res
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                  LEGACY FUNCTIONS (for backward compatibility)                        ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

#[ic_cdk::query]
fn get_principal_data() -> Result<String> {
    let principal = ic_cdk::api::msg_caller();
    match UserStorage::get(&principal) {
        Ok(user) => Ok(user.email),
        Err(_) => Err(BitcoinUSTBillsError::DidntFindUserData),
    }
}

#[ic_cdk::update]
fn set_principal_data(s: String) -> Result<()> {
    let principal = ic_cdk::api::msg_caller();
    match UserStorage::get(&principal) {
        Ok(mut user) => {
            user.email = s;
            UserStorage::update(user)?;
            Ok(())
        }
        Err(_) => Err(BitcoinUSTBillsError::DidntFindUserData),
    }
}

#[update]
pub fn test_func() -> Result<String> {
    guard::assert_admin()?;
    Ok("test_func with admin guard".to_string())
}

#[update]
pub fn add_to_list(p: Principal) -> Result<()> {
    guard::assert_admin()?;
    guard::add_to_list(p);
    Ok(())
}

#[test]
fn generate_candid() {
    candid::export_service!();
    std::fs::write("../distributed/backend/backend.did", __export_service())
        .expect("Failed to write backend.did");
}
