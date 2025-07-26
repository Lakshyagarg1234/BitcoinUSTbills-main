use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, CandidType, Clone, Serialize, Deserialize)]
pub enum BitcoinUSTBillsError {
    // User-related errors
    UserNotFound,
    UserAlreadyExists,
    KYCNotVerified,
    KYCExpired,
    InvalidUserData,
    
    // USTBill-related errors
    USTBillNotFound,
    USTBillAlreadyExists,
    USTBillSoldOut,
    USTBillMatured,
    USTBillCancelled,
    InvalidUSTBillData,
    
    // Trading-related errors
    InsufficientFunds,
    InsufficientTokens,
    InvalidAmount,
    MinimumInvestmentNotMet,
    MaximumInvestmentExceeded,
    TradingNotAllowed,
    
    // Holdings-related errors
    HoldingNotFound,
    HoldingAlreadySold,
    HoldingMatured,
    InvalidHoldingData,
    
    // Transaction-related errors
    TransactionNotFound,
    TransactionFailed,
    TransactionCancelled,
    InvalidTransactionType,
    
    // Platform errors
    PlatformConfigurationError,
    PlatformFeesCalculationError,
    
    // External API errors
    ExternalAPIError(String),
    TreasuryDataFetchError,
    HTTPRequestError(String),
    
    // Storage & Database errors
    DatabaseError(String),
    StorageError(String),
    SerializationError(String),
    
    // Authentication & Authorization errors
    Unauthorized,
    AnonymousCaller,
    InvalidPrincipal,
    AccessDenied,
    
    // Validation errors
    ValidationError(String),
    InvalidCUSIP,
    InvalidDate,
    InvalidYieldRate,
    InvalidTokenAmount,
    
    // System errors
    SystemError(String),
    InternalError(String),
    NotImplemented,
    
    // Yield calculation errors
    YieldCalculationError,
    YieldDistributionError,
    MaturityDatePassed,
    
    // Legacy errors (keeping for compatibility)
    DidntFindUserData,
    FailedToAddToList,
}

impl std::fmt::Display for BitcoinUSTBillsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // User-related errors
            BitcoinUSTBillsError::UserNotFound => write!(f, "User not found"),
            BitcoinUSTBillsError::UserAlreadyExists => write!(f, "User already exists"),
            BitcoinUSTBillsError::KYCNotVerified => write!(f, "KYC verification required"),
            BitcoinUSTBillsError::KYCExpired => write!(f, "KYC verification has expired"),
            BitcoinUSTBillsError::InvalidUserData => write!(f, "Invalid user data provided"),
            
            // USTBill-related errors
            BitcoinUSTBillsError::USTBillNotFound => write!(f, "US Treasury Bill not found"),
            BitcoinUSTBillsError::USTBillAlreadyExists => write!(f, "US Treasury Bill already exists"),
            BitcoinUSTBillsError::USTBillSoldOut => write!(f, "US Treasury Bill is sold out"),
            BitcoinUSTBillsError::USTBillMatured => write!(f, "US Treasury Bill has already matured"),
            BitcoinUSTBillsError::USTBillCancelled => write!(f, "US Treasury Bill has been cancelled"),
            BitcoinUSTBillsError::InvalidUSTBillData => write!(f, "Invalid US Treasury Bill data"),
            
            // Trading-related errors
            BitcoinUSTBillsError::InsufficientFunds => write!(f, "Insufficient funds in wallet"),
            BitcoinUSTBillsError::InsufficientTokens => write!(f, "Insufficient tokens for transaction"),
            BitcoinUSTBillsError::InvalidAmount => write!(f, "Invalid transaction amount"),
            BitcoinUSTBillsError::MinimumInvestmentNotMet => write!(f, "Minimum investment amount not met"),
            BitcoinUSTBillsError::MaximumInvestmentExceeded => write!(f, "Maximum investment amount exceeded"),
            BitcoinUSTBillsError::TradingNotAllowed => write!(f, "Trading not allowed for this user"),
            
            // Holdings-related errors
            BitcoinUSTBillsError::HoldingNotFound => write!(f, "Token holding not found"),
            BitcoinUSTBillsError::HoldingAlreadySold => write!(f, "Token holding already sold"),
            BitcoinUSTBillsError::HoldingMatured => write!(f, "Token holding has matured"),
            BitcoinUSTBillsError::InvalidHoldingData => write!(f, "Invalid holding data"),
            
            // Transaction-related errors
            BitcoinUSTBillsError::TransactionNotFound => write!(f, "Transaction not found"),
            BitcoinUSTBillsError::TransactionFailed => write!(f, "Transaction failed"),
            BitcoinUSTBillsError::TransactionCancelled => write!(f, "Transaction was cancelled"),
            BitcoinUSTBillsError::InvalidTransactionType => write!(f, "Invalid transaction type"),
            
            // Platform errors
            BitcoinUSTBillsError::PlatformConfigurationError => write!(f, "Platform configuration error"),
            BitcoinUSTBillsError::PlatformFeesCalculationError => write!(f, "Platform fees calculation error"),
            
            // External API errors
            BitcoinUSTBillsError::ExternalAPIError(msg) => write!(f, "External API error: {}", msg),
            BitcoinUSTBillsError::TreasuryDataFetchError => write!(f, "Failed to fetch Treasury data"),
            BitcoinUSTBillsError::HTTPRequestError(msg) => write!(f, "HTTP request error: {}", msg),
            
            // Storage & Database errors
            BitcoinUSTBillsError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            BitcoinUSTBillsError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            BitcoinUSTBillsError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            
            // Authentication & Authorization errors
            BitcoinUSTBillsError::Unauthorized => write!(f, "Unauthorized access"),
            BitcoinUSTBillsError::AnonymousCaller => write!(f, "Anonymous caller not allowed"),
            BitcoinUSTBillsError::InvalidPrincipal => write!(f, "Invalid principal"),
            BitcoinUSTBillsError::AccessDenied => write!(f, "Access denied"),
            
            // Validation errors
            BitcoinUSTBillsError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            BitcoinUSTBillsError::InvalidCUSIP => write!(f, "Invalid CUSIP identifier"),
            BitcoinUSTBillsError::InvalidDate => write!(f, "Invalid date format or value"),
            BitcoinUSTBillsError::InvalidYieldRate => write!(f, "Invalid yield rate"),
            BitcoinUSTBillsError::InvalidTokenAmount => write!(f, "Invalid token amount"),
            
            // System errors
            BitcoinUSTBillsError::SystemError(msg) => write!(f, "System error: {}", msg),
            BitcoinUSTBillsError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            BitcoinUSTBillsError::NotImplemented => write!(f, "Functionality not implemented"),
            
            // Yield calculation errors
            BitcoinUSTBillsError::YieldCalculationError => write!(f, "Yield calculation error"),
            BitcoinUSTBillsError::YieldDistributionError => write!(f, "Yield distribution error"),
            BitcoinUSTBillsError::MaturityDatePassed => write!(f, "Maturity date has already passed"),
            
            // Legacy errors
            BitcoinUSTBillsError::DidntFindUserData => write!(f, "User data not found"),
            BitcoinUSTBillsError::FailedToAddToList => write!(f, "Failed to add to list"),
        }
    }
}

impl std::error::Error for BitcoinUSTBillsError {}

// Result type alias for convenience
pub type Result<T> = std::result::Result<T, BitcoinUSTBillsError>;

// Legacy error type for backward compatibility
#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum GetUserDataError {
    AnonymousCaller,
    DidntFindUserData,
    FailedToAddToList,
}

impl From<GetUserDataError> for BitcoinUSTBillsError {
    fn from(error: GetUserDataError) -> Self {
        match error {
            GetUserDataError::AnonymousCaller => BitcoinUSTBillsError::AnonymousCaller,
            GetUserDataError::DidntFindUserData => BitcoinUSTBillsError::DidntFindUserData,
            GetUserDataError::FailedToAddToList => BitcoinUSTBillsError::FailedToAddToList,
        }
    }
}

// Helper functions for error creation
impl BitcoinUSTBillsError {
    pub fn external_api_error(msg: impl Into<String>) -> Self {
        BitcoinUSTBillsError::ExternalAPIError(msg.into())
    }
    
    pub fn database_error(msg: impl Into<String>) -> Self {
        BitcoinUSTBillsError::DatabaseError(msg.into())
    }
    
    pub fn validation_error(msg: impl Into<String>) -> Self {
        BitcoinUSTBillsError::ValidationError(msg.into())
    }
    
    pub fn system_error(msg: impl Into<String>) -> Self {
        BitcoinUSTBillsError::SystemError(msg.into())
    }
    
    pub fn http_request_error(msg: impl Into<String>) -> Self {
        BitcoinUSTBillsError::HTTPRequestError(msg.into())
    }
}
