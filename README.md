
<p align="center">
  <img src="static/logo.png" alt="BitcoinUSTbills Logo" width="120"/>
</p>

# BitcoinUSTbills

<p align="center">
  <b>Decentralized US Treasury Bills Platform</b><br>
  <i>Tokenize, trade, and earn yield on US Treasury Bills with Bitcoin-backed security on the Internet Computer.</i>
</p>

<p align="center">
  <a href="https://github.com/your-org/BitcoinUSTbills/actions"><img src="https://img.shields.io/github/workflow/status/your-org/BitcoinUSTbills/CI" alt="CI Status"></a>
  <a href="#"><img src="https://img.shields.io/badge/IC-Internet%20Computer-blue" alt="Internet Computer"></a>
  <a href="#"><img src="https://img.shields.io/badge/Rust-Backend-orange" alt="Rust Backend"></a>
  <a href="#"><img src="https://img.shields.io/badge/Svelte-Frontend-red" alt="Svelte Frontend"></a>
</p>

A comprehensive decentralized platform for tokenizing US Treasury Bills on the Internet Computer Protocol, enabling fractional ownership starting from $1. <br>
<b>Accessible. Secure. Global.</b>


---

## 🚀 Overview

BitcoinUSTbills revolutionizes access to US Treasury Bills by:
- **Fractional Ownership**: Invest in T-Bills starting from just $1
- **Decentralized Trading**: Trade T-Bill tokens 24/7 on the Internet Computer
- **Global Access**: Worldwide access to Bitcoin-backed treasury investments
- **Transparent Yields**: Real-time yield calculations and distributions
- **Secure Storage**: Persistent data storage using IC stable structures


## 🏗️ Architecture

The platform is built using a multi-canister architecture on the Internet Computer:

<details>
<summary><b>Architecture Diagram</b> (click to expand)</summary>

```
┌────────────────────┐      ┌────────────────────┐
│  User Canister    │◄────►│  USTBills Canister │
└────────────────────┘      └────────────────────┘
         ▲                          ▲
         │                          │
         ▼                          ▼
┌────────────────────┐      ┌────────────────────┐
│ Trading Canister   │      │ Yield Canister     │
└────────────────────┘      └────────────────────┘
         │                          │
         ▼                          ▼
         └─────► Storage (IC Stable Structures)
```

</details>

### Core Canisters
- **USTBills Canister**: Manages T-Bill data and operations
- **User Canister**: Handles user registration, KYC, and wallet management
- **Trading Canister**: Processes buy/sell transactions
- **Yield Canister**: Calculates and distributes yields
- **Storage**: Persistent data layer using IC stable structures

### Key Components
- **Rust Backend**: High-performance canister implementation
- **Stable Storage**: Persistent data using `ic-stable-structures`
- **HTTP Outcalls**: Real-time Treasury data integration
- **Access Control**: Role-based security system
- **Comprehensive Testing**: Unit and integration tests
## ⚡ Quickstart

```bash
# 1. Install Rust and DFX
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -ci "$(DFX_VERSION=0.17.0; curl -fsSL https://internetcomputer.org/install.sh)"

# 2. Clone the repository
git clone https://github.com/your-org/BitcoinUSTbills.git
cd BitcoinUSTbills

# 3. Start local replica and deploy
dfx start --clean
dfx deploy
```



## 📊 Data Structures

### USTBill
```rust
pub struct USTBill {
    pub id: String,
    pub cusip: String,
    pub face_value: u64,        // In cents
    pub purchase_price: u64,    // In cents
    pub maturity_date: u64,     // Unix timestamp
    pub annual_yield: f64,      // Decimal (0.0526 = 5.26%)
    pub total_tokens: u64,      // Total tokenized shares
    pub tokens_sold: u64,       // Tokens already sold
    pub status: USTBillStatus,  // Active, SoldOut, Matured, Cancelled
    // ... additional fields
}
```

### User
```rust
pub struct User {
    pub principal: Principal,
    pub email: String,
    pub kyc_status: KYCStatus,  // Pending, Verified, Rejected, Expired
    pub wallet_balance: u64,    // In cents
    pub total_invested: u64,    // Total investment amount
    pub total_yield_earned: u64, // Total yield earned
    // ... additional fields
}
```

### TokenHolding
```rust
pub struct TokenHolding {
    pub id: String,
    pub user_principal: Principal,
    pub ustbill_id: String,
    pub tokens_owned: u64,
    pub purchase_price_per_token: u64,
    pub yield_option: YieldOption, // Maturity, Flexible
    pub status: HoldingStatus,     // Active, Sold, Matured
    // ... additional fields
}
```


## 🔧 Core Functions

### USTBill Management
- `create_ustbill()` - Create new T-Bill offerings (admin only)
- `get_ustbill()` - Retrieve T-Bill details
- `get_active_ustbills()` - List all active T-Bills
- `get_ustbill_availability()` - Check available tokens

### User Management
- `register_user()` - Register new users
- `update_kyc_status()` - Update KYC verification (admin only)
- `deposit_funds()` - Add funds to wallet
- `withdraw_funds()` - Withdraw funds from wallet

### Trading Operations
- `buy_ustbill_tokens()` - Purchase T-Bill tokens
- `sell_ustbill_tokens()` - Sell T-Bill tokens (coming soon)
- `calculate_purchase_cost()` - Calculate token purchase cost
- `get_user_holdings()` - Retrieve user's holdings

### Yield Management
- `calculate_maturity_yield()` - Calculate yield at maturity
- `get_yield_projection()` - Project future yields
- `distribute_maturity_yield()` - Distribute yields (coming soon)

### External Integration
- `fetch_treasury_rates()` - Get real-time Treasury rates
- `update_ustbill_market_data()` - Sync with external data


## 🛠️ Technical Implementation

### Dependencies
```toml
[dependencies]
candid = "0.10.4"
ic-cdk = "0.13.1"
ic-stable-structures = "0.6.4"
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.6.1", features = ["v4"] }
sha2 = "0.10.8"
```

### Storage System
- **Stable BTreeMaps**: For persistent key-value storage
- **Memory Management**: Efficient memory allocation
- **Data Persistence**: Survives canister upgrades
- **Performance Optimized**: Fast query and update operations

### Security Features
- **Access Control**: Role-based permissions
- **KYC Verification**: User identity validation
- **Principal Validation**: Anonymous caller prevention
- **Input Validation**: Comprehensive data validation
- **Error Handling**: Detailed error reporting


## 📱 API Endpoints

### Query Functions (Fast, Read-only)
- `get_ustbill(ustbill_id: String) -> Result<USTBill>`
- `get_active_ustbills() -> Vec<USTBill>`
- `get_user_profile(principal: Principal) -> Result<User>`
- `calculate_purchase_cost(ustbill_id: String, token_amount: u64) -> Result<u64>`
- `get_yield_projection(holding_id: String) -> Result<YieldProjection>`

### Update Functions (State-changing)
- `register_user(user_data: UserRegistrationRequest) -> Result<User>`
- `buy_ustbill_tokens(ustbill_id: String, token_amount: u64) -> Result<TokenHolding>`
- `deposit_funds(amount: u64) -> Result<u64>`
- `withdraw_funds(amount: u64) -> Result<u64>`

### Admin Functions
- `create_ustbill(ustbill_data: USTBillCreateRequest) -> Result<USTBill>`
- `update_kyc_status(principal: Principal, status: KYCStatus) -> Result<()>`
- `update_platform_config(config: PlatformConfig) -> Result<()>`


## 💰 Economic Model

### Platform Configuration
```rust
pub struct PlatformConfig {
    pub platform_fee_percentage: f64,    // 0.5%
    pub minimum_investment: u64,          // $1 (100 cents)
    pub maximum_investment: u64,          // $10,000 (1,000,000 cents)
    pub yield_distribution_frequency: u64, // Daily
    pub kyc_expiry_days: u64,            // 365 days
}
```

### Fee Structure
- **Platform Fee**: 0.5% on purchases
- **Minimum Investment**: $1 USD
- **Maximum Investment**: $10,000 USD per transaction

### Yield Calculations
- **Simple Interest**: For short-term projections
- **Compound Interest**: For long-term holdings
- **Daily Accrual**: Real-time yield tracking


## 🧪 Testing

### Comprehensive Test Suite
- **Unit Tests**: Individual function testing
- **Integration Tests**: End-to-end workflows
- **Validation Tests**: Input validation coverage
- **Error Handling**: Error scenario testing

### Test Coverage
- USTBill creation and management
- User registration and KYC
- Trading operations
- Yield calculations
- Utility functions
- Error handling

Run tests with:
```bash
cargo test
```


## 🚀 Deployment

### Local Development
```bash
# Start local replica
dfx start --clean

# Deploy canisters
dfx deploy

# Generate candid files
dfx generate
```

### Production Deployment
```bash
# Deploy to IC mainnet
dfx deploy --network ic

# Verify deployment
dfx canister --network ic status ustbills_backend
```

### Configuration Files
- `dfx.json` - Multi-canister deployment configuration
- `Cargo.toml` - Rust dependencies and build settings
- `.env` - Environment variables


## 📊 Monitoring & Analytics

### Storage Statistics
- Total users registered
- Active T-Bills count
- Total trading volume
- Platform fees collected

### Trading Metrics
- Transaction volume
- Average prices
- Yield distributions
- User activity


## 🔒 Security Considerations

### Access Control
- Admin-only functions protected
- User authentication required
- Principal validation
- Anonymous caller prevention

### Data Validation
- CUSIP format validation
- Email format checking
- Phone number validation
- Investment limit enforcement

### Error Handling
- Comprehensive error types
- Detailed error messages
- Graceful failure handling
- User-friendly responses


## 🗺️ Roadmap

### Phase 1: Core Platform ✅
- [x] Data structures and storage
- [x] User registration and management
- [x] T-Bill creation and management
- [x] Basic trading functionality

### Phase 2: Advanced Features 🔄
- [ ] Secondary market trading
- [ ] Automated yield distribution
- [ ] Advanced portfolio analytics
- [ ] Mobile app integration

### Phase 3: Scale & Optimize 📈
- [ ] Performance optimization
- [ ] Advanced security features
- [ ] Compliance integration
- [ ] Global market expansion


## 📚 Documentation

### API Documentation
- Candid interface files
- Function signatures
- Parameter descriptions
- Return type definitions

### Developer Guide
- Setup instructions
- Code examples
- Best practices
- Troubleshooting


## 🤝 Contributing

### How to Contribute
We welcome contributions from the community! To get started:

1. Fork the repository and create your branch from `main`.
2. Ensure your code follows Rust and Svelte best practices.
3. Write tests for new features and bug fixes.
4. Submit a pull request with a clear description of your changes.

For major changes, please open an issue first to discuss what you would like to change.

### Code of Conduct
Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

### Development Setup
1. Install Rust and dfx
2. Clone the repository
3. Run `dfx start`
4. Deploy with `dfx deploy`

### Code Standards
- Rust best practices
- Comprehensive testing
- Clear documentation
- Security-first approach



## 🆘 Support


For support and questions:
- [GitHub Issues](https://github.com/your-org/BitcoinUSTbills/issues)
- Community Discord
- Documentation Wiki
- Developer Forums

---


---

<p align="center">
  <b>Built with ❤️ on the Internet Computer</b><br>
  <i>Making US Treasury Bills accessible to everyone, everywhere.</i>
</p>

