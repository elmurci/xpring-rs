//! The xpring-rs Library
//!
//! xpring-rs is a Rust client-side library that:
//!
//! - Performs some offline calculations around XRP Ledger wallet generation/derivation
//! - Provides an easy interface to interact with the XRP Ledger.
//!
//! # Features
//!
//! xpring-rs provides the following features:
//!
//! - Wallet generation and derivation (Seed-based or HD Wallet-based)
//! - Address validation
//! - Account balance retrieval
//! - Sending XRP Payments
use anyhow::Error;

// Private modules
#[macro_use]
mod javascript;
mod config;
mod util;
mod xpring;
mod x {
    tonic::include_proto!("org.xrpl.rpc.v1");
    pub mod prelude {
        pub use super::{
            currency_amount, get_transaction_response, payment::Path as xPath,
            transaction::TransactionData, xrp_ledger_api_service_client::XrpLedgerApiServiceClient,
            Account, AccountAddress, AccountRoot, Amount, CurrencyAmount, Destination,
            GetAccountInfoRequest, GetFeeRequest, GetFeeResponse, GetTransactionRequest,
            GetTransactionResponse, LastLedgerSequence, LedgerRange, Memo, Payment, Sequence,
            Signer, SigningPublicKey, SubmitTransactionRequest, Transaction, TransactionResult,
            XrpDropsAmount,
        };
    }
}

// Public modules
pub mod address;
pub mod transaction;
pub mod wallet;
pub mod xrpclient;

pub use crate::xpring::Xpring;
