use crate::address::XClassicAddress;
use crate::ilpclient::{IlpBalanceResponse, IlpClient, IlpSendResponse};
use crate::javascript::JavaScript;
use crate::transaction::XTransactionStatus;
use crate::util;
use crate::wallet::{self, XWallet, XWalletGenerationResult};
use crate::xrplclient::{XrplClient, XrplReliableSendResponse};
use anyhow::Error;
use fehler::throws;
use std::{env, fs};

#[throws(_)]
fn copy_js_to_exec_path() -> String {
    let mut current_executable = std::env::current_exe()?;
    current_executable.pop();
    let xpringjs_content = include_str!("../js/dist/xpring.js");
    let xpringjs_path = format!("{}/xpring.js", current_executable.display());
    fs::write(xpringjs_path.clone(), xpringjs_content).expect("Unable to write file");
    xpringjs_path
}

#[throws(_)]
fn set_vars() {
    env::set_var("NODE_NO_WARNINGS", "1");
}

/// The Xrpl struct will allow you to access all the Xrpl methods
pub struct Xrpl {
    pub(crate) jscontext: JavaScript,
    pub(crate) xrplclient: XrplClient,
    pub(crate) test: bool,
}

impl Xrpl {
    /// Creates a Xpring struct.
    ///
    /// # Arguments
    ///
    /// * `xrplclient_url` -  `&str` Url for the XRP Ledger node.
    /// * `test` -  `bool` true for TestNet, false for MainNet.
    ///
    /// # Remarks
    ///
    /// Returns a Xpring struct wrapped in a Result (Result<Xpring, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let mut xpring =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// # Ok(())
    /// # }
    /// ```
    #[throws(_)]
    pub fn new<S: Into<String>>(xrplclient_url: S, test: bool) -> Xrpl {
        let xrpljs_path = copy_js_to_exec_path()?;
        set_vars()?;
        Xrpl {
            jscontext: JavaScript::new(xrpljs_path)?,
            xrplclient: XrplClient::connect(xrplclient_url.into())?,
            test,
        }
    }

    // Wallet

    /// Generates a random wallet. An Entropy can be passed for generation but it is optional.
    ///
    /// # Arguments
    ///
    /// * `entropy` -  `Option<String>` (Optional) Entropy.
    ///
    /// # Remarks
    ///
    /// Returns a XWalletGenerationResult with the generated wallet wrapped in a Result (Result<XWalletGenerationResult, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::wallet::{XWalletGenerationResult};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let random_wallet = xpring.generate_random_wallet(None)?;
    /// # Ok(())
    /// # }
    ///
    /// // {
    /// //   wallet:
    /// //     XWallet {
    /// //        public_key: "029D92AA16B71AB5EBADFD7A911C7CF8253C86BABFD7C6CB6A5587FCE20D26C5F0",
    /// //        private_key: "0006039508EB1F0BDDD511276BB8E08CDC00426992840F20083DF8E81E0AD84270",
    /// //        test: false,
    /// //        address: Some("XVesH3RwNwJ3bpAcVh54A2TxaVyyyomhErvVhfjHvrA3z2h")
    /// //     },
    /// //     mnemonic: "notable dilemma fringe install chicken icon please aim era security utility atom",
    /// //     derivation_path: "m/44\'/144\'/0\'/0/0"
    /// // }
    /// ```
    #[throws(_)]
    pub fn generate_random_wallet<S: Into<Option<String>>>(
        &mut self,
        entropy: S,
    ) -> XWalletGenerationResult {
        wallet::generate_random(&mut self.jscontext, entropy.into(), self.test)?
    }

    /// Generates a wallet from a mnemonic (and derivation path).
    ///
    /// # Arguments
    ///
    /// * `mnemonic` -  `Option<String>` Mnemonic.
    ///
    /// # Remarks
    ///
    /// Returns a XWaller with the generated wallet wrapped in a Result (Result<XWallet, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::wallet::{XWallet};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", true)?;
    /// let wallet_from_mnemonic = xrpl.wallet_from_mnemonic(
    ///     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    ///     Some("m/44'/144'/0'/0/1")
    /// )?;
    /// # Ok(())
    /// # }
    /// // XWallet {
    /// //  public_key: "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89",
    /// //  private_key: "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389",
    /// //  test: true,
    /// //  address: Some("T7FxQEtaiNkq6ELhqGk3Pz2ov5aEoaGo6V642R74aaywJNT")
    /// // }
    /// ```
    #[throws(_)]
    pub fn wallet_from_mnemonic<S: Into<String>>(
        &mut self,
        mnemonic: S,
        derivation_path: Option<&str>,
    ) -> XWallet {
        let derivation_path = if derivation_path.is_some() {
            Some(derivation_path.unwrap().to_owned())
        } else {
            None
        };
        wallet::from_mnemonic(
            &mut self.jscontext,
            mnemonic.into(),
            derivation_path,
            self.test,
        )?
    }

    /// Generates a wallet from a seed.
    ///
    /// # Arguments
    ///
    /// * `seed` -  `String` Seed
    /// * `derivation_path` - `Option<String>` (Optional) Derivation path.
    ///
    /// # Remarks
    ///
    /// Returns a XWallet with the generated wallet wrapped in a Result (Result<XWallet, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::wallet::{XWallet};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xprl =  Xrpl::new("http://test.xrp.xpring.io:50051", true)?;
    /// let wallet_from_seed =
    ///     xprl.wallet_from_seed("snYP7oArxKepd3GPDcrjMsJYiJeJB", None)?;
    /// # Ok(())
    /// # }
    ///
    /// // XWallet {
    /// //  public_key: "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89",
    /// //  private_key: "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389",
    /// //  test: true,
    /// //  address: Some("T7FxQEtaiNkq6ELhqGk3Pz2ov5aEoaGo6V642R74aaywJNT")
    /// // }
    /// ```
    #[throws(_)]
    pub fn wallet_from_seed<S: Into<String>>(
        &mut self,
        seed: S,
        derivation_path: Option<&str>,
    ) -> XWallet {
        let derivation_path = if derivation_path.is_some() {
            Some(derivation_path.unwrap().to_owned())
        } else {
            None
        };
        wallet::from_seed(&mut self.jscontext, seed.into(), derivation_path, self.test)?
    }

    /// Signs a message with a private key.
    ///
    /// # Arguments
    ///
    /// * `message` -  `String` Message to be signed.
    /// * `private_key` - `String` Private key that will sign the message.
    ///
    /// # Remarks
    ///
    /// Returns a String with the signed message wrapped in a Result (Result<String, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let signed_message = xrpl.wallet_sign(
    ///     "mymessage",
    ///     "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389",
    /// )?;
    /// # Ok(())
    /// # }
    ///
    /// // "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32"
    /// ```
    #[throws(_)]
    pub fn wallet_sign<S: Into<String>>(&mut self, message: S, private_key: S) -> String {
        wallet::sign(&mut self.jscontext, message.into(), private_key.into())?
    }

    /// Verifies with a public key a signed message.
    ///
    /// # Arguments
    ///
    /// * `message` -  `String` Message to be signed.
    /// * `signature` -  `String` Message signature.
    /// * `public_key` - `String` Signer's public key.
    ///
    /// # Remarks
    ///
    /// Returns a bool, true if verification is successful, false if not, wrapped in a Result (Result<bool, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let message_verification_result = xrpl.wallet_verify(
    ///     "mymessage",
    ///     "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32",
    ///     "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89"
    /// )?;
    /// # Ok(())
    /// # }
    ///
    /// // true
    /// ```
    #[throws(_)]
    pub fn wallet_verify<S: Into<String>>(
        &mut self,
        message: S,
        signature: S,
        public_key: S,
    ) -> bool {
        wallet::verify(
            &mut self.jscontext,
            message.into(),
            signature.into(),
            public_key.into(),
        )?
    }

    // Util

    /// Validates an address (X or Classic).
    ///
    /// # Arguments
    ///
    /// * `address` -  `&str` Address.
    ///
    /// # Remarks
    ///
    /// Returns a bool, true if verification is successful, false if not, wrapped in a Result (Result<bool, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let is_address_valid =
    ///     xrpl.validate_address("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    ///
    /// // true
    /// ```
    #[throws(_)]
    pub fn validate_address(&mut self, address: &str) -> bool {
        util::is_valid_address(&mut self.jscontext, address.into())?
    }

    /// Validates an X-Address
    ///
    /// # Arguments
    ///
    /// * `x_address` -  `&str` X-Address
    ///
    /// # Remarks
    ///
    /// Returns a bool, true if verification is successful, false if not, wrapped in a Result (Result<bool, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let is_address_valid =
    ///     xrpl.validate_x_address("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    ///
    /// // true
    /// ```
    #[throws(_)]
    pub fn validate_x_address(&mut self, x_address: &str) -> bool {
        util::is_valid_x_address(&mut self.jscontext, x_address)?
    }

    /// Validates a Classic Address.
    ///
    /// # Arguments
    ///
    /// * `address` -  `&str` Classic Address.
    ///
    /// # Remarks
    ///
    /// Returns a bool, true if verification is successful, false if not, wrapped in a Result (Result<bool, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let is_address_valid =
    ///     xrpl.validate_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?;
    /// # Ok(())
    /// # }
    ///
    /// // true
    /// ```
    #[throws(_)]
    pub fn validate_classic_address(&mut self, classic_address: &str) -> bool {
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
    /// Returns a String with the X-Address wrapped in a Result (Result<String, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let x_address =
    ///     xrpl.encode_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None)?;
    /// # Ok(())
    ///
    /// # }
    ///
    /// // "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT"
    /// ```
    #[throws(_)]
    pub fn encode_classic_address(
        &mut self,
        classic_address: &str,
        tag: Option<u16>,
        test: Option<bool>,
    ) -> String {
        util::encode_classic_address(&mut self.jscontext, classic_address, tag, test)?
    }

    /// Decodes a X-Address into a Classic Address.x
    ///
    /// # Arguments
    ///
    /// * `x_address` -  `&str` X-Address.
    ///
    /// # Remarks
    ///
    /// Returns a XClassicAddress struct wrapped in a Result (Result<XClassicAddress, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::address::XClassicAddress;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let classic_address =
    ///     xrpl.decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT")?;
    /// # Ok(())
    /// # }
    ///
    /// // {
    /// //  address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
    /// //  tag: Some(12345),
    /// //  test: false
    /// // }
    /// ```
    #[throws(_)]
    pub fn decode_x_address(&mut self, x_address: &str) -> XClassicAddress {
        util::decode_x_address(&mut self.jscontext, x_address)?
    }

    // XrplClient

    /// Returns an account balance.
    ///
    /// # Arguments
    ///
    /// * `x_address` -  `&str` Account in x format.
    ///
    /// # Remarks
    ///
    /// Returns a f32 with the balance in decimal format wrapped in a Result (Result<f32, anyhow::Error> ).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let balance = xrpl.get_balance("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    ///
    /// // 1000
    /// ```
    #[throws(_)]
    pub fn get_balance(&mut self, x_address: &str) -> f32 {
        self.xrplclient
            .get_balance(&mut self.jscontext, x_address)?
    }

    /// Sends a payment from one account to another.
    ///
    /// # Arguments
    ///
    /// * `amount` -  `f32` Payment amount in decimal format (Ex. 10.32).
    /// * `from_address` -  `&str` Origin account in x format.
    /// * `to_address` -  `&str` Destination account in x format.
    /// * `source_wallet` -  `XWallet` Wallet that will fund the payment and sign the transaction.
    ///
    /// # Remarks
    ///
    /// Returns a XrplReliableSendResponse wrapped in a Result (Result<XrplReliableSendResponse, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::xrplclient::{XrplReliableSendResponse};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let sending_wallet =
    ///     xrpl.wallet_from_seed(
    ///         "shKtxFAYfNUHYayYMYkp3KjQQX2UY",
    ///         None
    ///     )?;
    /// let payment = xrpl.send(
    ///     12.12,
    ///     "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1",
    ///     "T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU",
    ///     sending_wallet,
    /// )?;
    /// # Ok(())
    /// # }
    ///
    /// // {
    /// //  transaction_status: FAILED,
    /// //  transaction_hash: "2E01FED358DDB9B843116D858695D8EF3285BA6C7A478D054E05AD20BD50857C",
    /// //  transaction_info: "Insufficient XRP balance to send."
    /// // }
    /// ```
    #[throws(_)]
    pub fn send(
        &mut self,
        amount: f32,
        from_x_address: &str,
        to_x_address: &str,
        source_wallet: XWallet,
    ) -> XrplReliableSendResponse {
        self.xrplclient.send(
            &mut self.jscontext,
            amount,
            from_x_address,
            to_x_address,
            source_wallet,
        )?
    }

    /// Returns a certain transaction status.
    ///
    /// # Arguments
    ///
    /// * `transaction_hash` -  `&str` Transaction hash.
    ///
    /// # Remarks
    ///
    /// Returns a XTransactionStatus with the transaction status wrapped in a Result (Result<XTransactionStatus, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xrpl;
    /// # use xpring::transaction::XTransactionStatus;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xrpl =  Xrpl::new("http://test.xrp.xpring.io:50051", false)?;
    /// let transaction_status = xrpl.get_transaction_status(
    ///      "DCDFF1F2A89762CC8209F545D3C950CDCA00262A6A07CCA99C06235198746803",
    /// )?;
    /// # Ok(())
    /// # }
    ///
    /// // FAILED
    /// ```
    #[throws(_)]
    pub fn get_transaction_status(&mut self, transaction_hash: &str) -> XTransactionStatus {
        self.xrplclient.get_transaction_status(transaction_hash)?
    }
}

/// The Ilp struct will allow you to access all ILP methods
pub struct Ilp {
    pub(crate) ilpclient: IlpClient,
}

impl Ilp {
    #[throws(_)]
    pub fn new(ilpclient_url: &'static str, account_id: &'static str, token: &str) -> Ilp {
        Ilp {
            ilpclient: IlpClient::connect(ilpclient_url, account_id, token)?,
        }
    }

    /// Returns an account balance.
    ///
    /// # Remarks
    ///
    /// Returns a IlpBalanceResponse with the transaction status wrapped in a Result (Result<IlpBalanceResponse, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Ilp;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut ilp = Ilp::new("http://hermes-grpc.ilpv4.dev", "sdk_account1", "password")?;
    /// let balance = ilp.get_balance()?;
    /// # Ok(())
    /// # }
    ///
    /// //  IlpBalanceResponse {
    /// //      account_id: "sdk_account1",
    /// //      asset_code: "XRP",  
    /// //      asset_scale: 9,
    /// //      net_balance: -10491,
    /// //      prepaid_amount: 0,
    /// //      clearing_balance: -10491,
    /// //  }
    /// ```
    #[throws(_)]
    pub fn get_balance(&mut self) -> IlpBalanceResponse {
        self.ilpclient.get_balance()?
    }

    /// Sends an ILP payment to an account.
    ///
    /// # Remarks
    ///
    /// Returns a IlpSendResponse with the transaction status wrapped in a Result (Result<IlpSendResponse, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Ilp;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut ilp = Ilp::new("http://hermes-grpc.ilpv4.dev", "sdk_account1", "password")?;
    /// let payment = ilp.send_to(
    ///         "$money.ilpv4.dev/sdk_account2",
    ///         13,
    ///         10
    ///     )?;
    /// # Ok(())
    /// # }
    ///
    /// //  IlpBalanceResponse {
    /// //      account_id: "sdk_account1",
    /// //      asset_code: "XRP",  
    /// //      asset_scale: 9,
    /// //      net_balance: -10491,
    /// //      prepaid_amount: 0,
    /// //      clearing_balance: -10491,
    /// //  }
    /// ```
    #[throws(_)]
    pub fn send_to<S: Into<String>>(
        &mut self,
        destination_payment_pointer: S,
        amount: u64,
        timeout_seconds: u64,
    ) -> IlpSendResponse {
        self.ilpclient
            .send(destination_payment_pointer.into(), amount, timeout_seconds)?
    }
}
