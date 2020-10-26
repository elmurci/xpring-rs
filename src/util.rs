use crate::address;
use crate::address::XClassicAddress;
use crate::javascript::JavaScript;
use anyhow::Error;
use fehler::throws;
use std::str;

// Address

#[throws(_)]
pub fn is_valid_address(jscontext: &mut JavaScript, address: &str) -> bool {
    address::is_valid_address(jscontext, address)?
}

#[throws(_)]
pub fn encode_classic_address(
    jscontext: &mut JavaScript,
    classic_address: &str,
    tag: Option<u16>,
    test: Option<bool>,
) -> String {
    address::encode_classic_address(jscontext, classic_address, tag, test)?
}

#[throws(_)]
pub fn decode_x_address(jscontext: &mut JavaScript, x_address: &str) -> XClassicAddress {
    address::decode_x_address(jscontext, x_address)?
}

#[throws(_)]
pub fn is_valid_x_address(jscontext: &mut JavaScript, x_address: &str) -> bool {
    address::is_valid_x_address(jscontext, x_address)?
}

#[throws(_)]
pub fn is_valid_classic_address(jscontext: &mut JavaScript, address: &str) -> bool {
    address::is_valid_classic_address(jscontext, address)?
}

#[cfg(test)]
mod tests {

    use super::*;
    use fehler::throws;

    #[throws(_)]
    #[test]
    fn test_valid_classic_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(&mut jscontext, "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
            true
        );
    }

    #[throws(_)]
    #[test]
    fn test_valid_x_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(
                &mut jscontext,
                "XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi"
            )
            .unwrap(),
            true
        );
    }

    #[throws(_)]
    #[test]
    fn test_address_wrong_alphabet() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(&mut jscontext, "1EAG1MwmzkG6gRZcYqcRMfC17eMt8TDTit").unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_classic_address_wrong_checksum() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(&mut jscontext, "rU6K7V3Po4sBBBBBaU29sesqs2qTQJWDw1").unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_x_address_wrong_checksum() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(
                &mut jscontext,
                "XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHI"
            )
            .unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_address_invalid_character() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(&mut jscontext, "rU6K7V3Po4sBBBBBaU@#$%qs2qTQJWDw1").unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_address_invalid_too_long() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(
                &mut jscontext,
                "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1"
            )
            .unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_address_invalid_too_short() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_address(&mut jscontext, "rU6K7V3Po4s2qTQJWDw1").unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_encode_x_address_mainnet_and_tag() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            encode_classic_address(
                &mut jscontext,
                "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
                Some(12345),
                None
            )
            .unwrap(),
            "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT".to_owned()
        );
    }

    #[throws(_)]
    #[test]
    fn test_encode_x_address_testnet_and_tag() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            encode_classic_address(
                &mut jscontext,
                "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
                Some(12345),
                None
            )
            .unwrap(),
            "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT".to_owned()
        );
    }

    #[throws(_)]
    #[test]
    fn test_encode_x_address_only() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            encode_classic_address(
                &mut jscontext,
                "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
                None,
                Some(false)
            )
            .unwrap(),
            "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUFyQVMzRrMGUZpokKH".to_owned()
        );
    }

    #[throws(_)]
    #[test]
    fn test_encode_x_address_invalid_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let x_address =
            encode_classic_address(&mut jscontext, "xrp", None, Some(false)).unwrap_err();
        assert_eq!(
            "Invalid Parameters",
            x_address.downcast_ref::<String>().unwrap()
        );
    }

    #[throws(_)]
    #[test]
    fn test_decode_x_address_valid_mainnet_with_tag() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let expected = XClassicAddress {
            address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
            tag: Some(12345),
            test: false,
        };
        assert_eq!(
            decode_x_address(
                &mut jscontext,
                "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT"
            )
            .unwrap(),
            expected
        );
    }

    #[throws(_)]
    #[test]
    fn test_decode_x_address_valid_testnet_with_tag() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let expected = XClassicAddress {
            address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
            tag: Some(12345),
            test: true,
        };
        assert_eq!(
            decode_x_address(
                &mut jscontext,
                "TVsBZmcewpEHgajPi1jApLeYnHPJw82v9JNYf7dkGmWphmh"
            )
            .unwrap(),
            expected
        );
    }

    #[throws(_)]
    #[test]
    fn test_decode_x_address_valid_testnet_without_tag() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let expected = XClassicAddress {
            address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
            tag: None,
            test: false,
        };
        assert_eq!(
            decode_x_address(
                &mut jscontext,
                "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUFyQVMzRrMGUZpokKH"
            )
            .unwrap(),
            expected
        );
    }

    #[throws(_)]
    #[test]
    fn test_decode_x_address_invalid_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let x_address = decode_x_address(&mut jscontext, "xrp").unwrap_err();
        assert_eq!(
            "Invalid Address",
            x_address.downcast_ref::<String>().unwrap()
        );
    }

    #[throws(_)]
    #[test]
    fn test_is_valid_x_address_with_classic_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_x_address(&mut jscontext, "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
            false
        );
    }

    #[throws(_)]
    #[test]
    fn test_is_valid_x_address_with_invalid_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(is_valid_x_address(&mut jscontext, "xrp").unwrap(), false);
    }

    #[throws(_)]
    #[test]
    fn test_is_valid_x_address_with_valid_x_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_x_address(
                &mut jscontext,
                "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT"
            )
            .unwrap(),
            true
        );
    }

    #[throws(_)]
    #[test]
    fn test_is_valid_classic_address_with_valid_classic_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_classic_address(&mut jscontext, "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
            true
        );
    }

    #[throws(_)]
    #[test]
    fn test_is_valid_classic_address_with_invalid_classic_address() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        assert_eq!(
            is_valid_classic_address(&mut jscontext, "xrp").unwrap(),
            false
        );
    }
}
