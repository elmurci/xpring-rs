# xRs

xRs is a Rust client-side library that:
- Performs some offline calculations around XRP Ledger wallet generation/derivation
- Provides an easy interface to interact with the XRP Ledger.

## Architecture

![alt text](architecture.png "xRs Architecture")

## Features

xRs provides the following features:

- Wallet generation and derivation (Seed-based or HD Wallet-based)
- Address validation
- Account balance retrieval
- Sending XRP payments

## Requirements

- Node.js (10+)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xrs = "0.1"
```

## Usage

**Note:** xRs only works with the X-Address format. For more information about this format, see the [Utilities section](#utilities) and <http://xrpaddress.info>.

### Wallets

A wallet is a fundamental model object in xRs. A wallet provides:

- key management
- address derivation
- signing functionality

Wallets can be derived from either a seed or a mnemonic and derivation path. You can also generate a new random HD wallet.

#### Wallet Derivation

xRs can derive a wallet from a seed or it can derive a hierarchical deterministic wallet (HD Wallet) from a mnemonic and derivation path.

##### Hierarchical Deterministic Wallets

A hierarchical deterministic wallet is created using a mnemonic and a derivation path. Simply pass the mnemonic and derivation path to the wallet generation function. Note that you omit passing a derivation path and have a default path be used instead.

```rust
use xrs::{Wallet};

...

let mut wallet = Wallet::new()?;
let mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

// With mnemonic and default derivation path
let wallet = wallet.from_mnemonic(mnemonic, None, false).unwrap();

// With mnemonic and custom derivation path
let wallet = wallet.from_mnemonic(mnemonic, Some("m/44'/144'/0'/0/1".to_owned()), false).unwrap();
```

##### Seed-Based Wallets

You can construct a seed based wallet by passing a base58check encoded seed string.

```rust
use xrs::{Wallet};

...

let mut wallet = Wallet::new()?;
let wallet = wallet.from_seed("snRiAJGeKCkPVddbjB3zRwwoiYDBm1M", None, true)?;
// XWalletGenerationResult { wallet: 
//   XWallet 
//     { 
//       public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE", 
//       private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4", 
//       test: true, address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW") 
//     }, 
//   mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", 
//   derivation_path: "m/44\'/144\'/0\'/0/0" 
// }
```

#### Wallet Generation

xRs can generate a new and random HD Wallet. The result of a wallet generation call is a `XWalletGenerationResult` struct which contains the following:

- A randomly generated mnemonic
- The derivation path used, which is the default path
- A reference to the new wallet


```rust
use xrs::{wallet};

...

let mut wallet = Wallet::new()?;

// Generate a random wallet.
let wallet = wallet.random_wallet(None, true).unwrap(); //no entropy and testnet 
let wallet = wallet.random_wallet("00000000000000000000000000000000".to_owned(), false).unwrap(); //entropy and mainnet 

// XWalletGenerationResult { wallet: 
//   XWallet 
//     { 
//       public_key: "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE", 
//       private_key: "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4", 
//       test: true, 
//       address: Some("TVHLFWLKvbMv1LFzd6FA2Bf9MPpcy4mRto4VFAAxLuNpvdW") 
//     }, 
//   mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", 
//   derivation_path: "m/44\'/144\'/0\'/0/0" 
// }
```

#### Wallet Properties

A generated wallet can provide its public key, private key, and address on the XRP ledger.

```rust
use xrs::{Wallet};

...

let mut wallet = Wallet::new()?;
let wallet = wallet.from_mnemonic(
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(),
    None,
    true
).unwrap();

println!("Address: {}", wallet.address); //XVMFQQBMhdouRqhPMuawgBMN1AVFTofPAdRsXG5RkPtUPNQ
println!("Public Key: {}", wallet.publicKey); //031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE
println!("Private Key: {}", wallet.privateKey); //0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4
```

#### Signing / Verifying

A wallet can also sign and verify arbitrary messages. Generally, users should use the functions on `wallet` to perform cryptographic functions rather than using these low level APIs.

```rust
use xrs::{Wallet};

...

let mut wallet = Wallet::new()?;
let signed_message = wallet.sign(
    "test message".to_owned(),
    "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389".to_owned()
).unwrap();

println!("Signed Message: {}", signed_message); //304402204146402099809E1F021421569F72BA34DCAFCC832741AB6310F887F60734D9F002203E813AD6A59D67D8EE06C8EA05BCC1BA8F690B631E6F243E8BE60633D27BE05D

...

let verified_message = wallet.verify(
    message,
    signed_message,
    "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned()
)?;
// true
```

### XpringClient

```rust
use xrs::{XrpClient};

...

let client = XrpClient::connect("test.xrp.xpring.io:50051")?;
```

#### Retrieving a Balance

```rust
use xrs::{XrpClient};

...

let client = XrpClient::connect("http://test.xrp.xpring.io:50051")?;
let response = client.get_balance("T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1")?;
//1000.00
```

### Checking Transaction Status

An `XrpClient` can check the status of an transaction on the XRP Ledger.

xRs returns the following transaction states:
- `SUCCEEDED`: The transaction was successfully validated and applied to the XRP Ledger.
- `FAILED:` The transaction was successfully validated but not applied to the XRP Ledger. Or the operation will never be validated.
- `PENDING`: The transaction has not yet been validated, but may be validated in the future.
- `UNKNOWN`: The transaction status could not be determined.

**Note:** For more information, see [Reliable Transaction Submission](https://xrpl.org/reliable-transaction-submission.html) and [Transaction Results](https://xrpl.org/transaction-results.html).

These states are determined by the `XTransactionStatus` enum.

```rust
use xrs::{XrpClient};

...

let mut client = XrpClient::connect("http://test.xrp.xpring.io:50051")?;
let status = client.get_transaction_status("4DBA25199653A2E8BC5879DF2F830DA0149D9DE5216D2A4496A59505C107D6BB")?;
//SUCCEEDED
```

**Note:** The example transactionHash may lead to a "Transaction not found." error because the TestNet is regularly reset, or the accessed node may only maintain one month of history.  Recent transaction hashes can be found in the [XRP Ledger Explorer](https://livenet.xrpl.org)

#### Sending XRP

An `XRPClient` can send XRP to other [accounts](https://xrpl.org/accounts.html) on the XRP Ledger.

**Note:** The payment operation will block the calling thread until the operation reaches a definitive and irreversible success or failure state.

```rust
use xrs::{XrpClient, Wallet};

...

let mut client = XrpClient::connect("http://test.xrp.xpring.io:50051")?;
let w = wallet.from_seed(
    "shKtxFAYfNUHYayYMYkp3KjQQX2UY".to_owned(),
    None,
    true
).unwrap();
let response = client.send(12.12, "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1","T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU", w); 
//XReliableSendResponse {
//  transaction_status: SUCCEEDED,
//  transaction_hash: "EE70A3A5B8F5E5C6B7AA8A76E640E4AF4AFC37E87767130824F7D211CE45604E",
//  transaction_info: ""
//}
```

### Utilities

#### Address validation

```rust
use xrs::{Util};

...

let mut util = Util::new()?;

util.is_valid_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns true
util.is_valid_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns true
util.is_valid_address("1DiqLtKZZviDxccRpowkhVowsbLSNQWBE8")?; // returns false
```

You can also validate if an address is an X-Address or a classic address.

```rust
use xrs::{Util};

...

util.is_valid_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns false
util.is_valid_x_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns true
util.is_valid_x_address("1DiqLtKZZviDxccRpowkhVowsbLSNQWBE8")?; // returns false
```

```rust
use xrs::{address};

...

util.is_valid_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns true
util.is_valid_classic_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns false
address::util.```

### X-Address Encoding

```rust
use xrs::{Util};

...

// Encode an X-Address.
util.encode_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None)?; 
//XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT

// Decode an X-Address.
util.decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT")?; 
// ClassicAddress {
//     address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
//     tag: Some(12345),
//     test: false
// }
```

# Contributing

TBD

# License

xRs is available under the MIT license. See the [LICENSE](LICENSE) file for more info.