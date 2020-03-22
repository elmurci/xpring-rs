[![Docs](https://docs.rs/xpring/badge.svg?version=0.0.3)](https://docs.rs/xpring/badge.svg?version=0.0.3)

# xpring-rs

xpring-rs is a Rust client-side library that:

- Performs some offline calculations around XRP Ledger wallet generation/derivation
- Provides an easy interface to interact with the XRP Ledger.

## Architecture

![alt text](https://raw.githubusercontent.com/elmurci/xpring-rs/master/architecture.png "xpring-rs Architecture")

## Features

xpring-rs provides the following features:

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
xpring = "0.0.3"
```

## Usage

**Note:** xpring-rs only works with the X-Address format. For more information about this format, see the [Utilities section](#utilities) and <http://xrpaddress.info>.

### Wallets

A wallet is a fundamental model object in xpring-rs. A wallet provides:

- key management
- address derivation
- signing functionality

Wallets can be derived from either a seed or a mnemonic and derivation path. You can also generate a new random HD wallet.

#### Wallet Derivation

xpring-rs can derive a wallet from a seed or it can derive a hierarchical deterministic wallet (HD Wallet) from a mnemonic and derivation path.

##### Hierarchical Deterministic Wallets

A hierarchical deterministic wallet is created using a mnemonic and a derivation path. Simply pass the mnemonic and derivation path to the wallet generation function. Note that you omit passing a derivation path and have a default path be used instead.

```rust
use xpring::{Xprl};

...

let mut xrpl = Xprl::new("http://test.xrp.xpring.io:50051")?;

// With mnemonic and default derivation path
let wallet_from_mnemonic = xrpl.wallet_from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), None, true)?;

// With mnemonic and custom derivation path
let wallet_from_mnemonic = xrpl.wallet_from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), Some("m/44'/144'/0'/0/1".to_owned()), true)?;
```

##### Seed-Based Wallets

You can construct a seed based wallet by passing a base58check encoded seed string.

```rust
let wallet_from_seed = xrpl.wallet_from_seed("snYP7oArxKepd3GPDcrjMsJYiJeJB".to_owned(), None, true)?;
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

xpring-rs can generate a new and random HD Wallet. The result of a wallet generation call is a `XWalletGenerationResult` struct which contains the following:

- A randomly generated mnemonic
- The derivation path used, which is the default path
- A reference to the new wallet


```rust
// Generate a random wallet.
let random_wallet = xrpl.generate_random_wallet(None, false)?; //no entropy and testnet 
let random_wallet_with_entropy = xrpl.generate_random_wallet("00000000000000000000000000000000".to_owned(), false)?; //entropy and mainnet 

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
let wallet_from_mnemonic = xrpl.wallet_from_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_owned(), Some("m/44'/144'/0'/0/1".to_owned()), false)?;

println!("Address: {}", wallet.address); //XVMFQQBMhdouRqhPMuawgBMN1AVFTofPAdRsXG5RkPtUPNQ
println!("Public Key: {}", wallet.publicKey); //031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE
println!("Private Key: {}", wallet.privateKey); //0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4
```

#### Signing / Verifying

A wallet can also sign and verify arbitrary messages. Generally, users should use the functions on `wallet` to perform cryptographic functions rather than using these low level APIs.

```rust
let signed_message = xrpl.wallet_sign(
    "mymessage".to_owned(), 
    "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389".to_owned()
    ).unwrap();

println!("Signed Message: {:?}", signed_message); //304402204146402099809E1F021421569F72BA34DCAFCC832741AB6310F887F60734D9F002203E813AD6A59D67D8EE06C8EA05BCC1BA8F690B631E6F243E8BE60633D27BE05D

...

let verified_message = xrpl.wallet_verify(
    "mymessage".to_owned(), 
    "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32".to_owned(), 
    "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89".to_owned()
).unwrap();
// true
```

#### Retrieving a Balance

```rust
let balance = xrpl.get_balance("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
//1000.00
```

#### Checking Transaction Status

An `Xrpl` instance can check the status of an transaction on the XRP Ledger.

Xrpl returns the following transaction states:
- `SUCCEEDED`: The transaction was successfully validated and applied to the XRP Ledger.
- `FAILED:` The transaction was successfully validated but not applied to the XRP Ledger. Or the operation will never be validated.
- `PENDING`: The transaction has not yet been validated, but may be validated in the future.
- `UNKNOWN`: The transaction status could not be determined.

**Note:** For more information, see [Reliable Transaction Submission](https://xrpl.org/reliable-transaction-submission.html) and [Transaction Results](https://xrpl.org/transaction-results.html).

These states are determined by the `XTransactionStatus` enum.

```rust
let status = xrpl.get_transaction_status("4DBA25199653A2E8BC5879DF2F830DA0149D9DE5216D2A4496A59505C107D6BB")?;
//SUCCEEDED
```

**Note:** The example transactionHash may lead to a "Transaction not found." error because the TestNet is regularly reset, or the accessed node may only maintain one month of history.  Recent transaction hashes can be found in the [XRP Ledger Explorer](https://livenet.xrpl.org)

#### Sending XRP

An `XrplClient` can send XRP to other [accounts](https://xrpl.org/accounts.html) on the XRP Ledger.

**Note:** The payment operation will block the calling thread until the operation reaches a definitive and irreversible success or failure state.

```rust
let w = wallet.from_seed(
    "shKtxFAYfNUHYayYMYkp3KjQQX2UY".to_owned(),
    None,
    true
).unwrap();
let response = client.send(12.12, "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1","T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU", w); 
//XrplReliableSendResponse {
//  transaction_status: SUCCEEDED,
//  transaction_hash: "EE70A3A5B8F5E5C6B7AA8A76E640E4AF4AFC37E87767130824F7D211CE45604E",
//  transaction_info: ""
//}
```

### Utilities

#### Address validation

```rust
...
xrpl.validate_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns true
xrpl.validate_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns true
xrpl.validate_address("1DiqLtKZZviDxccRpowkhVowsbLSNQWBE8")?; // returns false
```

You can also validate if an address is an X-Address or a classic address.

```rust
xrpl.validate_x_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns false
xrpl.validate_x_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns true
xrpl.validate_x_address("1DiqLtKZZviDxccRpowkhVowsbLSNQWBE8")?; // returns false
```

```rust
...

xrpl.validate_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?; // returns true
xrpl.validate_classic_address("XVLhHMPHU98es4dbozjVtdWzVrDjtV18pX8yuPT7y4xaEHi")?; // returns false
```

#### X-Address Encoding

```rust
// Encode an X-Address.
xrpl.encode_clasic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None)?; 
//XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT

// Decode an X-Address.
xrpl.decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT")?; 
// ClassicAddress {
//     address: "rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1",
//     tag: Some(12345),
//     test: false
// }
```

### ILP

An `Ilp` instance can send ILP Payments and get account balances.

#### Get Balance

```rust
// Ilp instance
let mut ilp = Ilp::new("http://hermes-grpc.ilpv4.dev", "test", "password")?;

// Send Payment
let payment =
    ilp.get_balance()?;
//  IlpBalanceResponse {
//      account_id: "sdk_account1",
//      asset_code: "XRP",
//      asset_scale: 9,
//      net_balance: -10491,
//      prepaid_amount: 0,
//      clearing_balance: -10491,
//  }
```

#### Send ILP Payment

```rust
// Ilp instance
let mut ilp = Ilp::new("http://hermes-grpc.ilpv4.dev", "test", "password")?;

// Send Payment
let payment =
    ilp.send_to(
        "$money.ilpv4.dev/test2".to_owned(),
        8,
        10
    )?;
//  IlpSendResponse {
//      payment_status: SUCCEEDED,
//      original_amount: 8,
//      amount_delivered: 8,
//      amount_sent: 8,
//  }
```

# Examples

You can find some sample code in the [examples](examples) folder.

# Contributing

Pull requests are welcome! To get started with building this library and opening pull requests, please see [contributing.md](CONTRIBUTING.md).

# License

xpring-rs is available under the MIT license. See the [LICENSE](LICENSE) file for more info.