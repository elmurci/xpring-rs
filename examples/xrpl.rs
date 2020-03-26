use anyhow::Error;
use fehler::throws;
use xpring::Xrpl;

#[throws(_)]
fn main() {

    // Xrpl instance (TestNet)
    let mut xrpl = Xrpl::new("http://test.xrp.xpring.io:50051", false)?;

    // Encode an X-Address
    let x_address =
        xrpl.encode_classic_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1", Some(12345), None)?;
    println!("X-Address {}", x_address);

    // Decode an X-Address
    let classic_address =
        xrpl.decode_x_address("XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUvtU3HnooQDgBnUpQT")?;
    println!("\nClassic Address {:#?}", classic_address);

    // Address Validation
    let is_wrong_address_valid = xrpl
        .validate_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?;
    println!("\nis this Address rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1 valid? {}", is_wrong_address_valid);

    // X Address Validation
    let is_x_address_valid =
        xrpl.validate_address("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;
    println!(
        "\nis this Address TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ valid? {}",
        is_x_address_valid
    );

    // Classic Address Validation
    let is_classic_address_valid = xrpl.validate_address("rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1")?;
    println!(
        "\nis this Address rU6K7V3Po4snVhBBaU29sesqs2qTQJWDw1 valid? {}",
        is_classic_address_valid
    );

    // Generate a Random Wallet
    let random_wallet = xrpl.generate_random_wallet(None)?;
    println!("\nRandom Wallet {}", random_wallet.wallet.public_key);

    // // Generate a Wallet from a seed
    let wallet_from_seed =
        xrpl.wallet_from_seed("snYP7oArxKepd3GPDcrjMsJYiJeJB", None)?;
    println!("\nWallet from seed {:#?}", wallet_from_seed);

    // Generate a Wallet from mnemonic
    let wallet_from_mnemonic = xrpl.wallet_from_mnemonic(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", 
        Some("m/44'/144'/0'/0/1")
    )?;
    println!("\nWallet from mnemonic {:#?}", wallet_from_mnemonic);

    // Sign a message
    let signed_message = xrpl.wallet_sign(
        "mymessage",
        "000974B4CFE004A2E6C4364CBF3510A36A352796728D0861F6B555ED7E54A70389",
    )?;
    println!("\nSigned Message {:?}", signed_message);

    // Verify a message
    let message_verification_result = xrpl.wallet_verify("mymessage", "3045022100DD88E31FF9AFD2A6DA48D40C4B4E8F11725E11C9D9E52388710E35ED19212EF6022068CFA9C09071322751C11DD21E89088879DC28B3B683D3F863090FB7C331EC32", "038BF420B5271ADA2D7479358FF98A29954CF18DC25155184AEAD05796DA737E89")?;
    println!(
        "\nSigned Message Verification {:?}",
        message_verification_result
    );

    // Account Balance
    let balance = xrpl.get_balance("TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ")?;

    println!(
        "\nBalance for TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ is {}",
        balance
    );

    //Send Payment
    println!("\nSending payment...");
    let sending_wallet =
        xrpl.wallet_from_seed("shKtxFAYfNUHYayYMYkp3KjQQX2UY", None)?;
    println!("sending_wallet {:?}", sending_wallet);
    let payment = xrpl.send(
        12.12,
        "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1",
        "T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU",
        sending_wallet,
    )?;
    println!("Payment sen: {:#?}", payment);

    // Transaction Status
    let transaction_status = xrpl.get_transaction_status(
        "51338E39369AECBA05B5826D77BD4C9092BAD6B578664548FE742C75D3C187CE",
    )?;

    println!("Status {:?}", transaction_status);
}
