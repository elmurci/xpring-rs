/// A representation of an account address
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAddress {
    /// base58 encoding of an account
    #[prost(string, tag = "1")]
    pub address: std::string::String,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyAmount {
    #[prost(oneof = "currency_amount::Amount", tags = "1, 2")]
    pub amount: ::std::option::Option<currency_amount::Amount>,
}
pub mod currency_amount {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Amount {
        #[prost(message, tag = "1")]
        XrpAmount(super::XrpDropsAmount),
        #[prost(message, tag = "2")]
        IssuedCurrencyAmount(super::IssuedCurrencyAmount),
    }
}
/// A representation of an amount of XRP.
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XrpDropsAmount {
    #[prost(uint64, tag = "1")]
    pub drops: u64,
}
/// A representation of an amount of issued currency.
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssuedCurrencyAmount {
    /// The currency used to value the amount.
    #[prost(message, optional, tag = "1")]
    pub currency: ::std::option::Option<Currency>,
    /// The value of the amount. 8 bytes
    #[prost(string, tag = "2")]
    pub value: std::string::String,
    /// Unique account address of the entity issuing the currency.
    #[prost(message, optional, tag = "3")]
    pub issuer: ::std::option::Option<AccountAddress>,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    /// 3 character ASCII code
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// 160 bit currency code. 20 bytes
    #[prost(bytes, tag = "2")]
    pub code: std::vec::Vec<u8>,
}
// These fields are used in many different messsage types. They can be present
// in one or more transactions, as well as metadata of one or more transactions.
// Each is defined as its own message type with a single field "value", to
// ensure the field is the correct type everywhere it's used

// *** Messages wrapping uint32 ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelAfter {
    /// time in seconds since Ripple epoch
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearFlag {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseTime {
    /// time in seconds since Ripple epoch
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Date {
    /// time in seconds since Ripple epoch
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationTag {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expiration {
    /// time in seconds since Ripple epoch
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishAfter {
    /// time in seconds since Ripple epoch
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flags {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighQualityIn {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighQualityOut {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastLedgerSequence {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LowQualityIn {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LowQualityOut {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferSequence {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerCount {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviousTransactionLedgerSequence {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityIn {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityOut {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceFeeUnits {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBase {
    /// in drops
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveIncrement {
    /// in drops
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sequence {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFlag {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleDelay {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerListId {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerQuorum {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerWeight {
    /// is actually uint16
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTag {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickSize {
    /// is actually uint8
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRate {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
// *** Messages wrapping uint64 ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseFee {
    /// in drops
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookNode {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationNode {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighNode {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexNext {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexPrevious {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LowNode {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerNode {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
// *** Messages wrapping 16 bytes ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailHash {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
// *** Messages wrapping 20 bytes ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerGetsIssuer {
    /// 20 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerPaysIssuer {
    /// 20 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
// *** Messages wrapping 32 bytes ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTransactionId {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookDirectory {
    /// 32 btes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckId {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceId {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviousTransactionId {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RootIndex {
    /// 32 bytes
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
// *** Messages wrapping variable length byte arrays ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fulfillment {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoData {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoFormat {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoType {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageKey {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentChannelSignature {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningPublicKey {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSignature {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
// *** Messages wrapping a Currency value ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerGetsCurreny {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<Currency>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerPaysCurrency {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<Currency>,
}
// *** Messages wrapping a CurrencyAmount ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    /// Note, CurrencyAmount is a oneof, that can represent an XRP drops amount
    /// or an Issued Currency amount. However, in some transaction types/ledger
    /// objects, this value can only be in drops. For instance, the Amount field
    /// of a Payment transaction can be specified in XRP drops or an Issued
    /// Currency amount, but the Amount field of a PaymentChannelClaim
    /// transaction can only be an XRP drops amount.
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliverMin {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveredAmount {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighLimit {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitAmount {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LowLimit {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMax {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerGets {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakerPays {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<CurrencyAmount>,
}
// *** Messages wrapping an AccountAddress ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorize {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegularKey {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unauthorize {
    #[prost(message, optional, tag = "1")]
    pub value: ::std::option::Option<AccountAddress>,
}
// *** Messages wrapping a string ***

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(string, tag = "1")]
    pub value: std::string::String,
}
// *** Aggregate type messages

/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerEntry {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub signer_weight: ::std::option::Option<SignerWeight>,
}
/// Next field: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerObject {
    #[prost(
        oneof = "ledger_object::Object",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub object: ::std::option::Option<ledger_object::Object>,
}
pub mod ledger_object {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Object {
        #[prost(message, tag = "1")]
        AccountRoot(super::AccountRoot),
        #[prost(message, tag = "2")]
        Amendments(super::Amendments),
        #[prost(message, tag = "3")]
        Check(super::Check),
        #[prost(message, tag = "4")]
        DepositPreauth(super::DepositPreauthObject),
        #[prost(message, tag = "5")]
        DirectoryNode(super::DirectoryNode),
        #[prost(message, tag = "6")]
        Escrow(super::Escrow),
        #[prost(message, tag = "7")]
        FeeSettings(super::FeeSettings),
        #[prost(message, tag = "8")]
        LedgerHashes(super::LedgerHashes),
        #[prost(message, tag = "9")]
        Offer(super::Offer),
        #[prost(message, tag = "10")]
        PayChannel(super::PayChannel),
        #[prost(message, tag = "11")]
        RippleState(super::RippleState),
        #[prost(message, tag = "12")]
        SignerList(super::SignerList),
    }
}
/// Next field: 15
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRoot {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::std::option::Option<Balance>,
    #[prost(message, optional, tag = "3")]
    pub sequence: ::std::option::Option<Sequence>,
    #[prost(message, optional, tag = "4")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "5")]
    pub owner_count: ::std::option::Option<OwnerCount>,
    #[prost(message, optional, tag = "6")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "7")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
    #[prost(message, optional, tag = "8")]
    pub account_transaction_id: ::std::option::Option<AccountTransactionId>,
    #[prost(message, optional, tag = "9")]
    pub domain: ::std::option::Option<Domain>,
    #[prost(message, optional, tag = "10")]
    pub email_hash: ::std::option::Option<EmailHash>,
    #[prost(message, optional, tag = "11")]
    pub message_key: ::std::option::Option<MessageKey>,
    #[prost(message, optional, tag = "12")]
    pub regular_key: ::std::option::Option<RegularKey>,
    #[prost(message, optional, tag = "13")]
    pub tick_size: ::std::option::Option<TickSize>,
    #[prost(message, optional, tag = "14")]
    pub transfer_rate: ::std::option::Option<TransferRate>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amendments {
    #[prost(message, repeated, tag = "1")]
    pub amendments: ::std::vec::Vec<amendments::Amendment>,
    #[prost(message, repeated, tag = "2")]
    pub majorities: ::std::vec::Vec<amendments::Majority>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::std::option::Option<Flags>,
}
pub mod amendments {
    /// Next field: 2
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Amendment {
        /// 32 bytes
        #[prost(bytes, tag = "1")]
        pub value: std::vec::Vec<u8>,
    }
    /// Next field: 3
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Majority {
        #[prost(message, optional, tag = "1")]
        pub amendment: ::std::option::Option<Amendment>,
        #[prost(message, optional, tag = "2")]
        pub close_time: ::std::option::Option<super::CloseTime>,
    }
}
/// Next field: 14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Check {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "4")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, optional, tag = "5")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "6")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
    #[prost(message, optional, tag = "7")]
    pub send_max: ::std::option::Option<SendMax>,
    #[prost(message, optional, tag = "8")]
    pub sequence: ::std::option::Option<Sequence>,
    #[prost(message, optional, tag = "9")]
    pub destination_node: ::std::option::Option<DestinationNode>,
    #[prost(message, optional, tag = "10")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
    #[prost(message, optional, tag = "11")]
    pub expiration: ::std::option::Option<Expiration>,
    #[prost(message, optional, tag = "12")]
    pub invoice_id: ::std::option::Option<InvoiceId>,
    #[prost(message, optional, tag = "13")]
    pub source_tag: ::std::option::Option<SourceTag>,
}
/// Next field: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositPreauthObject {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub authorize: ::std::option::Option<Authorize>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "4")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, optional, tag = "5")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "6")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
}
/// Next field: 11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryNode {
    #[prost(message, optional, tag = "1")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "2")]
    pub root_index: ::std::option::Option<RootIndex>,
    #[prost(message, repeated, tag = "3")]
    pub indexes: ::std::vec::Vec<Index>,
    #[prost(message, optional, tag = "4")]
    pub index_next: ::std::option::Option<IndexNext>,
    #[prost(message, optional, tag = "5")]
    pub index_previous: ::std::option::Option<IndexPrevious>,
    #[prost(message, optional, tag = "6")]
    pub owner: ::std::option::Option<Owner>,
    #[prost(message, optional, tag = "7")]
    pub taker_pays_currency: ::std::option::Option<TakerPaysCurrency>,
    #[prost(message, optional, tag = "8")]
    pub taker_pays_issuer: ::std::option::Option<TakerPaysIssuer>,
    #[prost(message, optional, tag = "9")]
    pub taker_gets_currency: ::std::option::Option<TakerGetsCurreny>,
    #[prost(message, optional, tag = "10")]
    pub taker_gets_issuer: ::std::option::Option<TakerGetsIssuer>,
}
/// Next field: 14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Escrow {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub condition: ::std::option::Option<Condition>,
    #[prost(message, optional, tag = "5")]
    pub cancel_after: ::std::option::Option<CancelAfter>,
    #[prost(message, optional, tag = "6")]
    pub finish_after: ::std::option::Option<FinishAfter>,
    #[prost(message, optional, tag = "7")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "8")]
    pub source_tag: ::std::option::Option<SourceTag>,
    #[prost(message, optional, tag = "9")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
    #[prost(message, optional, tag = "10")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, optional, tag = "11")]
    pub destination_node: ::std::option::Option<DestinationNode>,
    #[prost(message, optional, tag = "12")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "13")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
}
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeSettings {
    #[prost(message, optional, tag = "1")]
    pub base_fee: ::std::option::Option<BaseFee>,
    #[prost(message, optional, tag = "2")]
    pub reference_fee_units: ::std::option::Option<ReferenceFeeUnits>,
    #[prost(message, optional, tag = "3")]
    pub reserve_base: ::std::option::Option<ReserveBase>,
    #[prost(message, optional, tag = "4")]
    pub reserve_increment: ::std::option::Option<ReserveIncrement>,
    #[prost(message, optional, tag = "5")]
    pub flags: ::std::option::Option<Flags>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerHashes {
    #[prost(message, optional, tag = "1")]
    pub last_ledger_sequence: ::std::option::Option<LastLedgerSequence>,
    #[prost(message, repeated, tag = "2")]
    pub hashes: ::std::vec::Vec<Hash>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::std::option::Option<Flags>,
}
/// Next field: 12
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Offer {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub sequence: ::std::option::Option<Sequence>,
    #[prost(message, optional, tag = "3")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "4")]
    pub taker_pays: ::std::option::Option<TakerPays>,
    #[prost(message, optional, tag = "5")]
    pub taker_gets: ::std::option::Option<TakerGets>,
    #[prost(message, optional, tag = "6")]
    pub book_directory: ::std::option::Option<BookDirectory>,
    #[prost(message, optional, tag = "7")]
    pub book_node: ::std::option::Option<BookNode>,
    #[prost(message, optional, tag = "8")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, optional, tag = "9")]
    pub expiration: ::std::option::Option<Expiration>,
    #[prost(message, optional, tag = "10")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "11")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
}
/// Next field: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayChannel {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub balance: ::std::option::Option<Balance>,
    #[prost(message, optional, tag = "5")]
    pub public_key: ::std::option::Option<PublicKey>,
    #[prost(message, optional, tag = "6")]
    pub settle_delay: ::std::option::Option<SettleDelay>,
    #[prost(message, optional, tag = "7")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, optional, tag = "8")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "9")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
    #[prost(message, optional, tag = "10")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "11")]
    pub expiration: ::std::option::Option<Expiration>,
    #[prost(message, optional, tag = "12")]
    pub cancel_after: ::std::option::Option<CancelAfter>,
    #[prost(message, optional, tag = "13")]
    pub source_tag: ::std::option::Option<SourceTag>,
    #[prost(message, optional, tag = "14")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
}
/// Next field: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RippleState {
    #[prost(message, optional, tag = "1")]
    pub balance: ::std::option::Option<Balance>,
    #[prost(message, optional, tag = "2")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "3")]
    pub low_limit: ::std::option::Option<LowLimit>,
    #[prost(message, optional, tag = "4")]
    pub high_limit: ::std::option::Option<HighLimit>,
    #[prost(message, optional, tag = "5")]
    pub low_node: ::std::option::Option<LowNode>,
    #[prost(message, optional, tag = "6")]
    pub high_node: ::std::option::Option<HighNode>,
    #[prost(message, optional, tag = "7")]
    pub low_quality_in: ::std::option::Option<LowQualityIn>,
    #[prost(message, optional, tag = "8")]
    pub low_quality_out: ::std::option::Option<LowQualityOut>,
    #[prost(message, optional, tag = "9")]
    pub high_quality_in: ::std::option::Option<HighQualityIn>,
    #[prost(message, optional, tag = "10")]
    pub high_quality_out: ::std::option::Option<HighQualityOut>,
    #[prost(message, optional, tag = "11")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "12")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
}
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerList {
    #[prost(message, optional, tag = "1")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "2")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "3")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
    #[prost(message, optional, tag = "4")]
    pub owner_node: ::std::option::Option<OwnerNode>,
    #[prost(message, repeated, tag = "5")]
    pub signer_entries: ::std::vec::Vec<SignerEntry>,
    #[prost(message, optional, tag = "6")]
    pub signer_list_id: ::std::option::Option<SignerListId>,
    #[prost(message, optional, tag = "7")]
    pub signer_quorum: ::std::option::Option<SignerQuorum>,
}
/// Next field: 13
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LedgerEntryType {
    Unspecified = 0,
    AccountRoot = 1,
    Amendments = 2,
    Check = 3,
    DepositPreauth = 4,
    DirectoryNode = 5,
    Escrow = 6,
    FeeSettings = 7,
    LedgerHashes = 8,
    Offer = 9,
    PayChannel = 10,
    RippleState = 11,
    SignerList = 12,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerSpecifier {
    #[prost(oneof = "ledger_specifier::Ledger", tags = "1, 2, 3")]
    pub ledger: ::std::option::Option<ledger_specifier::Ledger>,
}
pub mod ledger_specifier {
    /// Next field: 4
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Shortcut {
        Unspecified = 0,
        Validated = 1,
        Closed = 2,
        Current = 3,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ledger {
        #[prost(enumeration = "Shortcut", tag = "1")]
        Shortcut(i32),
        #[prost(uint32, tag = "2")]
        Sequence(u32),
        /// 32 bytes
        #[prost(bytes, tag = "3")]
        Hash(std::vec::Vec<u8>),
    }
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerRange {
    #[prost(uint32, tag = "1")]
    pub ledger_index_min: u32,
    /// Note, if ledger_index_min is non-zero and ledger_index_max is 0, the
    /// software will use the max validated ledger in place of ledger_index_max
    #[prost(uint32, tag = "2")]
    pub ledger_index_max: u32,
}
/// A request to get info about an account.
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountInfoRequest {
    /// The address to get info about.
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<AccountAddress>,
    #[prost(bool, tag = "2")]
    pub strict: bool,
    #[prost(message, optional, tag = "3")]
    pub ledger: ::std::option::Option<LedgerSpecifier>,
    #[prost(bool, tag = "4")]
    pub queue: bool,
    #[prost(bool, tag = "5")]
    pub signer_lists: bool,
}
/// Response to GetAccountInfo RPC
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub account_data: ::std::option::Option<AccountRoot>,
    #[prost(message, optional, tag = "2")]
    pub signer_list: ::std::option::Option<SignerList>,
    #[prost(uint32, tag = "3")]
    pub ledger_index: u32,
    #[prost(message, optional, tag = "4")]
    pub queue_data: ::std::option::Option<QueueData>,
    #[prost(bool, tag = "5")]
    pub validated: bool,
}
/// Aggregate data about queued transactions
/// Next field: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueData {
    #[prost(uint32, tag = "1")]
    pub txn_count: u32,
    #[prost(bool, tag = "2")]
    pub auth_change_queued: bool,
    #[prost(uint32, tag = "3")]
    pub lowest_sequence: u32,
    #[prost(uint32, tag = "4")]
    pub highest_sequence: u32,
    #[prost(message, optional, tag = "5")]
    pub max_spend_drops_total: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, repeated, tag = "6")]
    pub transactions: ::std::vec::Vec<QueuedTransaction>,
}
/// Data about a single queued transaction
/// Next field: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedTransaction {
    #[prost(bool, tag = "1")]
    pub auth_change: bool,
    #[prost(message, optional, tag = "2")]
    pub fee: ::std::option::Option<XrpDropsAmount>,
    #[prost(uint64, tag = "3")]
    pub fee_level: u64,
    #[prost(message, optional, tag = "4")]
    pub max_spend_drops: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, optional, tag = "5")]
    pub sequence: ::std::option::Option<Sequence>,
    #[prost(message, optional, tag = "6")]
    pub last_ledger_sequence: ::std::option::Option<LastLedgerSequence>,
}
/// A request for the current transaction fee on the ledger.
/// Next field: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeeRequest {}
/// Response to a GetFee RPC
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeeResponse {
    #[prost(uint64, tag = "1")]
    pub current_ledger_size: u64,
    #[prost(uint64, tag = "2")]
    pub current_queue_size: u64,
    #[prost(message, optional, tag = "3")]
    pub fee: ::std::option::Option<Fee>,
    #[prost(uint64, tag = "4")]
    pub expected_ledger_size: u64,
    #[prost(uint32, tag = "5")]
    pub ledger_current_index: u32,
    #[prost(message, optional, tag = "6")]
    pub levels: ::std::option::Option<FeeLevels>,
    #[prost(uint64, tag = "7")]
    pub max_queue_size: u64,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    #[prost(message, optional, tag = "1")]
    pub base_fee: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, optional, tag = "2")]
    pub median_fee: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, optional, tag = "3")]
    pub minimum_fee: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, optional, tag = "4")]
    pub open_ledger_fee: ::std::option::Option<XrpDropsAmount>,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeLevels {
    #[prost(uint64, tag = "1")]
    pub median_level: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_level: u64,
    #[prost(uint64, tag = "3")]
    pub open_ledger_level: u64,
    #[prost(uint64, tag = "4")]
    pub reference_level: u64,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    /// index in ledger
    #[prost(uint64, tag = "1")]
    pub transaction_index: u64,
    /// result code indicating whether the transaction succeeded or failed
    #[prost(message, optional, tag = "2")]
    pub transaction_result: ::std::option::Option<TransactionResult>,
    #[prost(message, repeated, tag = "3")]
    pub affected_nodes: ::std::vec::Vec<AffectedNode>,
    #[prost(message, optional, tag = "4")]
    pub delivered_amount: ::std::option::Option<DeliveredAmount>,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionResult {
    /// category of the transaction result
    #[prost(enumeration = "transaction_result::ResultType", tag = "1")]
    pub result_type: i32,
    /// full result string, i.e. tesSUCCESS
    #[prost(string, tag = "2")]
    pub result: std::string::String,
}
pub mod transaction_result {
    /// Next field: 7
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResultType {
        Unspecified = 0,
        /// Claimed cost only
        Tec = 1,
        /// Failure
        Tef = 2,
        /// Local error
        Tel = 3,
        /// Malformed transaction
        Tem = 4,
        /// Retry
        Ter = 5,
        /// Success
        Tes = 6,
    }
}
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffectedNode {
    #[prost(enumeration = "LedgerEntryType", tag = "1")]
    pub ledger_entry_type: i32,
    /// 32 bytes
    #[prost(bytes, tag = "2")]
    pub ledger_index: std::vec::Vec<u8>,
    #[prost(oneof = "affected_node::Node", tags = "3, 4, 5")]
    pub node: ::std::option::Option<affected_node::Node>,
}
pub mod affected_node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Node {
        #[prost(message, tag = "3")]
        CreatedNode(super::CreatedNode),
        #[prost(message, tag = "4")]
        DeletedNode(super::DeletedNode),
        #[prost(message, tag = "5")]
        ModifiedNode(super::ModifiedNode),
    }
}
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatedNode {
    #[prost(message, optional, tag = "1")]
    pub new_fields: ::std::option::Option<LedgerObject>,
}
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletedNode {
    #[prost(message, optional, tag = "1")]
    pub final_fields: ::std::option::Option<LedgerObject>,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifiedNode {
    #[prost(message, optional, tag = "1")]
    pub final_fields: ::std::option::Option<LedgerObject>,
    #[prost(message, optional, tag = "2")]
    pub previous_fields: ::std::option::Option<LedgerObject>,
    #[prost(message, optional, tag = "3")]
    pub previous_transaction_id: ::std::option::Option<PreviousTransactionId>,
    #[prost(message, optional, tag = "4")]
    pub previous_transaction_ledger_sequence:
        ::std::option::Option<PreviousTransactionLedgerSequence>,
}
/// A request to submit the signed transaction to the ledger.
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionRequest {
    /// The signed transaction to submit.
    #[prost(bytes, tag = "1")]
    pub signed_transaction: std::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub fail_hard: bool,
}
/// A response when a signed transaction is submitted to the ledger.
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTransactionResponse {
    /// Code indicating the preliminary result of the transaction.
    #[prost(message, optional, tag = "1")]
    pub engine_result: ::std::option::Option<TransactionResult>,
    /// Numeric code indicating the preliminary result of the transaction,
    /// directly correlated to engine_result.
    #[prost(int64, tag = "2")]
    pub engine_result_code: i64,
    /// Human-readable explanation of the transaction's preliminary result.
    #[prost(string, tag = "3")]
    pub engine_result_message: std::string::String,
    /// 32 bytes
    #[prost(bytes, tag = "4")]
    pub hash: std::vec::Vec<u8>,
}
/// A message encompassing all transaction types
/// Next field: 30
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub fee: ::std::option::Option<XrpDropsAmount>,
    #[prost(message, optional, tag = "3")]
    pub sequence: ::std::option::Option<Sequence>,
    #[prost(message, optional, tag = "5")]
    pub signing_public_key: ::std::option::Option<SigningPublicKey>,
    #[prost(message, optional, tag = "6")]
    pub transaction_signature: ::std::option::Option<TransactionSignature>,
    #[prost(message, optional, tag = "7")]
    pub flags: ::std::option::Option<Flags>,
    #[prost(message, optional, tag = "8")]
    pub last_ledger_sequence: ::std::option::Option<LastLedgerSequence>,
    #[prost(message, optional, tag = "9")]
    pub source_tag: ::std::option::Option<SourceTag>,
    #[prost(message, repeated, tag = "10")]
    pub memos: ::std::vec::Vec<Memo>,
    #[prost(message, repeated, tag = "11")]
    pub signers: ::std::vec::Vec<Signer>,
    #[prost(message, optional, tag = "12")]
    pub account_transaction_id: ::std::option::Option<AccountTransactionId>,
    /// Data specific to the type of transaction
    #[prost(
        oneof = "transaction::TransactionData",
        tags = "4, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29"
    )]
    pub transaction_data: ::std::option::Option<transaction::TransactionData>,
}
pub mod transaction {
    /// Data specific to the type of transaction
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TransactionData {
        #[prost(message, tag = "4")]
        Payment(super::Payment),
        #[prost(message, tag = "13")]
        AccountSet(super::AccountSet),
        #[prost(message, tag = "14")]
        AccountDelete(super::AccountDelete),
        #[prost(message, tag = "15")]
        CheckCancel(super::CheckCancel),
        #[prost(message, tag = "16")]
        CheckCash(super::CheckCash),
        #[prost(message, tag = "17")]
        CheckCreate(super::CheckCreate),
        #[prost(message, tag = "18")]
        DepositPreauth(super::DepositPreauth),
        #[prost(message, tag = "19")]
        EscrowCancel(super::EscrowCancel),
        #[prost(message, tag = "20")]
        EscrowCreate(super::EscrowCreate),
        #[prost(message, tag = "21")]
        EscrowFinish(super::EscrowFinish),
        #[prost(message, tag = "22")]
        OfferCancel(super::OfferCancel),
        #[prost(message, tag = "23")]
        OfferCreate(super::OfferCreate),
        #[prost(message, tag = "24")]
        PaymentChannelClaim(super::PaymentChannelClaim),
        #[prost(message, tag = "25")]
        PaymentChannelCreate(super::PaymentChannelCreate),
        #[prost(message, tag = "26")]
        PaymentChannelFund(super::PaymentChannelFund),
        #[prost(message, tag = "27")]
        SetRegularKey(super::SetRegularKey),
        #[prost(message, tag = "28")]
        SignerListSet(super::SignerListSet),
        #[prost(message, tag = "29")]
        TrustSet(super::TrustSet),
    }
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memo {
    #[prost(message, optional, tag = "1")]
    pub memo_data: ::std::option::Option<MemoData>,
    #[prost(message, optional, tag = "2")]
    pub memo_format: ::std::option::Option<MemoFormat>,
    #[prost(message, optional, tag = "3")]
    pub memo_type: ::std::option::Option<MemoType>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signer {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<Account>,
    #[prost(message, optional, tag = "2")]
    pub transaction_signature: ::std::option::Option<TransactionSignature>,
    #[prost(message, optional, tag = "3")]
    pub signing_public_key: ::std::option::Option<SigningPublicKey>,
}
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSet {
    #[prost(message, optional, tag = "1")]
    pub clear_flag: ::std::option::Option<ClearFlag>,
    #[prost(message, optional, tag = "2")]
    pub domain: ::std::option::Option<Domain>,
    #[prost(message, optional, tag = "3")]
    pub email_hash: ::std::option::Option<EmailHash>,
    #[prost(message, optional, tag = "4")]
    pub message_key: ::std::option::Option<MessageKey>,
    #[prost(message, optional, tag = "5")]
    pub set_flag: ::std::option::Option<SetFlag>,
    #[prost(message, optional, tag = "6")]
    pub transfer_rate: ::std::option::Option<TransferRate>,
    #[prost(message, optional, tag = "7")]
    pub tick_size: ::std::option::Option<TickSize>,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDelete {
    #[prost(message, optional, tag = "1")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "2")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
}
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCancel {
    #[prost(message, optional, tag = "1")]
    pub check_id: ::std::option::Option<CheckId>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCash {
    #[prost(message, optional, tag = "1")]
    pub check_id: ::std::option::Option<CheckId>,
    #[prost(oneof = "check_cash::AmountOneof", tags = "2, 3")]
    pub amount_oneof: ::std::option::Option<check_cash::AmountOneof>,
}
pub mod check_cash {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AmountOneof {
        #[prost(message, tag = "2")]
        Amount(super::Amount),
        #[prost(message, tag = "3")]
        DeliverMin(super::DeliverMin),
    }
}
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCreate {
    #[prost(message, optional, tag = "1")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "2")]
    pub send_max: ::std::option::Option<SendMax>,
    #[prost(message, optional, tag = "3")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
    #[prost(message, optional, tag = "4")]
    pub expiration: ::std::option::Option<Expiration>,
    #[prost(message, optional, tag = "5")]
    pub invoice_id: ::std::option::Option<InvoiceId>,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositPreauth {
    #[prost(oneof = "deposit_preauth::AuthorizationOneof", tags = "1, 2")]
    pub authorization_oneof: ::std::option::Option<deposit_preauth::AuthorizationOneof>,
}
pub mod deposit_preauth {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthorizationOneof {
        #[prost(message, tag = "1")]
        Authorize(super::Authorize),
        #[prost(message, tag = "2")]
        Unauthorize(super::Unauthorize),
    }
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EscrowCancel {
    #[prost(message, optional, tag = "1")]
    pub owner: ::std::option::Option<Owner>,
    #[prost(message, optional, tag = "2")]
    pub offer_sequence: ::std::option::Option<OfferSequence>,
}
/// Next field: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EscrowCreate {
    #[prost(message, optional, tag = "1")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub cancel_after: ::std::option::Option<CancelAfter>,
    #[prost(message, optional, tag = "4")]
    pub finish_after: ::std::option::Option<FinishAfter>,
    #[prost(message, optional, tag = "5")]
    pub condition: ::std::option::Option<Condition>,
    #[prost(message, optional, tag = "6")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EscrowFinish {
    #[prost(message, optional, tag = "1")]
    pub owner: ::std::option::Option<Owner>,
    #[prost(message, optional, tag = "2")]
    pub offer_sequence: ::std::option::Option<OfferSequence>,
    #[prost(message, optional, tag = "3")]
    pub condition: ::std::option::Option<Condition>,
    #[prost(message, optional, tag = "4")]
    pub fulfillment: ::std::option::Option<Fulfillment>,
}
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferCancel {
    #[prost(message, optional, tag = "1")]
    pub offer_sequence: ::std::option::Option<OfferSequence>,
}
/// Next field: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferCreate {
    #[prost(message, optional, tag = "1")]
    pub expiration: ::std::option::Option<Expiration>,
    #[prost(message, optional, tag = "2")]
    pub offer_sequence: ::std::option::Option<OfferSequence>,
    #[prost(message, optional, tag = "3")]
    pub taker_gets: ::std::option::Option<TakerGets>,
    #[prost(message, optional, tag = "4")]
    pub taker_pays: ::std::option::Option<TakerPays>,
}
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    #[prost(message, optional, tag = "1")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
    #[prost(message, optional, tag = "4")]
    pub invoice_id: ::std::option::Option<InvoiceId>,
    #[prost(message, repeated, tag = "5")]
    pub paths: ::std::vec::Vec<payment::Path>,
    #[prost(message, optional, tag = "6")]
    pub send_max: ::std::option::Option<SendMax>,
    #[prost(message, optional, tag = "7")]
    pub deliver_min: ::std::option::Option<DeliverMin>,
}
pub mod payment {
    /// Next field: 4
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathElement {
        #[prost(message, optional, tag = "1")]
        pub account: ::std::option::Option<super::AccountAddress>,
        #[prost(message, optional, tag = "2")]
        pub currency: ::std::option::Option<super::Currency>,
        #[prost(message, optional, tag = "3")]
        pub issuer: ::std::option::Option<super::AccountAddress>,
    }
    /// Next field: 2
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Path {
        #[prost(message, repeated, tag = "1")]
        pub elements: ::std::vec::Vec<PathElement>,
    }
}
/// Next field: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentChannelClaim {
    #[prost(message, optional, tag = "1")]
    pub channel: ::std::option::Option<Channel>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::std::option::Option<Balance>,
    #[prost(message, optional, tag = "3")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "4")]
    pub payment_channel_signature: ::std::option::Option<PaymentChannelSignature>,
    #[prost(message, optional, tag = "5")]
    pub public_key: ::std::option::Option<PublicKey>,
}
/// Next field: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentChannelCreate {
    #[prost(message, optional, tag = "1")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::std::option::Option<Destination>,
    #[prost(message, optional, tag = "3")]
    pub settle_delay: ::std::option::Option<SettleDelay>,
    #[prost(message, optional, tag = "4")]
    pub public_key: ::std::option::Option<PublicKey>,
    #[prost(message, optional, tag = "5")]
    pub cancel_after: ::std::option::Option<CancelAfter>,
    #[prost(message, optional, tag = "6")]
    pub destination_tag: ::std::option::Option<DestinationTag>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentChannelFund {
    #[prost(message, optional, tag = "1")]
    pub channel: ::std::option::Option<Channel>,
    #[prost(message, optional, tag = "2")]
    pub amount: ::std::option::Option<Amount>,
    #[prost(message, optional, tag = "3")]
    pub expiration: ::std::option::Option<Expiration>,
}
/// Next field: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRegularKey {
    #[prost(message, optional, tag = "1")]
    pub regular_key: ::std::option::Option<RegularKey>,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignerListSet {
    #[prost(message, optional, tag = "1")]
    pub signer_quorum: ::std::option::Option<SignerQuorum>,
    #[prost(message, repeated, tag = "2")]
    pub signer_entries: ::std::vec::Vec<SignerEntry>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustSet {
    #[prost(message, optional, tag = "1")]
    pub limit_amount: ::std::option::Option<LimitAmount>,
    #[prost(message, optional, tag = "2")]
    pub quality_in: ::std::option::Option<QualityIn>,
    #[prost(message, optional, tag = "3")]
    pub quality_out: ::std::option::Option<QualityOut>,
}
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    /// hash of the transaction. 32 bytes
    /// ATTN: this is in binary, not hex. The JSON API accepts a hex string for
    /// a transaction hash, but here we need that hex string converted into its
    /// binary form. Each pair of hex characters should be converted into its
    /// corresponding byte. For example, the 4 character hex string "00FF"
    /// should be converted to a 2 byte array: [0, 255]
    #[prost(bytes, tag = "1")]
    pub hash: std::vec::Vec<u8>,
    /// if true, return data in binary format. defaults to false
    #[prost(bool, tag = "2")]
    pub binary: bool,
    /// search only specified range. optional
    #[prost(message, optional, tag = "3")]
    pub ledger_range: ::std::option::Option<LedgerRange>,
}
/// Next field: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    /// Sequence number of ledger that contains this transaction
    #[prost(uint32, tag = "3")]
    pub ledger_index: u32,
    /// 32 bytes
    #[prost(bytes, tag = "4")]
    pub hash: std::vec::Vec<u8>,
    /// whether the ledger has been validated
    #[prost(bool, tag = "5")]
    pub validated: bool,
    #[prost(message, optional, tag = "8")]
    pub date: ::std::option::Option<Date>,
    #[prost(
        oneof = "get_transaction_response::SerializedTransaction",
        tags = "1, 2"
    )]
    pub serialized_transaction:
        ::std::option::Option<get_transaction_response::SerializedTransaction>,
    /// metadata about the transaction
    #[prost(oneof = "get_transaction_response::SerializedMeta", tags = "6, 7")]
    pub serialized_meta: ::std::option::Option<get_transaction_response::SerializedMeta>,
}
pub mod get_transaction_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SerializedTransaction {
        #[prost(message, tag = "1")]
        Transaction(super::Transaction),
        /// Variable length
        #[prost(bytes, tag = "2")]
        TransactionBinary(std::vec::Vec<u8>),
    }
    /// metadata about the transaction
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SerializedMeta {
        #[prost(message, tag = "6")]
        Meta(super::Meta),
        /// Variable length
        #[prost(bytes, tag = "7")]
        MetaBinary(std::vec::Vec<u8>),
    }
}
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountTransactionHistoryRequest {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<AccountAddress>,
    /// Return results as binary blobs. Defaults to false.
    #[prost(bool, tag = "4")]
    pub binary: bool,
    /// If set to true, returns values indexed by older ledger first.
    /// Default to false.
    #[prost(bool, tag = "5")]
    pub forward: bool,
    /// Limit the number of results. Server may choose a lower limit.
    /// If this value is 0, the limit is ignored and the number of results
    /// returned is determined by the server
    #[prost(uint32, tag = "6")]
    pub limit: u32,
    /// Marker to resume where previous request left off
    /// Used for pagination
    #[prost(message, optional, tag = "7")]
    pub marker: ::std::option::Option<Marker>,
    /// What ledger to include results from. Specifying a not yet validated
    /// ledger results in an error. Not specifying a ledger uses the entire
    /// range of validated ledgers available to the server.
    #[prost(
        oneof = "get_account_transaction_history_request::Ledger",
        tags = "2, 3"
    )]
    pub ledger: ::std::option::Option<get_account_transaction_history_request::Ledger>,
}
pub mod get_account_transaction_history_request {
    /// What ledger to include results from. Specifying a not yet validated
    /// ledger results in an error. Not specifying a ledger uses the entire
    /// range of validated ledgers available to the server.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ledger {
        #[prost(message, tag = "2")]
        LedgerSpecifier(super::LedgerSpecifier),
        #[prost(message, tag = "3")]
        LedgerRange(super::LedgerRange),
    }
}
/// Next field: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountTransactionHistoryResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::std::option::Option<AccountAddress>,
    #[prost(uint32, tag = "2")]
    pub ledger_index_min: u32,
    #[prost(uint32, tag = "3")]
    pub ledger_index_max: u32,
    #[prost(uint32, tag = "4")]
    pub limit: u32,
    #[prost(message, optional, tag = "5")]
    pub marker: ::std::option::Option<Marker>,
    #[prost(message, repeated, tag = "6")]
    pub transactions: ::std::vec::Vec<GetTransactionResponse>,
    #[prost(bool, tag = "7")]
    pub validated: bool,
}
/// Next field: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Marker {
    #[prost(uint32, tag = "1")]
    pub ledger_index: u32,
    #[prost(uint32, tag = "2")]
    pub account_sequence: u32,
}
#[doc = r" Generated client implementations."]
pub mod xrp_ledger_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " RPCs available to interact with the XRP Ledger."]
    pub struct XrpLedgerApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl XrpLedgerApiServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> XrpLedgerApiServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Get account info for an account on the XRP Ledger."]
        pub async fn get_account_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountInfoRequest>,
        ) -> Result<tonic::Response<super::GetAccountInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetAccountInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the fee for a transaction on the XRP Ledger."]
        pub async fn get_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeeRequest>,
        ) -> Result<tonic::Response<super::GetFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/org.xrpl.rpc.v1.XRPLedgerAPIService/GetFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submit a signed transaction to the XRP Ledger."]
        pub async fn submit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/SubmitTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the status of a transaction"]
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_transaction_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountTransactionHistoryRequest>,
        ) -> Result<tonic::Response<super::GetAccountTransactionHistoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetAccountTransactionHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for XrpLedgerApiServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod xrp_ledger_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with XrpLedgerApiServiceServer."]
    #[async_trait]
    pub trait XrpLedgerApiService: Send + Sync + 'static {
        #[doc = " Get account info for an account on the XRP Ledger."]
        async fn get_account_info(
            &self,
            request: tonic::Request<super::GetAccountInfoRequest>,
        ) -> Result<tonic::Response<super::GetAccountInfoResponse>, tonic::Status>;
        #[doc = " Get the fee for a transaction on the XRP Ledger."]
        async fn get_fee(
            &self,
            request: tonic::Request<super::GetFeeRequest>,
        ) -> Result<tonic::Response<super::GetFeeResponse>, tonic::Status>;
        #[doc = " Submit a signed transaction to the XRP Ledger."]
        async fn submit_transaction(
            &self,
            request: tonic::Request<super::SubmitTransactionRequest>,
        ) -> Result<tonic::Response<super::SubmitTransactionResponse>, tonic::Status>;
        #[doc = " Get the status of a transaction"]
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        async fn get_account_transaction_history(
            &self,
            request: tonic::Request<super::GetAccountTransactionHistoryRequest>,
        ) -> Result<tonic::Response<super::GetAccountTransactionHistoryResponse>, tonic::Status>;
    }
    #[doc = " RPCs available to interact with the XRP Ledger."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct XrpLedgerApiServiceServer<T: XrpLedgerApiService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: XrpLedgerApiService> XrpLedgerApiServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T: XrpLedgerApiService> Service<http::Request<HyperBody>> for XrpLedgerApiServiceServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetAccountInfo" => {
                    struct GetAccountInfoSvc<T: XrpLedgerApiService>(pub Arc<T>);
                    impl<T: XrpLedgerApiService>
                        tonic::server::UnaryService<super::GetAccountInfoRequest>
                        for GetAccountInfoSvc<T>
                    {
                        type Response = super::GetAccountInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_account_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAccountInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetFee" => {
                    struct GetFeeSvc<T: XrpLedgerApiService>(pub Arc<T>);
                    impl<T: XrpLedgerApiService> tonic::server::UnaryService<super::GetFeeRequest> for GetFeeSvc<T> {
                        type Response = super::GetFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFeeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/SubmitTransaction" => {
                    struct SubmitTransactionSvc<T: XrpLedgerApiService>(pub Arc<T>);
                    impl<T: XrpLedgerApiService>
                        tonic::server::UnaryService<super::SubmitTransactionRequest>
                        for SubmitTransactionSvc<T>
                    {
                        type Response = super::SubmitTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.submit_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SubmitTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetTransaction" => {
                    struct GetTransactionSvc<T: XrpLedgerApiService>(pub Arc<T>);
                    impl<T: XrpLedgerApiService>
                        tonic::server::UnaryService<super::GetTransactionRequest>
                        for GetTransactionSvc<T>
                    {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/org.xrpl.rpc.v1.XRPLedgerAPIService/GetAccountTransactionHistory" => {
                    struct GetAccountTransactionHistorySvc<T: XrpLedgerApiService>(pub Arc<T>);
                    impl<T: XrpLedgerApiService>
                        tonic::server::UnaryService<super::GetAccountTransactionHistoryRequest>
                        for GetAccountTransactionHistorySvc<T>
                    {
                        type Response = super::GetAccountTransactionHistoryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountTransactionHistoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.get_account_transaction_history(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAccountTransactionHistorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: XrpLedgerApiService> Clone for XrpLedgerApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: XrpLedgerApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: XrpLedgerApiService> tonic::transport::NamedService for XrpLedgerApiServiceServer<T> {
        const NAME: &'static str = "org.xrpl.rpc.v1.XRPLedgerAPIService";
    }
}
