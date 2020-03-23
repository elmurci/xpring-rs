# Contributing

Thanks for considering a contribution to [xpring-rs](https://github.com/elmurci/xpring-rs)!

We're thrilled you're interested and your help is greatly appreciated. Contributing is a great way to learn about the [XRP Ledger](https://xrpl.org) and [Interledger Protocol (ILP)](https://interledger.org/). We are happy to review your pull requests. To make the process as smooth as possible, please read this document and follow the stated guidelines.

## About This Library

<img src="https://raw.githubusercontent.com/elmurci/xpring-rs/master/architecture.png" alt="Architecture Diagram of Xpring SDK"/>

xpring-rs is a Rust library that is shipped as a consumable dependency in crates.io..

This library depends on [Xpring Common JS](https://github.com/xpring-eng/xpring-common-js), common code shared across Xpring SDK in JavaScript.

Rust comminicates via IPC with the node process to perform some of the cryptographic calculations.

## Requirements for a Successful Pull Request

Before being considered for review or merging, each pull request must:

- Pass continuous integration tests.
- Update documentation for any new features.
- Please run `clippy` and `rustfmt` before sending a pull request.
- Be [marked as drafts](https://github.blog/2019-02-14-introducing-draft-pull-requests/) until they are ready for review.
- Adhere to the [code of conduct](CODE_OF_CONDUCT.md) for this repository.

## Building The Library

Protocol buffers are compiled into Rust in the build (`build.rs`) process.

If JavaScript changes are required you will need to install the node dependencies and bundle them, this can be done via the `build_js.sh` script in the scripts folder.

```shell
$ ./build_js.sh
```

The library should build and pass all tests. 

```shell
cargo t
```