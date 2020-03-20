const {Wallet, Signer, Serializer, Utils} = require('xpring-common-js');
const {AccountAddress,CurrencyAmount,XRPDropsAmount} = require('../node_modules/xpring-common-js/build/generated/rpc/v1/amount_pb');
const {Payment, Transaction} = require('../node_modules/xpring-common-js/build/generated/rpc/v1/transaction_pb');
const rippleCodec = require("ripple-binary-codec");

function signTransaction(transaction, wallet) {

  wallet = new Wallet(wallet.public_key, wallet.private_key);

  const t = transaction.transaction;
  const value = t.payment.xrp_amount.drops;
  const destination = t.payment.destination;
  const fee = t.fee.drops;
  const sequence = t.sequence;
  const account = t.account;

  const paymentAmount = new XRPDropsAmount();
  paymentAmount.setDrops(value);

  const currencyAmount = new CurrencyAmount();
  currencyAmount.setXrpAmount(paymentAmount);

  const destinationAddress = new AccountAddress();
  destinationAddress.setAddress(destination);

  const payment = new Payment();
  payment.setDestination(destinationAddress);
  payment.setAmount(currencyAmount);

  const transactionFee = new XRPDropsAmount();
  transactionFee.setDrops(fee);

  const sender = new AccountAddress();
  sender.setAddress(account);

  const trx = new Transaction();
  trx.setAccount(sender);
  trx.setFee(transactionFee);
  trx.setSequence(sequence);
  trx.setPayment(payment);
  trx.setSigningPublicKey(Utils.toBytes(t.signing_public_key_hex));
  trx.setLastLedgerSequence(t.last_ledger_sequence);

  const a = Signer.signTransaction(trx, wallet);
  if (a) {
    const r = {
      transaction: Serializer.transactionToJSON(trx),
      result: Utils.toHex(a)
    };
    return r;
  } else {
    throw Error('Invalid Parameters');
  }
}

const signerExports = {
  signTransaction,
};

module.exports = signerExports;