use std::{fs};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let _ = fs::remove_file(format!("{}/xpring.js", out_dir));

    tonic_build::compile_protos("lib/ripple/xrp_ledger.proto").unwrap();

    fs::copy("js/dist/xpring.js", format!("{}/xpring.js", out_dir)).expect("Unable to copy file");
}
