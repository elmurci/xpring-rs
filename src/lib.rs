//! # reqwest
//!
//! xpring-rs is a Rust client-side library that:
//! 
//! - Performs some offline calculations around XRP Ledger wallet generation/derivation
//! - Provides an easy interface to interact with the XRP Ledger.
//! 
use anyhow::Error;

// Private modules
#[macro_use]
mod javascript;
mod address;
mod config;
mod transaction;
mod util;
mod wallet;
mod xpring;
mod xrpclient;
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

pub use crate::xpring::Xpring;
