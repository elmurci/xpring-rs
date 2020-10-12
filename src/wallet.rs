use crate::javascript::{JavaScript, JsCall};
use anyhow::Error;
use fehler::throws;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct XGenerateWalletOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    entropy: Option<String>,
    test: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mnemonic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    derivation_path: Option<String>,
}

impl<'a> XGenerateWalletOptions {
    fn new(test: bool) -> XGenerateWalletOptions {
        XGenerateWalletOptions {
            entropy: None,
            test,
            seed: None,
            mnemonic: None,
            derivation_path: None,
        }
    }

    fn seed(&'a mut self, seed: String) -> &'a mut XGenerateWalletOptions {
        self.seed = Some(seed);
        self
    }

    fn entropy(&'a mut self, entropy: String) -> &'a mut XGenerateWalletOptions {
        self.entropy = Some(entropy);
        self
    }

    fn mnemonic(&'a mut self, mnemonic: String) -> &'a mut XGenerateWalletOptions {
        self.mnemonic = Some(mnemonic);
        self
    }

    fn derivation_path(&'a mut self, derivation_path: String) -> &'a mut XGenerateWalletOptions {
        self.derivation_path = Some(derivation_path);
        self
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct XWallet {
    #[serde(rename(deserialize = "publicKey"))]
    pub public_key: String,
    #[serde(rename(deserialize = "privateKey"))]
    pub private_key: String,
    pub test: bool,
    pub address: Option<String>,
}

impl XWallet {
    #[allow(dead_code)]
    pub(crate) fn new(public_key: String, private_key: String, test: bool) -> XWallet {
        XWallet {
            public_key,
            private_key,
            test,
            address: None,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct XSignOptions {
    message: String,
    private_key: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct XVerifyOptions {
    message: String,
    signature: String,
    public_key: String,
}

impl XVerifyOptions {
    pub(crate) fn new(message: String, signature: String, public_key: String) -> XVerifyOptions {
        XVerifyOptions {
            message: hex::encode(message),
            signature,
            public_key,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct XWalletGenerationResult {
    pub wallet: XWallet,
    pub mnemonic: String,
    #[serde(rename(deserialize = "derivationPath"))]
    pub derivation_path: String,
}

#[throws(_)]
pub(crate) fn generate_random(
    jscontext: &mut JavaScript,
    entropy: Option<String>,
    test: bool,
) -> XWalletGenerationResult {
    let mut options = XGenerateWalletOptions::new(test);
    if let Some(e) = entropy {
        options.entropy(e);
    }
    let result = js!(jscontext
        .wallet
        .generateRandomWallet::<XWalletGenerationResult>(options))?;
    result
}

#[throws(_)]
pub(crate) fn from_mnemonic(
    jscontext: &mut JavaScript,
    mnemonic: String,
    derivation_path: Option<String>,
    test: bool,
) -> XWallet {
    let mut options = XGenerateWalletOptions::new(test);
    options.mnemonic(mnemonic);
    if let Some(d) = derivation_path {
        options.derivation_path(d);
    }
    let result = js!(jscontext
        .wallet
        .generateWalletFromMnemonic::<XWallet>(options))?;
    result
}

#[throws(_)]
pub(crate) fn from_seed(
    jscontext: &mut JavaScript,
    seed: String,
    derivation_path: Option<String>,
    test: bool,
) -> XWallet {
    let mut options = XGenerateWalletOptions::new(test);
    options.seed(seed);
    if let Some(d) = derivation_path {
        options.derivation_path(d);
    }
    let result = js!(jscontext.wallet.generateWalletFromSeed::<XWallet>(options))?;
    result
}

#[throws(_)]
pub(crate) fn sign(jscontext: &mut JavaScript, message: String, private_key: String) -> String {
    let sign_options = XSignOptions {
        message: hex::encode(message),
        private_key,
    };
    let result = js!(jscontext.wallet.sign::<String>(sign_options))?;
    result
}

#[throws(_)]
pub(crate) fn verify(
    jscontext: &mut JavaScript,
    message: String,
    signature: String,
    public_key: String,
) -> bool {
    let verify_options = XVerifyOptions::new(message, signature, public_key);
    let result = js!(jscontext.wallet.verify::<bool>(verify_options))?;
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use fehler::throws;

    #[throws(_)]
    #[test]
    fn test_random_wallet_no_entropy_testnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(&mut jscontext, None, true)?;
        assert_eq!(wallet.mnemonic.is_empty(), false);
    }

    #[throws(_)]
    #[test]
    fn test_random_wallet_with_entropy_testnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(
            &mut jscontext,
            Some("00000000000000000000000000000000".to_owned()),
            true,
        )?;
        assert_eq!(wallet.mnemonic, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about");
    }

    #[throws(_)]
    #[test]
    fn test_random_wallet_with_invalid_entropy_testnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(&mut jscontext, Some("wrong".to_owned()), true).unwrap_err();
        assert_eq!("Invalid Entropy", wallet.downcast_ref::<String>().unwrap());
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_testnet() {
        let expected = XWalletGenerationResult {
            wallet: XWallet {
                public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE".to_owned(),
                private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4".to_owned(),
                test: true,
                address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW".to_owned())
            },
            mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(),
            derivation_path: "m/44\'/144\'/0\'/0/0".to_owned()
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(
            &mut jscontext,
            Some("00000000000000000000000000000000".to_owned()),
            true,
        )
        .unwrap();
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_mainnet_with_entropy() {
        let expected = XWalletGenerationResult {
            wallet: XWallet {
                public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE".to_owned(),
                private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4".to_owned(),
                test: false,
                address: Some("XVMFQQBMhdouRqhPMuawgBMN1AVFTofPAdRsXG5RkPtUPNQ".to_owned())
            },
            mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(),
            derivation_path: "m/44\'/144\'/0\'/0/0".to_owned()
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(
            &mut jscontext,
            Some("00000000000000000000000000000000".to_owned()),
            false,
        )?;
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_testnet_with_entropy() {
        let expected = XWalletGenerationResult {
            wallet: XWallet {
                public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE".to_owned(),
                private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4".to_owned(),
                test: true,
                address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW".to_owned())
            },
            mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(),
            derivation_path: "m/44\'/144\'/0\'/0/0".to_owned()
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(
            &mut jscontext,
            Some("00000000000000000000000000000000".to_owned()),
            true,
        )
        .unwrap();
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_testnet_no_entropy() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(&mut jscontext, None, true).unwrap();
        assert!(!wallet.mnemonic.is_empty());
        assert!(!wallet.wallet.address.unwrap().is_empty());
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_mainnet_no_entropy() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(&mut jscontext, None, false).unwrap();
        assert!(!wallet.mnemonic.is_empty());
        assert!(!wallet.wallet.address.unwrap().is_empty());
    }

    #[throws(_)]
    #[test]
    fn test_generate_random_wallet_invalid_entropy() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = generate_random(&mut jscontext, Some("wrong".to_owned()), false).unwrap_err();
        assert_eq!("Invalid Entropy", wallet.downcast_ref::<String>().unwrap());
    }

    #[throws(_)]
    #[test]
    fn test_from_mnemonic_with_derivation_path_mainnet() {
        let expected = XWallet {
            public_key: "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89"
                .to_owned(),
            private_key: "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389"
                .to_owned(),
            test: true,
            address: Some("T7FxQEtaiNkq6ELhqGk3Pz2ov5aEoaGo6V642R74aaywJNT".to_owned()),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_mnemonic(&mut jscontext, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), Some("m/44'/144'/0'/0/1".to_owned()), true)?;
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_from_mnemonic_without_derivation_path_testnet() {
        let expected = XWallet {
            public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE"
                .to_owned(),
            private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4"
                .to_owned(),
            test: true,
            address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW".to_owned()),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_mnemonic(&mut jscontext, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), None, true)?;
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_mnemonic_mainnet_derivation_path_index_1() {
        let expected = XWallet {
            public_key: "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89"
                .to_owned(),
            private_key: "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389"
                .to_owned(),
            test: false,
            address: Some("X7uRz9jfzHUFEjZTZ7rMVzFuTGZTHWcmkKjvGkNqVbfMhca".to_owned()),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_mnemonic(&mut jscontext, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), Some("m/44'/144'/0'/0/1".to_owned()), false).unwrap();
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_mnemonic_mainnet_no_derivation_path() {
        let expected = XWallet {
            public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE"
                .to_owned(),
            private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4"
                .to_owned(),
            test: false,
            address: Some("XVMFQQBMhdouRqhPMuawgBMN1AVFTofPAdRsXG5RkPtUPNQ".to_owned()),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_mnemonic(&mut jscontext, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), None, false).unwrap();
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_mnemonic_testnet_no_derivation_path() {
        let expected = XWallet {
            public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE"
                .to_owned(),
            private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4"
                .to_owned(),
            test: true,
            address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW".to_owned()),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_mnemonic(&mut jscontext, "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), None, true).unwrap();
        assert_eq!(wallet, expected);
    }

    #[throws(_)]
    #[test]
    fn test_from_seed_without_derivation_path_mainnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(
            &mut jscontext,
            "snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(),
            None,
            false,
        )?;
        assert_eq!(
            wallet.address.unwrap(),
            "XVnJMYQFqA8EAijpKh5EdjEY5JqyxykMKKSbrUX8uchF6U8"
        );
    }

    #[throws(_)]
    #[test]
    fn test_from_seed_with_derivation_path_maintnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(
            &mut jscontext,
            "snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(),
            Some("m/44'/144'/0'/0/1".to_owned()),
            false,
        )?;
        assert_eq!(
            wallet.address.unwrap(),
            "XVnJMYQFqA8EAijpKh5EdjEY5JqyxykMKKSbrUX8uchF6U8"
        );
    }

    #[throws(_)]
    #[test]
    fn test_from_seed_with_invalid_seed() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(&mut jscontext, "xxx".to_owned(), None, false).unwrap_err();
        assert_eq!("Invalid Seed", wallet.downcast_ref::<String>().unwrap());
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_seed_mainnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(
            &mut jscontext,
            "snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(),
            None,
            false,
        )
        .unwrap();
        assert_eq!(
            wallet.address.unwrap(),
            "XVnJMYQFqA8EAijpKh5EdjEY5JqyxykMKKSbrUX8uchF6U8"
        );
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_seed_testnet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(
            &mut jscontext,
            "snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(),
            None,
            true,
        )
        .unwrap();
        assert_eq!(
            wallet.address.unwrap(),
            "T7zFmeZo6uLHP4Vd21TpXjrTBk487ZQPGVQsJ1mKWGCD5rq"
        );
    }

    #[throws(_)]
    #[test]
    fn test_generate_wallet_from_invalid_seed() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let wallet = from_seed(&mut jscontext, "xrp".to_owned(), None, false).unwrap_err();
        assert_eq!("Invalid Seed", wallet.downcast_ref::<String>().unwrap());
    }

    #[throws(_)]
    #[test]
    fn test_sign_message() {
        let message = "test message".to_owned();
        let expected_signature = "304402204146402099809E1F021421569F72BA34DCAFCC832741AB6310F887F60734D9F002203E813AD6A59D67D8EE06C8EA05BCC1BA8F690B631E6F243E8BE60633D27BE05D";
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let signed_message = sign(
            &mut jscontext,
            message,
            "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389".to_owned(),
        )
        .unwrap();
        assert_eq!(expected_signature, signed_message);
    }

    #[throws(_)]
    #[test]
    fn test_verify_message_valid_signature() {
        let message = "test message".to_owned();
        let signature = "304402204146402099809E1F021421569F72BA34DCAFCC832741AB6310F887F60734D9F002203E813AD6A59D67D8EE06C8EA05BCC1BA8F690B631E6F243E8BE60633D27BE05D".to_owned();
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let verified_message = verify(
            &mut jscontext,
            message,
            signature,
            "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned(),
        )?;
        assert!(verified_message);
    }

    #[throws(_)]
    #[test]
    fn test_verify_message_invalid_signature() {
        let message = "test message".to_owned();
        let signature = "DEADBEEF".to_owned();
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let verified_message = verify(
            &mut jscontext,
            message,
            signature,
            "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned(),
        )?;
        assert!(!verified_message);
    }

    #[throws(_)]
    #[test]
    fn test_verify_message_bad_signature() {
        let message = "test message".to_owned();
        let signature = "xrp".to_owned();
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let verified_message = verify(
            &mut jscontext,
            message,
            signature,
            "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned(),
        )?;
        assert!(!verified_message);
    }

    #[throws(_)]
    #[test]
    fn test_signs_and_verifies_empty_message() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let signed_message = sign(
            &mut jscontext,
            "".to_owned(),
            "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389".to_owned(),
        )
        .unwrap();
        let verified_message = verify(
            &mut jscontext,
            "".to_owned(),
            signed_message,
            "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned(),
        )?;
        assert!(verified_message);
    }

    #[throws(_)]
    #[test]
    fn test_fails_to_verify_a_bad_signature_on_an_empty_string() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let verified_message = verify(
            &mut jscontext,
            "".to_owned(),
            "DEADBEEF".to_owned(),
            "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned(),
        )?;
        assert!(!verified_message);
    }
}
