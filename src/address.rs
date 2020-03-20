use crate::javascript::{JsCall, JavaScript};
use fehler::throws;
use serde::{Deserialize, Serialize};

// Everything that gets passed to JavaScript needs to derive Serialize.
#[derive(Debug, Serialize)]
struct XAddressOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    classic_address: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_address: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<u16>,
    test: bool,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct XClassicAddress {
    pub address: String,
    pub tag: Option<u16>,
    pub test: bool,
}

impl XAddressOptions {
    pub fn new(test: bool) -> XAddressOptions {
        XAddressOptions {
            classic_address: None,
            x_address: None,
            tag: None,
            test,
        }
    }

    pub fn classic_address<'a>(
        &'a mut self,
        classic_addres: &'static str,
    ) -> &'a mut XAddressOptions {
        self.classic_address = Some(classic_addres);
        self
    }

    pub fn x_address<'a>(&'a mut self, x_address: &'static str) -> &'a mut XAddressOptions {
        self.x_address = Some(x_address);
        self
    }

    pub fn tag<'a>(&'a mut self, tag: u16) -> &'a mut XAddressOptions {
        self.tag = Some(tag);
        self
    }
}

#[throws(_)]
pub fn is_valid_address(jscontext: &mut JavaScript, address: &str) -> bool {
    let result = js!(jscontext.utils.isValidAddress::<bool>(address))?;
    result
}

#[throws(_)]
pub fn encode_classic_address(
    jscontext: &mut JavaScript,
    classic_address: &'static str,
    tag: Option<u16>,
    test: Option<bool>,
) -> String {
    let mut address = XAddressOptions::new(test.unwrap_or(false));
    address.classic_address(classic_address);
    if let Some(i) = tag {
        address.tag(i);
    }
    let result = js!(jscontext.utils.encodeXAddress::<String>(address))?;
    result
}

#[throws(_)]
pub fn is_valid_x_address(jscontext: &mut JavaScript, address: &str) -> bool {
    let result = js!(jscontext.utils.isValidXAddress::<bool>(address))?;
    result
}

#[throws(_)]
pub fn is_valid_classic_address(jscontext: &mut JavaScript,address: &str) -> bool {
    let result = js!(jscontext.utils.isValidClassicAddress::<bool>(address))?;
    result
}

#[throws(_)]
pub fn decode_x_address(jscontext: &mut JavaScript, x_address: &'static str) -> XClassicAddress {
    let mut address = XAddressOptions::new(false);
    address.x_address(x_address);
    let result = js!(jscontext.utils.decodeXAddress::<XClassicAddress>(address))?;
    result
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use fehler::throws;

//     #[throws(_)]
//     #[test]
//     fn test_valid_classic_address() {
//         assert_eq!(
//             is_valid_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
//             true
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_valid_x_address() {
//         assert_eq!(
//             is_valid_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi").unwrap(),
//             true
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_address_wrong_alphabet() {
//         assert_eq!(
//             is_valid_address("1EAG1MwmzkG6gRZcYqcRMfC17eMt8TDTit").unwrap(),
//             false
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_classic_address_wrong_checksum() {
//         assert_eq!(
//             is_valid_address("rU6K7V3Po4sBBBBBaU29sesqs2qTQJWDw1").unwrap(),
//             false
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_x_address_wrong_checksum() {
//         assert_eq!(
//             is_valid_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHI").unwrap(),
//             false
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_address_invalid_character() {
//         let jscontext = &mut XPRING_CLIENT.lock().unwrap().jscontext;
//         let result = js!(jscontext
//             .utils
//             .isValidAddress::<bool>("rU6K7V3Po4sBBBBBaU@#$%qs2qTQJWDw1"))?;
//         assert_eq!(result, false);
//     }

//     #[throws(_)]
//     #[test]
//     fn test_address_invalid_too_long() {
//         assert_eq!(
//             is_valid_address(
//                 "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1"
//             )
//             .unwrap(),
//             false
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_address_invalid_too_short() {
//         assert_eq!(is_valid_address("rU6K7V3Po4s2qTQJWDw1").unwrap(), false);
//     }

//     #[throws(_)]
//     #[test]
//     fn test_encode_x_address_mainnet_and_tag() {
//         assert_eq!(
//             encode_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None).unwrap(),
//             "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT".to_owned()
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_encode_x_address_testnet_and_tag() {
//         assert_eq!(
//             encode_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None).unwrap(),
//             "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT".to_owned()
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_encode_x_address_only() {
//         assert_eq!(
//             encode_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", None, Some(false)).unwrap(),
//             "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUFyQVMzRrMGUZpokKH".to_owned()
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_encode_x_address_invalid_address() {
//         let x_address = encode_x_address("xrp", None, Some(false)).unwrap_err();
//         assert_eq!(
//             "Invalid Parameters",
//             x_address.downcast_ref::<String>().unwrap()
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_decode_x_address_valid_mainnet_with_tag() {
//         let expected = XClassicAddress {
//             address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
//             tag: Some(12345),
//             test: false,
//         };
//         assert_eq!(
//             decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT").unwrap(),
//             expected
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_decode_x_address_valid_testnet_with_tag() {
//         let expected = XClassicAddress {
//             address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
//             tag: Some(12345),
//             test: true,
//         };
//         assert_eq!(
//             decode_x_address("TVsBZmcewpEHgajPi1jApLeYnHPJw82v9JNYf7dkGmWphmh").unwrap(),
//             expected
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_decode_x_address_valid_testnet_without_tag() {
//         let expected = XClassicAddress {
//             address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1".to_owned(),
//             tag: None,
//             test: false,
//         };
//         assert_eq!(
//             decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUFyQVMzRrMGUZpokKH").unwrap(),
//             expected
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_decode_x_address_invalid_address() {
//         let x_address = decode_x_address("xrp").unwrap_err();
//         assert_eq!(
//             "Invalid Address",
//             x_address.downcast_ref::<String>().unwrap()
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_is_valid_x_address_with_classic_address() {
//         assert_eq!(
//             is_valid_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
//             false
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_is_valid_x_address_with_invalid_address() {
//         assert_eq!(is_valid_x_address("xrp").unwrap(), false);
//     }

//     #[throws(_)]
//     #[test]
//     fn test_is_valid_x_address_with_valid_x_address() {
//         assert_eq!(
//             is_valid_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT").unwrap(),
//             true
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_is_valid_classic_address_with_valid_classic_address() {
//         assert_eq!(
//             is_valid_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1").unwrap(),
//             true
//         );
//     }

//     #[throws(_)]
//     #[test]
//     fn test_is_valid_classic_address_with_invalid_classic_address() {
//         assert_eq!(is_valid_classic_address("xrp").unwrap(), false);
//     }
// }