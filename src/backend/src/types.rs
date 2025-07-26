use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============= CORE DATA STRUCTURES =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct USTBill {
    pub id: String,
    pub cusip: String,
    pub face_value: u64,        // In cents ($1000 = 100000)
    pub purchase_price: u64,    // In cents ($950 = 95000)
    pub maturity_date: u64,     // Unix timestamp
    pub annual_yield: f64,      // 5.26% = 0.0526
    pub total_tokens: u64,      // 1000 tokens
    pub tokens_sold: u64,       // Tokens already sold
    pub status: USTBillStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub issuer: String,         // Treasury issuer info
    pub bill_type: String,      // 4-week, 13-week, 26-week, 52-week
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum USTBillStatus {
    Active,
    SoldOut,
    Matured,
    Cancelled,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct User {
    pub principal: Principal,
    pub email: String,
    pub kyc_status: KYCStatus,
    pub wallet_balance: u64,    // In cents
    pub total_invested: u64,    // Total amount invested
    pub total_yield_earned: u64, // Total yield earned
    pub created_at: u64,
    pub updated_at: u64,
    pub is_active: bool,
    pub phone_number: Option<String>,
    pub country: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum KYCStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TokenHolding {
    pub id: String,
    pub user_principal: Principal,
    pub ustbill_id: String,
    pub tokens_owned: u64,
    pub purchase_price_per_token: u64,  // In cents
    pub purchase_date: u64,
    pub yield_option: YieldOption,
    pub status: HoldingStatus,
    pub current_value: u64,      // Current market value
    pub projected_yield: u64,    // Projected yield at maturity
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum YieldOption {
    Maturity,    // Hold till maturity (full yield)
    Flexible,    // Can sell anytime (market rate)
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum HoldingStatus {
    Active,
    Sold,
    Matured,
    Cancelled,
}

// ============= REQUEST/RESPONSE STRUCTURES =============

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct USTBillCreateRequest {
    pub cusip: String,
    pub face_value: u64,
    pub purchase_price: u64,
    pub maturity_date: u64,
    pub annual_yield: f64,
    pub total_tokens: u64,
    pub issuer: String,
    pub bill_type: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UserRegistrationRequest {
    pub email: String,
    pub phone_number: Option<String>,
    pub country: String,
}

#[derive(Clone, Debug, CandidType, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub has_next: bool,
}

// ============= YIELD & TRADING STRUCTURES =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct YieldDistribution {
    pub holding_id: String,
    pub user_principal: Principal,
    pub yield_amount: u64,
    pub distribution_date: u64,
    pub ustbill_id: String,
}

#[derive(Clone, Debug, CandidType, Serialize)]
pub struct YieldProjection {
    pub holding_id: String,
    pub current_value: u64,
    pub projected_yield: u64,
    pub yield_percentage: f64,
    pub days_to_maturity: u64,
    pub annual_yield_rate: f64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TradingMetrics {
    pub total_volume: u64,
    pub total_transactions: u64,
    pub average_price: u64,
    pub highest_price: u64,
    pub lowest_price: u64,
    pub last_updated: u64,
}

// ============= EXTERNAL API STRUCTURES =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TreasuryRate {
    pub record_date: String,
    pub security_type: String,
    pub security_desc: String,
    pub rate_date: String,
    pub rate: f64,
    pub cusip: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TreasuryApiResponse {
    pub data: Vec<TreasuryRate>,
    pub meta: TreasuryApiMeta,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TreasuryApiMeta {
    pub count: u64,
    pub labels: HashMap<String, String>,
}

// ============= TRANSACTION STRUCTURES =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub user_principal: Principal,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub ustbill_id: Option<String>,
    pub holding_id: Option<String>,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub fees: u64,
    pub description: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Purchase,
    Sale,
    YieldDistribution,
    Fee,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

// ============= PLATFORM CONFIGURATION =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PlatformConfig {
    pub platform_fee_percentage: f64,  // 0.5% = 0.005
    pub minimum_investment: u64,        // $1 = 100 cents
    pub maximum_investment: u64,        // $10,000 = 1,000,000 cents
    pub yield_distribution_frequency: u64, // Days
    pub kyc_expiry_days: u64,          // 365 days
    pub treasury_api_refresh_interval: u64, // Seconds
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            platform_fee_percentage: 0.005,  // 0.5%
            minimum_investment: 100,          // $1
            maximum_investment: 1_000_000,    // $10,000
            yield_distribution_frequency: 1,  // Daily
            kyc_expiry_days: 365,            // 1 year
            treasury_api_refresh_interval: 3600, // 1 hour
        }
    }
}

// ============= VERIFIED BROKER PURCHASE =============

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct VerifiedBrokerPurchase {
    pub amount: u64,
    pub price: u64,
    pub timestamp: u64,
    pub broker_txn_id: String,
    pub ustbill_type: String,
}

// ============= HELPER FUNCTIONS =============

impl USTBill {
    pub fn available_tokens(&self) -> u64 {
        self.total_tokens - self.tokens_sold
    }
    
    pub fn is_available_for_purchase(&self) -> bool {
        self.status == USTBillStatus::Active && self.available_tokens() > 0
    }
    
    pub fn days_to_maturity(&self) -> u64 {
        let current_time = ic_cdk::api::time() / 1_000_000_000; // Convert to seconds
        if self.maturity_date > current_time {
            (self.maturity_date - current_time) / 86400 // Convert to days
        } else {
            0
        }
    }
}

impl User {
    pub fn is_eligible_for_trading(&self) -> bool {
        self.kyc_status == KYCStatus::Verified && self.is_active
    }
    
    pub fn total_portfolio_value(&self) -> u64 {
        // This will be calculated dynamically by aggregating holdings
        self.total_invested
    }
}

impl TokenHolding {
    pub fn calculate_current_yield(&self, annual_rate: f64, days_held: u64) -> u64 {
        let daily_rate = annual_rate / 365.0;
        let current_value = self.tokens_owned * self.purchase_price_per_token;
        (current_value as f64 * daily_rate * days_held as f64) as u64
    }
    
    pub fn is_active(&self) -> bool {
        self.status == HoldingStatus::Active
    }
} 