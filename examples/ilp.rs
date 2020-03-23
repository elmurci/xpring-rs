use anyhow::Error;
use fehler::throws;
use xpring::Ilp;

#[throws(_)]
fn main() {
    // Ilp instance
    let mut ilp = Ilp::new("http://hermes-grpc.ilpv4.dev", "sdk_account1", "password")?;

    // Get Balance
    let balance =
        ilp.get_balance()?;
    println!("Get Balance {:#?}", balance);

    // Send Payment
    let payment =
        ilp.send_to(
            "$money.ilpv4.dev/sdk_account2",
            13,
            10
        )?;
    println!("\nILP Payment {:#?}", payment);

    
}
