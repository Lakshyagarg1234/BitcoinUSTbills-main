import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type BitcoinUSTBillsError = { 'UserAlreadyExists' : null } |
  { 'TradingNotAllowed' : null } |
  { 'AccessDenied' : null } |
  { 'InvalidAmount' : null } |
  { 'TransactionCancelled' : null } |
  { 'MinimumInvestmentNotMet' : null } |
  { 'HoldingNotFound' : null } |
  { 'DatabaseError' : string } |
  { 'YieldCalculationError' : null } |
  { 'FailedToAddToList' : null } |
  { 'TreasuryDataFetchError' : null } |
  { 'TransactionFailed' : null } |
  { 'MaximumInvestmentExceeded' : null } |
  { 'USTBillSoldOut' : null } |
  { 'SystemError' : string } |
  { 'InvalidPrincipal' : null } |
  { 'HTTPRequestError' : string } |
  { 'HoldingAlreadySold' : null } |
  { 'USTBillMatured' : null } |
  { 'ValidationError' : string } |
  { 'ExternalAPIError' : string } |
  { 'InvalidTokenAmount' : null } |
  { 'USTBillCancelled' : null } |
  { 'InsufficientTokens' : null } |
  { 'InvalidDate' : null } |
  { 'Unauthorized' : null } |
  { 'KYCExpired' : null } |
  { 'KYCNotVerified' : null } |
  { 'PlatformConfigurationError' : null } |
  { 'InvalidCUSIP' : null } |
  { 'DidntFindUserData' : null } |
  { 'InvalidHoldingData' : null } |
  { 'USTBillAlreadyExists' : null } |
  { 'InvalidTransactionType' : null } |
  { 'InvalidUserData' : null } |
  { 'YieldDistributionError' : null } |
  { 'TransactionNotFound' : null } |
  { 'NotImplemented' : null } |
  { 'USTBillNotFound' : null } |
  { 'StorageError' : string } |
  { 'InvalidYieldRate' : null } |
  { 'InternalError' : string } |
  { 'InvalidUSTBillData' : null } |
  { 'PlatformFeesCalculationError' : null } |
  { 'SerializationError' : string } |
  { 'UserNotFound' : null } |
  { 'AnonymousCaller' : null } |
  { 'MaturityDatePassed' : null } |
  { 'InsufficientFunds' : null } |
  { 'HoldingMatured' : null };
export type HoldingStatus = { 'Sold' : null } |
  { 'Active' : null } |
  { 'Matured' : null } |
  { 'Cancelled' : null };
export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type KYCStatus = { 'Rejected' : null } |
  { 'Verified' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export interface PaginatedResponse {
  'per_page' : bigint,
  'total' : bigint,
  'data' : Array<USTBill>,
  'page' : bigint,
  'has_next' : boolean,
}
export interface PlatformConfig {
  'minimum_investment' : bigint,
  'kyc_expiry_days' : bigint,
  'platform_fee_percentage' : number,
  'yield_distribution_frequency' : bigint,
  'maximum_investment' : bigint,
  'treasury_api_refresh_interval' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_1 = { 'Ok' : TokenHolding } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_2 = { 'Ok' : bigint } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_3 = { 'Ok' : USTBill } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_4 = { 'Ok' : Array<TreasuryRate> } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_5 = { 'Ok' : string } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_6 = { 'Ok' : User } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_7 = { 'Ok' : PaginatedResponse } |
  { 'Err' : BitcoinUSTBillsError };
export type Result_8 = { 'Ok' : YieldProjection } |
  { 'Err' : BitcoinUSTBillsError };
export interface TokenHolding {
  'id' : string,
  'status' : HoldingStatus,
  'user_principal' : Principal,
  'purchase_price_per_token' : bigint,
  'purchase_date' : bigint,
  'tokens_owned' : bigint,
  'yield_option' : YieldOption,
  'current_value' : bigint,
  'projected_yield' : bigint,
  'ustbill_id' : string,
}
export interface TradingMetrics {
  'average_price' : bigint,
  'last_updated' : bigint,
  'lowest_price' : bigint,
  'total_transactions' : bigint,
  'total_volume' : bigint,
  'highest_price' : bigint,
}
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface TreasuryRate {
  'record_date' : string,
  'rate' : number,
  'cusip' : string,
  'security_desc' : string,
  'security_type' : string,
  'rate_date' : string,
}
export interface USTBill {
  'id' : string,
  'status' : USTBillStatus,
  'updated_at' : bigint,
  'purchase_price' : bigint,
  'face_value' : bigint,
  'cusip' : string,
  'tokens_sold' : bigint,
  'created_at' : bigint,
  'annual_yield' : number,
  'maturity_date' : bigint,
  'issuer' : string,
  'total_tokens' : bigint,
  'bill_type' : string,
}
export interface USTBillCreateRequest {
  'purchase_price' : bigint,
  'face_value' : bigint,
  'cusip' : string,
  'annual_yield' : number,
  'maturity_date' : bigint,
  'issuer' : string,
  'total_tokens' : bigint,
  'bill_type' : string,
}
export type USTBillStatus = { 'Active' : null } |
  { 'SoldOut' : null } |
  { 'Matured' : null } |
  { 'Cancelled' : null };
export interface User {
  'updated_at' : bigint,
  'principal' : Principal,
  'country' : string,
  'created_at' : bigint,
  'email' : string,
  'total_invested' : bigint,
  'kyc_status' : KYCStatus,
  'is_active' : boolean,
  'phone_number' : [] | [string],
  'wallet_balance' : bigint,
  'total_yield_earned' : bigint,
}
export interface UserRegistrationRequest {
  'country' : string,
  'email' : string,
  'phone_number' : [] | [string],
}
export interface VerifiedBrokerPurchase {
  'ustbill_type' : string,
  'broker_txn_id' : string,
  'timestamp' : bigint,
  'price' : bigint,
  'amount' : bigint,
}
export type YieldOption = { 'Maturity' : null } |
  { 'Flexible' : null };
export interface YieldProjection {
  'days_to_maturity' : bigint,
  'holding_id' : string,
  'annual_yield_rate' : number,
  'current_value' : bigint,
  'projected_yield' : bigint,
  'yield_percentage' : number,
}
export interface _SERVICE {
  'add_to_list' : ActorMethod<[Principal], Result>,
  'admin_add_broker_purchase_record' : ActorMethod<
    [bigint, bigint, string, string],
    Result
  >,
  'buy_ustbill_tokens' : ActorMethod<[string, bigint], Result_1>,
  'calculate_current_value' : ActorMethod<[string], Result_2>,
  'calculate_maturity_yield' : ActorMethod<[string], Result_2>,
  'calculate_purchase_cost' : ActorMethod<[string, bigint], Result_2>,
  'create_ustbill' : ActorMethod<[USTBillCreateRequest], Result_3>,
  'deposit_funds' : ActorMethod<[bigint], Result_2>,
  'fetch_treasury_rates' : ActorMethod<[], Result_4>,
  'get_active_ustbills' : ActorMethod<[], Array<USTBill>>,
  'get_all_verified_broker_purchases' : ActorMethod<
    [],
    Array<VerifiedBrokerPurchase>
  >,
  'get_platform_config' : ActorMethod<[], PlatformConfig>,
  'get_principal_data' : ActorMethod<[], Result_5>,
  'get_storage_stats' : ActorMethod<[], Array<[string, bigint]>>,
  'get_trading_metrics' : ActorMethod<[], TradingMetrics>,
  'get_user_holdings' : ActorMethod<[Principal], Array<TokenHolding>>,
  'get_user_profile' : ActorMethod<[Principal], Result_6>,
  'get_ustbill' : ActorMethod<[string], Result_3>,
  'get_ustbill_availability' : ActorMethod<[string], Result_2>,
  'get_ustbills_paginated' : ActorMethod<[bigint, bigint], Result_7>,
  'get_yield_projection' : ActorMethod<[string], Result_8>,
  'register_user' : ActorMethod<[UserRegistrationRequest], Result_6>,
  'set_principal_data' : ActorMethod<[string], Result>,
  'test_func' : ActorMethod<[], Result_5>,
  'transform_treasury_response' : ActorMethod<[TransformArgs], HttpResponse>,
  'update_kyc_status' : ActorMethod<[Principal, KYCStatus], Result>,
  'update_platform_config' : ActorMethod<[PlatformConfig], Result>,
  'update_ustbill_market_data' : ActorMethod<[], Result>,
  'withdraw_funds' : ActorMethod<[bigint], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
