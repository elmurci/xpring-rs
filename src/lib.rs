// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! xRs
//!
//! xRs is a Rust client-side library that:
//! - Performs some offline calculations around XRP Ledger wallet generation/derivation
//! - Provides an easy interface to interact with the XRP Ledger.
//!
//! # Details
//!
//! xRs provides the following features:
//!
//! - Wallet generation and derivation (Seed-based or HD Wallet-based)
//! - Address validation
//! - Account balance retrieval
//! - Sending XRP payments

use anyhow::Error;

// Private modules
#[macro_use]
mod javascript;
mod transaction;
mod wallet;
mod address;
mod xpring;
mod config;
mod xrpclient;
mod util;
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
            XrpDropsAmount
        };
    }
}

pub use crate::xpring::{Xpring};