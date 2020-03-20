const {Utils} = require('xpring-common-js');

function isValidAddress(address) {
  return  Utils.isValidAddress(address);
}

function encodeXAddress(addresOptions) {
  const a = Utils.encodeXAddress(addresOptions.classic_address, addresOptions.tag, addresOptions.test);
    if (a) {
        return a;
    } else {
        throw Error('Invalid Parameters');
    }
}

function decodeXAddress(addresOptions) {
  const a = Utils.decodeXAddress(addresOptions.x_address);
    if (a) {
        return a;
    } else {
        throw Error('Invalid Address');
    }
}

function transactionBlobToTransactionHash(transactionBlobHex) {
  const a = Utils.transactionBlobToTransactionHash(transactionBlobHex);
    if (a) {
        return a;
    } else {
        throw Error('Invalid Input');
    }
}

function isValidXAddress(address) {
  return  Utils.isValidXAddress(address);
}

function isValidClassicAddress(address) {
  return  Utils.isValidClassicAddress(address);
}

const utilExports = {
  isValidAddress,
  encodeXAddress,
  decodeXAddress,
  isValidXAddress,
  isValidClassicAddress,
  transactionBlobToTransactionHash,
};

module.exports = utilExports;