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

/// The Xpring struct will allow you to access all sdk the methods
pub struct Xpring {
    pub(crate) jscontext: JavaScript,
    pub(crate) xrpclient: XrpClient,
}

impl Xpring {
    #[throws(_)]
    /// Creates a Xpring struct.
    ///
    /// # Arguments
    ///
    /// * `xrpclient_url` -  `&str` Url for the XRP Ledger node.
    ///
    /// # Remarks
    ///
    /// Returns a Xpring struct wrapped in a Result (Result<Xpring, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(xrpclient_url: &'static str) -> Xpring {
        let xpringjs_path = copy_js_to_exec_path()?;
        Xpring {
            jscontext: JavaScript::new(xpringjs_path)?,
            xrpclient: XrpClient::connect(xrpclient_url)?,
        }
    }

    // Wallet

    /// Generates a random wallet. An Entropy can be passed for generation but it is optional.
    ///
    /// # Arguments
    ///
    /// * `entropy` -  `Option<String>` (Optional) Entropy.
    /// * `test` -  `bool` true for TestNet, false for MainNet.
    ///
    /// # Remarks
    ///
    /// Returns a XWalletGenerationResult with the generated wallet wrapped in a Result (Result<XWalletGenerationResult, anyhow::Error>).
    ///
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # use xpring::wallet::{XWalletGenerationResult};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let random_wallet = xpring.generate_random_wallet(None, false)?;
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
    pub fn generate_random_wallet(
        &mut self,
        entropy: Option<String>,
        test: bool,
    ) -> XWalletGenerationResult {
        wallet::generate_random(&mut self.jscontext, entropy, test)?
    }

    /// Generates a wallet from a mnemonic (and derivation path).
    ///
    /// # Arguments
    ///
    /// * `mnemonic` -  `Option<String>` Mnemonic.
    /// * `test` -  `bool` true for TestNet, false for MainNet.
    ///
    /// # Remarks
    ///
    /// Returns a XWaller with the generated wallet wrapped in a Result (Result<XWallet, anyhow::Error>).
    /// 
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # use xpring::wallet::{XWallet};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let wallet_from_mnemonic = xpring.wallet_from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), Some("m/44'/144'/0'/0/1".to_owned()), true)?;
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
    pub fn wallet_from_mnemonic(
        &mut self,
        mnemonic: String,
        derivation_path: Option<String>,
        test: bool,
    ) -> XWallet {
        wallet::from_mnemonic(&mut self.jscontext, mnemonic, derivation_path, test)?
    }

    /// Generates a wallet from a seed.
    ///
    /// # Arguments
    ///
    /// * `seed` -  `String` Seed
    /// * `derivation_path` - `Option<String>` (Optional) Derivation path.
    /// * `test` -  `bool` true for TestNet, false for MainNet.
    ///
    /// # Remarks
    ///
    /// Returns a XWallet with the generated wallet wrapped in a Result (Result<XWallet, anyhow::Error>).
    /// 
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # use xpring::wallet::{XWallet};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let wallet_from_seed =
    ///     xpring.wallet_from_seed("snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(), None, true)?;
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
    pub fn wallet_from_seed(
        &mut self,
        seed: String,
        derivation_path: Option<String>,
        test: bool,
    ) -> XWallet {
        wallet::from_seed(&mut self.jscontext, seed, derivation_path, test)?
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
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let signed_message = xpring.wallet_sign(
    ///     "mymessage".to_owned(),
    ///     "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389".to_owned(),
    /// )?;
    /// # Ok(())
    /// # }
    /// 
    /// // "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32"
    /// ```
    #[throws(_)]
    pub fn wallet_sign(&mut self, message: String, private_key: String) -> String {
        wallet::sign(&mut self.jscontext, message, private_key)?
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
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let message_verification_result = xpring.wallet_verify("mymessage".to_owned(), "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32".to_owned(), "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned())?;
    /// # Ok(())
    /// # }
    /// 
    /// // true
    /// ```
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
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let is_address_valid =
    ///     xpring.validate_address("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    /// 
    /// // true
    /// ```
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
    /// Returns a bool, true if verification is successful, false if not, wrapped in a Result (Result<bool, anyhow::Error>).
    /// 
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let is_address_valid =
    ///     xpring.validate_x_address("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    /// 
    /// // true
    /// ```
    #[throws(_)]
    pub fn validate_x_address(&mut self, x_address: &'static str) -> bool {
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
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let is_address_valid =
    ///     xpring.validate_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?;
    /// # Ok(())
    /// # }
    /// 
    /// // true
    /// ```
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
    /// Returns a String with the X-Address wrapped in a Result (Result<String, anyhow::Error>).
    /// 
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let x_address =
    ///     xpring.encode_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None)?;
    /// # Ok(())
    /// 
    /// # }
    /// 
    /// // "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT"
    /// ```
    #[throws(_)]
    pub fn encode_classic_address(
        &mut self,
        classic_address: &'static str,
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
    /// # use xpring::Xpring;
    /// # use xpring::address::XClassicAddress;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let classic_address =
    ///     xpring.decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT")?;
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
    pub fn decode_x_address(&mut self, x_address: &'static str) -> XClassicAddress {
        util::decode_x_address(&mut self.jscontext, x_address)?
    }

    // XrpClient

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
    /// # use xpring::Xpring;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let balance = xpring.get_balance("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    /// # Ok(())
    /// # }
    /// 
    /// // 1000
    /// ```
    #[throws(_)]
    pub fn get_balance(&mut self, x_address: &'static str) -> f32 {
        self.xrpclient.get_balance(&mut self.jscontext, x_address)?
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
    /// Returns a XReliableSendResponse wrapped in a Result (Result<XReliableSendResponse, anyhow::Error>).
    /// 
    /// # Example
    ///
    /// ```
    /// # use xpring::Xpring;
    /// # use xpring::xrpclient::{XReliableSendResponse};
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let sending_wallet =
    ///     xpring.wallet_from_seed("shKtxFAYfNUHYayYMYkp3KjQQX2UY".to_owned(), None, true)?;
    /// let payment = xpring.send(
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
    /// # use xpring::Xpring;
    /// # use xpring::transaction::XTransactionStatus;
    /// # fn main() -> Result<(), anyhow::Error> {
    /// # let mut xpring = Xpring::new("http://test.xrp.xpring.io:50051")?;
    /// let transaction_status = xpring.get_transaction_status(
    ///      "51338E39369AECBA05B5826D77BD4C9092BAD6B578664548FE742C75D3C187CE",
    /// )?;
    /// # Ok(())
    /// # }
    /// 
    /// // FAILED
    /// ```
    #[throws(_)]
    pub fn get_transaction_status(&mut self, transaction_hash: &str) -> XTransactionStatus {
        self.xrpclient.get_transaction_status(transaction_hash)?
    }
}
