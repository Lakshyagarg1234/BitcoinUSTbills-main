use crate::errors::{BitcoinUSTBillsError, Result};
use crate::types::*;
use candid::Principal;
use hex;
use ic_cdk::api::time;
use sha2::{Digest, Sha256};

/// Validates CUSIP format and checksum
pub fn validate_cusip(cusip: &str) -> Result<()> {
    if cusip.len() != 9 {
        return Err(BitcoinUSTBillsError::InvalidCUSIP);
    }

    // Check if first 8 characters are alphanumeric
    let base = &cusip[..8];
    if !base.chars().all(|c| c.is_alphanumeric()) {
        return Err(BitcoinUSTBillsError::InvalidCUSIP);
    }

    // Validate checksum digit
    let check_digit = cusip.chars().nth(8).unwrap();
    if !check_digit.is_ascii_digit() {
        return Err(BitcoinUSTBillsError::InvalidCUSIP);
    }

    let calculated_check_digit = calculate_cusip_check_digit(base)?;
    if check_digit != calculated_check_digit {
        return Err(BitcoinUSTBillsError::InvalidCUSIP);
    }

    Ok(())
}

/// Calculates CUSIP check digit
fn calculate_cusip_check_digit(base: &str) -> Result<char> {
    let mut sum = 0;

    for (i, c) in base.chars().enumerate() {
        let mut value = if c.is_ascii_digit() {
            c.to_digit(10).unwrap() as u32
        } else {
            (c.to_ascii_uppercase() as u32) - ('A' as u32) + 10
        };

        // Multiply by 2 if position is odd (1-indexed)
        if (i + 1) % 2 == 0 {
            value *= 2;
        }

        sum += value;
    }

    let check_digit = (10 - (sum % 10)) % 10;
    Ok(std::char::from_digit(check_digit, 10).unwrap())
}

/// Validates email format
pub fn validate_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(BitcoinUSTBillsError::validation_error(
            "Email cannot be empty",
        ));
    }

    if !email.contains('@') {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid email format",
        ));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid email format",
        ));
    }

    let (local, domain) = (parts[0], parts[1]);

    if local.is_empty() || domain.is_empty() {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid email format",
        ));
    }

    if !domain.contains('.') {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid email domain",
        ));
    }

    Ok(())
}

/// Validates phone number format
pub fn validate_phone_number(phone: &str) -> Result<()> {
    if phone.is_empty() {
        return Ok(()); // Phone number is optional
    }

    // Remove common formatting characters
    let cleaned: String = phone
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();

    if cleaned.len() < 10 || cleaned.len() > 15 {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid phone number length",
        ));
    }

    // Check if starts with + for international numbers
    if cleaned.starts_with('+') {
        if cleaned.len() < 11 {
            return Err(BitcoinUSTBillsError::validation_error(
                "Invalid international phone number",
            ));
        }
    }

    Ok(())
}

/// Validates yield rate
pub fn validate_yield_rate(rate: f64) -> Result<()> {
    if rate < 0.0 {
        return Err(BitcoinUSTBillsError::InvalidYieldRate);
    }

    if rate > 1.0 {
        return Err(BitcoinUSTBillsError::InvalidYieldRate);
    }

    Ok(())
}

/// Validates maturity date
pub fn validate_maturity_date(maturity_date: u64) -> Result<()> {
    let current_time = get_current_timestamp();

    if maturity_date <= current_time {
        return Err(BitcoinUSTBillsError::InvalidDate);
    }

    // Check if maturity date is not too far in the future (e.g., 5 years)
    let max_maturity = current_time + (5 * 365 * 24 * 60 * 60); // 5 years in seconds
    if maturity_date > max_maturity {
        return Err(BitcoinUSTBillsError::InvalidDate);
    }

    Ok(())
}

/// Validates token amount
pub fn validate_token_amount(amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(BitcoinUSTBillsError::InvalidTokenAmount);
    }

    // Check reasonable upper limit
    if amount > 1_000_000_000 {
        return Err(BitcoinUSTBillsError::InvalidTokenAmount);
    }

    Ok(())
}

/// Validates investment amount against platform limits
pub fn validate_investment_amount(amount: u64, config: &PlatformConfig) -> Result<()> {
    if amount < config.minimum_investment {
        return Err(BitcoinUSTBillsError::MinimumInvestmentNotMet);
    }

    if amount > config.maximum_investment {
        return Err(BitcoinUSTBillsError::MaximumInvestmentExceeded);
    }

    Ok(())
}

/// Gets current timestamp in seconds
pub fn get_current_timestamp() -> u64 {
    time() / 1_000_000_000
}

/// Converts timestamp to days
pub fn timestamp_to_days(timestamp: u64) -> u64 {
    timestamp / 86400
}

/// Calculates days between two timestamps
pub fn days_between(start: u64, end: u64) -> u64 {
    if end > start {
        (end - start) / 86400
    } else {
        0
    }
}

/// Calculates compound interest
pub fn calculate_compound_interest(principal: u64, rate: f64, time_in_days: u64) -> u64 {
    let time_in_years = time_in_days as f64 / 365.0;
    let amount = principal as f64 * (1.0 + rate).powf(time_in_years);
    amount as u64
}

/// Calculates simple interest
pub fn calculate_simple_interest(principal: u64, rate: f64, time_in_days: u64) -> u64 {
    let time_in_years = time_in_days as f64 / 365.0;
    let interest = principal as f64 * rate * time_in_years;
    interest as u64
}

/// Calculates daily interest
pub fn calculate_daily_interest(principal: u64, annual_rate: f64, days: u64) -> u64 {
    let daily_rate = annual_rate / 365.0;
    let interest = principal as f64 * daily_rate * days as f64;
    interest as u64
}

/// Formats amount in cents to dollar string
pub fn format_amount_to_dollars(amount_cents: u64) -> String {
    let dollars = amount_cents / 100;
    let cents = amount_cents % 100;
    format!("${}.{:02}", dollars, cents)
}

/// Parses dollar string to amount in cents
pub fn parse_dollar_amount(amount_str: &str) -> Result<u64> {
    let cleaned = amount_str.trim_start_matches('$');

    if let Ok(amount) = cleaned.parse::<f64>() {
        Ok((amount * 100.0) as u64)
    } else {
        Err(BitcoinUSTBillsError::validation_error(
            "Invalid amount format",
        ))
    }
}

/// Generates a hash for data integrity
pub fn generate_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Generates a unique identifier
pub fn generate_unique_id(prefix: &str) -> String {
    let timestamp = get_current_timestamp();
    let caller = ic_cdk::api::msg_caller().to_text();
    let data = format!("{}_{}_{}", prefix, timestamp, caller);
    let hash = generate_hash(&data);
    format!("{}_{}", prefix, &hash[..8])
}

/// Calculates percentage
pub fn calculate_percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// Rounds to specified decimal places
pub fn round_to_decimal_places(value: f64, places: u32) -> f64 {
    let multiplier = 10_f64.powi(places as i32);
    (value * multiplier).round() / multiplier
}

/// Validates principal is not anonymous
pub fn validate_principal(principal: &Principal) -> Result<()> {
    if *principal == Principal::anonymous() {
        return Err(BitcoinUSTBillsError::AnonymousCaller);
    }
    Ok(())
}

/// Validates country code
pub fn validate_country(country: &str) -> Result<()> {
    if country.is_empty() {
        return Err(BitcoinUSTBillsError::validation_error(
            "Country is required",
        ));
    }

    // Simple validation - country should be 2-3 characters
    if country.len() < 2 || country.len() > 3 {
        return Err(BitcoinUSTBillsError::validation_error(
            "Invalid country code",
        ));
    }

    Ok(())
}

/// Calculates fees based on amount and rate
pub fn calculate_fees(amount: u64, fee_rate: f64) -> u64 {
    (amount as f64 * fee_rate) as u64
}

/// Validates that a string is not empty and doesn't contain only whitespace
pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(BitcoinUSTBillsError::validation_error(&format!(
            "{} cannot be empty",
            field_name
        )));
    }
    Ok(())
}

/// Checks if a date is in the past
pub fn is_date_in_past(date: u64) -> bool {
    date < get_current_timestamp()
}

/// Checks if a date is within a certain range from now
pub fn is_date_within_range(date: u64, max_days_from_now: u64) -> bool {
    let current_time = get_current_timestamp();
    let max_time = current_time + (max_days_from_now * 86400);
    date <= max_time
}

/// Converts basis points to percentage
pub fn basis_points_to_percentage(basis_points: u64) -> f64 {
    basis_points as f64 / 10000.0
}

/// Converts percentage to basis points
pub fn percentage_to_basis_points(percentage: f64) -> u64 {
    (percentage * 10000.0) as u64
}

/// Sanitizes string input
pub fn sanitize_string(input: &str) -> String {
    input.trim().to_string()
}

/// Validates that an amount is positive
pub fn validate_positive_amount(amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(BitcoinUSTBillsError::InvalidAmount);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cusip() {
        assert!(validate_cusip("912796RF6").is_ok());
        assert!(validate_cusip("12345678").is_err());
        assert!(validate_cusip("").is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_calculate_simple_interest() {
        let principal = 100000; // $1000
        let rate = 0.05; // 5%
        let days = 365; // 1 year
        let interest = calculate_simple_interest(principal, rate, days);
        assert_eq!(interest, 5000); // $50
    }

    #[test]
    fn test_format_amount_to_dollars() {
        assert_eq!(format_amount_to_dollars(100000), "$1000.00");
        assert_eq!(format_amount_to_dollars(150), "$1.50");
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(50, 200), 25.0);
        assert_eq!(calculate_percentage(0, 100), 0.0);
        assert_eq!(calculate_percentage(100, 0), 0.0);
    }
}
