import type { Principal } from '@dfinity/principal';

// Basic types for the application
export interface AppConfig {
    environment: "local" | "ic";
    canisterId: string;
}

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

  export type USTBillStatus = { 'Active' : null } |
  { 'SoldOut' : null } |
  { 'Matured' : null } |
  { 'Cancelled' : null };


export interface ApiResponse<T> {
    Ok?: T;
    Err?: string;
}

// Add more types as needed for your specific application

export interface VerifiedBrokerPurchase {
    'ustbill_type' : string,
    'broker_txn_id' : string,
    'timestamp' : bigint,
    'price' : bigint,
    'amount' : bigint,
  }
