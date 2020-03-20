use crate::address::XClassicAddress;
use crate::javascript::JavaScript;
use crate::transaction::XTransactionStatus;
use crate::util;
use crate::wallet::{self, XWallet, XWalletGenerationResult};
use crate::xrpclient::{XReliableSendResponse, XrpClient};
use fehler::throws;
use std::fs;

#[throws(_)]
fn copy_js_to_exec_path() -> String {
    let mut current_executable = std::env::current_exe()?;
    current_executable.pop();
    let xpringjs_content = include_str!("../js/dist/xpring.js");
    let xpringjs_path = format!("{}/xpring.js", current_executable.display());
    fs::write(xpringjs_path.clone(), xpringjs_content).expect("Unable to write file");
    xpringjs_path
}

pub struct Xpring {
    pub jscontext: JavaScript,
    pub xrpclient: XrpClient,
}

impl Xpring {
    #[throws(_)]
    pub fn new(xrpclient_url: &'static str) -> Xpring {
        let xpringjs_path = copy_js_to_exec_path()?;
        Xpring {
            jscontext: JavaScript::new(xpringjs_path)?,
            xrpclient: XrpClient::connect(xrpclient_url)?,
        }
    }

    // Wallet

    /// Generates a random wallet
    ///
    /// # Arguments
    ///
    /// * `entropy` -  `Option<String>` (Optional) Entropy
    /// * `test` -  `bool` true for TestNet, false for MainNet
    ///
    /// # Remarks
    ///
    /// Returns a XWalletGenerationResult with the generated wallet
    ///
    #[throws(_)]
    pub fn generate_random_wallet(
        &mut self,
        entropy: Option<String>,
        test: bool,
    ) -> XWalletGenerationResult {
        wallet::generate_random(&mut self.jscontext, entropy, test)?
    }

    /// Generates a wallet from a mnemonic (and derivation path)
    ///
    /// # Arguments
    ///
    /// * `mnemonic` -  `Option<String>` Mnemonic
    /// * `test` -  `bool` true for TestNet, false for MainNet
    ///
    /// # Remarks
    ///
    /// Returns a XWallet with the generated wallet
    #[throws(_)]
    pub fn wallet_from_mnemonic(
        &mut self,
        mnemonic: String,
        derivation_path: Option<String>,
        test: bool,
    ) -> XWallet {
        wallet::from_mnemonic(&mut self.jscontext, mnemonic, derivation_path, test)?
    }

    /// Generates a wallet from a seed
    ///
    /// # Arguments
    ///
    /// * `seed` -  `String` Seed
    /// * `derivation_path` - `Option<String>` (Optional) Derivation path
    /// * `test` -  `bool` true for TestNet, false for MainNet
    ///
    /// # Remarks
    ///
    /// Returns a XWallet with the generated wallet
    #[throws(_)]
    pub fn wallet_from_seed(
        &mut self,
        seed: String,
        derivation_path: Option<String>,
        test: bool,
    ) -> XWallet {
        wallet::from_seed(&mut self.jscontext, seed, derivation_path, test)?
    }

    /// Signs a message with a private key
    ///
    /// # Arguments
    ///
    /// * `message` -  `String` Message to be signed
    /// * `private_key` - `String` Private key that will sign the message
    ///
    /// # Remarks
    ///
    /// Returns a String with the signed message
    #[throws(_)]
    pub fn wallet_sign(&mut self, message: String, private_key: String) -> String {
        wallet::sign(&mut self.jscontext, message, private_key)?
    }

    /// Verifies with a public key a signed message
    ///
    /// # Arguments
    ///
    /// * `message` -  `String` Message to be signed
    /// * `signature` -  `String` Message signature
    /// * `public_key` - `String` Signer's public key
    ///
    /// # Remarks
    ///
    /// Returns a true if verification is successful, false if not
    #[throws(_)]
    pub fn wallet_verify(
        &mut self,
        message: String,
        signature: String,
        public_key: String,
    ) -> bool {
        wallet::verify(&mut self.jscontext, message, signature, public_key)?
    }

    // Util

    /// Validates an address (X or Classic)
    ///
    /// # Arguments
    ///
    /// * `address` -  `&str` Address
    ///
    /// # Remarks
    ///
    /// Returns a true if verification is successful, false if not
    #[throws(_)]
    pub fn validate_address(&mut self, address: &str) -> bool {
        util::is_valid_address(&mut self.jscontext, address)?
    }

    /// Validates an X-Address
    ///
    /// # Arguments
    ///
    /// * `x_address` -  `&str` X-Address
    ///
    /// # Remarks
    ///
    /// Returns a true if verification is successful, false if not
    #[throws(_)]
    pub fn validate_x_address(&mut self, x_address: &'static str) -> bool {
        util::is_valid_x_address(&mut self.jscontext, x_address)?
    }

    /// Validates a Classic Address
    ///
    /// # Arguments
    ///
    /// * `address` -  `&str` Classic Address
    ///
    /// # Remarks
    ///
    /// Returns a true if verification is successful, false if not
    #[throws(_)]
    pub fn validate_classic_address(&mut self, classic_address: &'static str) -> bool {
        util::is_valid_classic_address(&mut self.jscontext, classic_address)?
    }

    /// Encodes a Classic Address into a X-Address
    ///
    /// # Arguments
    ///
    /// * `classic_address` -  `&str` Classic Address
    ///
    /// # Remarks
    ///
    /// Returns a String with the X-Address
    #[throws(_)]
    pub fn encode_classic_address(
        &mut self,
        classic_address: &'static str,
        tag: Option<u16>,
        test: Option<bool>,
    ) -> String {
        util::encode_classic_address(&mut self.jscontext, classic_address, tag, test)?
    }

    /// Decodes a X-Address into a Classic Address
    ///
    /// # Arguments
    ///
    /// * `x_address` -  `&str` X-Address
    ///
    /// # Remarks
    ///
    /// Returns a XClassicAddress struct
    #[throws(_)]
    pub fn decode_x_address(&mut self, x_address: &'static str) -> XClassicAddress {
        util::decode_x_address(&mut self.jscontext, x_address)?
    }

    // XrpClient

    /// Returns an account balance
    ///
    /// # Arguments
    ///
    /// * `address` -  `&'static str` Account in x format
    ///
    /// # Remarks
    ///
    /// Returns a string f32 with the balance in decimal format
    ///
    #[throws(_)]
    pub fn get_balance(&mut self, x_address: &'static str) -> f32 {
        self.xrpclient.get_balance(&mut self.jscontext, x_address)?
    }

    /// Sends a payment from one account to another
    ///
    /// # Arguments
    ///
    /// * `amount` -  `f32` Payment amount in decimal format (Ex. 10.32)
    /// * `from_address` -  `&'static str` Origin account in x format
    /// * `to_address` -  `&'static str` Destination account in x format
    /// * `source_wallet` -  `XWallet` Wallet that will fund the payment and sign the transaction
    ///
    /// # Remarks
    ///
    /// Returns a XReliableSendResponse struct
    #[throws(_)]
    pub fn send(
        &mut self,
        amount: f32,
        from_x_address: &'static str,
        to_x_address: &'static str,
        source_wallet: XWallet,
    ) -> XReliableSendResponse {
        self.xrpclient.send(
            &mut self.jscontext,
            amount,
            from_x_address,
            to_x_address,
            source_wallet,
        )?
    }

    /// Returns a certain transaction status
    ///
    /// # Arguments
    ///
    /// * `transaction_hash` -  `&str` Transaction hash
    ///
    /// # Remarks
    ///
    /// Returns a transaction status (XTransactionStatus)
    ///
    #[throws(_)]
    pub fn get_transaction_status(&mut self, transaction_hash: &str) -> XTransactionStatus {
        self.xrpclient.get_transaction_status(transaction_hash)?
    }
}
