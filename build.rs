use std::{fs};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    match fs::remove_file(format!("{}/xpring.js", out_dir)) {
        Ok(result) => {
           println!("Removing existing file... {:?}", result); 
        },
        Err(error) => {
            println!("Error removing existing file... {:?}", error); 
        }
    }

    tonic_build::compile_protos("lib/ripple/xrp_ledger.proto").unwrap();

    fs::copy("js/dist/xpring.js", format!("{}/xpring.js", out_dir)).expect("Unable to copy file");
}
