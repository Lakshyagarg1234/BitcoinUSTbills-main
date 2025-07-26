use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

use crate::errors::{BitcoinUSTBillsError, Result};
use crate::types::*;

// Memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Memory IDs for different data types
const USTBILLS_MEMORY_ID: MemoryId = MemoryId::new(0);
const USERS_MEMORY_ID: MemoryId = MemoryId::new(1);
const HOLDINGS_MEMORY_ID: MemoryId = MemoryId::new(2);
const TRANSACTIONS_MEMORY_ID: MemoryId = MemoryId::new(3);
const PLATFORM_CONFIG_MEMORY_ID: MemoryId = MemoryId::new(4);
const TREASURY_RATES_MEMORY_ID: MemoryId = MemoryId::new(5);
const TRADING_METRICS_MEMORY_ID: MemoryId = MemoryId::new(6);
const ID_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(7);
const VERIFIED_PURCHASES_LEDGER_MEMORY_ID: MemoryId = MemoryId::new(8);

// Thread-local storage for memory manager and stable data structures
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(ID_COUNTER_MEMORY_ID)), 0)
    );

    static USTBILLS: RefCell<StableBTreeMap<String, USTBill, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USTBILLS_MEMORY_ID))
        )
    );

    static USERS: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEMORY_ID))
        )
    );

    static HOLDINGS: RefCell<StableBTreeMap<String, TokenHolding, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(HOLDINGS_MEMORY_ID))
        )
    );

    static TRANSACTIONS: RefCell<StableBTreeMap<String, Transaction, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTIONS_MEMORY_ID))
        )
    );

    static PLATFORM_CONFIG: RefCell<Cell<PlatformConfig, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(PLATFORM_CONFIG_MEMORY_ID)),
            PlatformConfig::default()
        )
    );

    static TREASURY_RATES: RefCell<StableBTreeMap<String, TreasuryRate, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TREASURY_RATES_MEMORY_ID))
        )
    );

    static TRADING_METRICS: RefCell<Cell<TradingMetrics, Memory>> = RefCell::new(
        Cell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(TRADING_METRICS_MEMORY_ID)),
            TradingMetrics {
                total_volume: 0,
                total_transactions: 0,
                average_price: 0,
                highest_price: 0,
                lowest_price: 0,
                last_updated: 0,
            }
        )
    );

    static VERIFIED_PURCHASES_LEDGER: RefCell<StableBTreeMap<u64, VerifiedBrokerPurchase, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(VERIFIED_PURCHASES_LEDGER_MEMORY_ID))
        )
    );
}

// Implement Storable for our custom types
impl Storable for USTBill {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for TokenHolding {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for Transaction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for PlatformConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for TreasuryRate {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for TradingMetrics {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

impl Storable for VerifiedBrokerPurchase {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

// Storage interface for USTBills
pub struct USTBillStorage;

impl USTBillStorage {
    pub fn insert(ustbill: USTBill) -> Result<()> {
        USTBILLS.with(|ustbills| {
            ustbills.borrow_mut().insert(ustbill.id.clone(), ustbill);
            Ok(())
        })
    }

    pub fn get(ustbill_id: &str) -> Result<USTBill> {
        USTBILLS.with(|ustbills| {
            ustbills
                .borrow()
                .get(&ustbill_id.to_string())
                .ok_or(BitcoinUSTBillsError::USTBillNotFound)
        })
    }

    pub fn update(ustbill: USTBill) -> Result<()> {
        USTBILLS.with(|ustbills| {
            let mut ustbills = ustbills.borrow_mut();
            if ustbills.contains_key(&ustbill.id) {
                ustbills.insert(ustbill.id.clone(), ustbill);
                Ok(())
            } else {
                Err(BitcoinUSTBillsError::USTBillNotFound)
            }
        })
    }

    pub fn remove(ustbill_id: &str) -> Result<USTBill> {
        USTBILLS.with(|ustbills| {
            ustbills
                .borrow_mut()
                .remove(&ustbill_id.to_string())
                .ok_or(BitcoinUSTBillsError::USTBillNotFound)
        })
    }

    pub fn get_all() -> Vec<USTBill> {
        USTBILLS.with(|ustbills| {
            ustbills
                .borrow()
                .iter()
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn get_active() -> Vec<USTBill> {
        USTBILLS.with(|ustbills| {
            ustbills
                .borrow()
                .iter()
                .filter(|entry| entry.value().status == USTBillStatus::Active)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn count() -> u64 {
        USTBILLS.with(|ustbills| ustbills.borrow().len())
    }
}

// Storage interface for Users
pub struct UserStorage;

impl UserStorage {
    pub fn insert(user: User) -> Result<()> {
        USERS.with(|users| {
            if users.borrow().contains_key(&user.principal) {
                Err(BitcoinUSTBillsError::UserAlreadyExists)
            } else {
                users.borrow_mut().insert(user.principal, user);
                Ok(())
            }
        })
    }

    pub fn get(principal: &Principal) -> Result<User> {
        USERS.with(|users| {
            users
                .borrow()
                .get(principal)
                .ok_or(BitcoinUSTBillsError::UserNotFound)
        })
    }

    pub fn update(user: User) -> Result<()> {
        USERS.with(|users| {
            let mut users = users.borrow_mut();
            if users.contains_key(&user.principal) {
                users.insert(user.principal, user);
                Ok(())
            } else {
                Err(BitcoinUSTBillsError::UserNotFound)
            }
        })
    }

    pub fn remove(principal: &Principal) -> Result<User> {
        USERS.with(|users| {
            users
                .borrow_mut()
                .remove(principal)
                .ok_or(BitcoinUSTBillsError::UserNotFound)
        })
    }

    pub fn get_all() -> Vec<User> {
        USERS.with(|users| {
            users
                .borrow()
                .iter()
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn count() -> u64 {
        USERS.with(|users| users.borrow().len())
    }
}

// Storage interface for Token Holdings
pub struct HoldingStorage;

impl HoldingStorage {
    pub fn insert(holding: TokenHolding) -> Result<()> {
        HOLDINGS.with(|holdings| {
            holdings.borrow_mut().insert(holding.id.clone(), holding);
            Ok(())
        })
    }

    pub fn get(holding_id: &str) -> Result<TokenHolding> {
        HOLDINGS.with(|holdings| {
            holdings
                .borrow()
                .get(&holding_id.to_string())
                .ok_or(BitcoinUSTBillsError::HoldingNotFound)
        })
    }

    pub fn update(holding: TokenHolding) -> Result<()> {
        HOLDINGS.with(|holdings| {
            let mut holdings = holdings.borrow_mut();
            if holdings.contains_key(&holding.id) {
                holdings.insert(holding.id.clone(), holding);
                Ok(())
            } else {
                Err(BitcoinUSTBillsError::HoldingNotFound)
            }
        })
    }

    pub fn remove(holding_id: &str) -> Result<TokenHolding> {
        HOLDINGS.with(|holdings| {
            holdings
                .borrow_mut()
                .remove(&holding_id.to_string())
                .ok_or(BitcoinUSTBillsError::HoldingNotFound)
        })
    }

    pub fn get_by_user(user_principal: &Principal) -> Vec<TokenHolding> {
        HOLDINGS.with(|holdings| {
            holdings
                .borrow()
                .iter()
                .filter(|entry| entry.value().user_principal == *user_principal)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn get_by_ustbill(ustbill_id: &str) -> Vec<TokenHolding> {
        HOLDINGS.with(|holdings| {
            holdings
                .borrow()
                .iter()
                .filter(|entry| entry.value().ustbill_id == ustbill_id)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn get_active() -> Vec<TokenHolding> {
        HOLDINGS.with(|holdings| {
            holdings
                .borrow()
                .iter()
                .filter(|entry| entry.value().status == HoldingStatus::Active)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn count() -> u64 {
        HOLDINGS.with(|holdings| holdings.borrow().len())
    }
}

// Storage interface for Transactions
pub struct TransactionStorage;

impl TransactionStorage {
    pub fn insert(transaction: Transaction) -> Result<()> {
        TRANSACTIONS.with(|transactions| {
            transactions
                .borrow_mut()
                .insert(transaction.id.clone(), transaction);
            Ok(())
        })
    }

    pub fn get(transaction_id: &str) -> Result<Transaction> {
        TRANSACTIONS.with(|transactions| {
            transactions
                .borrow()
                .get(&transaction_id.to_string())
                .ok_or(BitcoinUSTBillsError::TransactionNotFound)
        })
    }

    pub fn update(transaction: Transaction) -> Result<()> {
        TRANSACTIONS.with(|transactions| {
            let mut transactions = transactions.borrow_mut();
            if transactions.contains_key(&transaction.id) {
                transactions.insert(transaction.id.clone(), transaction);
                Ok(())
            } else {
                Err(BitcoinUSTBillsError::TransactionNotFound)
            }
        })
    }

    pub fn get_by_user(user_principal: &Principal) -> Vec<Transaction> {
        TRANSACTIONS.with(|transactions| {
            transactions
                .borrow()
                .iter()
                .filter(|entry| entry.value().user_principal == *user_principal)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn get_by_type(transaction_type: &TransactionType) -> Vec<Transaction> {
        TRANSACTIONS.with(|transactions| {
            transactions
                .borrow()
                .iter()
                .filter(|entry| entry.value().transaction_type == *transaction_type)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn count() -> u64 {
        TRANSACTIONS.with(|transactions| transactions.borrow().len())
    }
}

// Storage interface for Platform Configuration
pub struct PlatformConfigStorage;

impl PlatformConfigStorage {
    pub fn get() -> PlatformConfig {
        PLATFORM_CONFIG.with(|config| config.borrow().get().clone())
    }

    pub fn update(config: PlatformConfig) -> Result<()> {
        PLATFORM_CONFIG.with(|platform_config| {
            platform_config.borrow_mut().set(config);
            Ok(())
        })
    }
}

// Storage interface for Treasury Rates
pub struct TreasuryRateStorage;

impl TreasuryRateStorage {
    pub fn insert(rate: TreasuryRate) -> Result<()> {
        let key = format!("{}_{}", rate.cusip, rate.rate_date);
        TREASURY_RATES.with(|rates| {
            rates.borrow_mut().insert(key, rate);
            Ok(())
        })
    }

    pub fn get_by_cusip(cusip: &str) -> Vec<TreasuryRate> {
        TREASURY_RATES.with(|rates| {
            rates
                .borrow()
                .iter()
                .filter(|entry| entry.value().cusip == cusip)
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn get_all() -> Vec<TreasuryRate> {
        TREASURY_RATES.with(|rates| {
            rates
                .borrow()
                .iter()
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn clear() -> Result<()> {
        TREASURY_RATES.with(|rates| {
            *rates.borrow_mut() = StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(TREASURY_RATES_MEMORY_ID)),
            );
            Ok(())
        })
    }
}

// Storage interface for Trading Metrics
pub struct TradingMetricsStorage;

impl TradingMetricsStorage {
    pub fn get() -> TradingMetrics {
        TRADING_METRICS.with(|metrics| metrics.borrow().get().clone())
    }

    pub fn update(metrics: TradingMetrics) -> Result<()> {
        TRADING_METRICS.with(|trading_metrics| {
            trading_metrics.borrow_mut().set(metrics);
            Ok(())
        })
    }

    pub fn update_volume(volume: u64) -> Result<()> {
        let mut metrics = Self::get();
        metrics.total_volume += volume;
        metrics.total_transactions += 1;
        metrics.last_updated = ic_cdk::api::time() / 1_000_000_000;
        Self::update(metrics)
    }

    pub fn update_price(price: u64) -> Result<()> {
        let mut metrics = Self::get();
        if metrics.highest_price == 0 || price > metrics.highest_price {
            metrics.highest_price = price;
        }
        if metrics.lowest_price == 0 || price < metrics.lowest_price {
            metrics.lowest_price = price;
        }
        // Calculate new average price
        let total_volume = metrics.total_volume;
        if total_volume > 0 {
            metrics.average_price =
                (metrics.average_price * total_volume + price) / (total_volume + 1);
        } else {
            metrics.average_price = price;
        }
        metrics.last_updated = ic_cdk::api::time() / 1_000_000_000;
        Self::update(metrics)
    }
}

// Storage interface for Verified Purchases Ledger
pub struct VerifiedPurchasesLedgerStorage;

impl VerifiedPurchasesLedgerStorage {
    pub fn insert(purchase: VerifiedBrokerPurchase) -> Result<()> {
        VERIFIED_PURCHASES_LEDGER.with(|ledger| {
            let id = ledger.borrow().len();
            ledger.borrow_mut().insert(id, purchase);
            Ok(())
        })
    }

    pub fn get_all() -> Vec<VerifiedBrokerPurchase> {
        VERIFIED_PURCHASES_LEDGER.with(|ledger| {
            ledger
                .borrow()
                .iter()
                .map(|entry| entry.value().clone())
                .collect()
        })
    }

    pub fn count() -> u64 {
        VERIFIED_PURCHASES_LEDGER.with(|ledger| ledger.borrow().len())
    }
}


// Utility functions for storage operations
pub fn generate_id() -> String {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1);
        current_value.to_string()
    })
}

pub fn get_current_timestamp() -> u64 {
    ic_cdk::api::time() / 1_000_000_000 // Convert from nanoseconds to seconds
}

// Storage statistics
pub fn get_storage_stats() -> std::collections::HashMap<String, u64> {
    let mut stats = std::collections::HashMap::new();
    stats.insert("ustbills".to_string(), USTBillStorage::count());
    stats.insert("users".to_string(), UserStorage::count());
    stats.insert("holdings".to_string(), HoldingStorage::count());
    stats.insert("transactions".to_string(), TransactionStorage::count());
    stats.insert("verified_purchases".to_string(), VerifiedPurchasesLedgerStorage::count());
    stats
}
