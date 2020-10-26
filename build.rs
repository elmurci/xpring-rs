use std::fs;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let _ = fs::remove_file(format!("{}/xpring.js", out_dir));

    tonic_build::compile_protos("lib/protos/rippled/xrp_ledger.proto").unwrap();
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "lib/protos/ilp/ilp_over_http_service.proto",
                "lib/protos/ilp/balance_service.proto",
            ],
            &["lib/protos/ilp/"],
        )
        .unwrap();

    fs::copy("js/dist/xpring.js", format!("{}/xpring.js", out_dir)).expect("Unable to copy file");
}
