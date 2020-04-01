# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-

## Version 0.0.9 - Apr 1, 2020

- Fixed: Remove static references when unneeded
- Minor cosmetic fixes in examples and docs

## Version 0.0.8 - Mar 26, 2020

- Fixed bug: Calls to nodejs process was causing some DeprecationWarnings (thrown by a 3rd party dependency)
- Fixed bug: XWalletGenerationResult struct fields were private
- Minor cosmetic fixes in examples and docs

## Version 0.0.7 - Mar 24, 2020

- Fix docs link for crates.io

## Version 0.0.6 - Mar 24, 2020

- Docs improvements and fixes

## Version 0.0.5 - Mar 23, 2020

- Into trait in proxy calls
We can make our function easier for the caller to work with by using the Into trait.
This trait will can automatically convert a &str into a String. If we already have a String, then no conversion happens.

- Moving test boolean form methods to Xrpl constructor
Moving the test boolean parameter in functions to the Xprl constructor will make things easier as we won't have to repeat it for every method.

### ILP Support

#### Into trait in proxy calls
We can make our function easier for the caller to work with by using the Into trait.
This trait will can automatically convert a &str into a String. If we already have a String, then no conversion happens.

#### Moving test boolean form methods to Xrpl constructor
Moving the test boolean parameter in functions to the Xprl constructor will make things easier as we won't have to repeat it for every method.

## Version 0.0.4 - Mar 22, 2020

### ILP Support

This release adds support for the ILP methods to send a payment and retrieve a balance:

- Get Balance
- Send Money
