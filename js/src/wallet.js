const {Wallet} = require('xpring-common-js');

function generateRandomWallet(genOptions) {
    const w = Wallet.generateRandomWallet(genOptions.entropy, genOptions.test);
    if (w) {
        w.wallet = enhance(w.wallet);
        return w;
    } else {
        throw Error('Invalid Entropy');
    }
}

function generateWalletFromMnemonic(genOptions) {
    const w = Wallet.generateWalletFromMnemonic(genOptions.mnemonic, genOptions.derivation_path, genOptions.test);
    if (w) {
        return enhance(w);
    } else {
        throw Error('Invalid mnemonic');
    }
}

function generateWalletFromSeed(genOptions) {
    const w = Wallet.generateWalletFromSeed(genOptions.seed, genOptions.test);
    if (w) {
        return enhance(w);
    } else {
        throw Error('Invalid Seed');
    }
}

function sign(signOptions) {
    const w = new Wallet(null, signOptions.private_key, false);
    const s = w.sign(signOptions.message, signOptions.private_key);
    if (s) {
        return s;
    } else {
        throw Error('Invalid Message or Key');
    }
}

function verify(verifyOptions) {
    const w = new Wallet(verifyOptions.public_key, null, false);
    return w.verify(verifyOptions.message, verifyOptions.signature, verifyOptions.public_key);
}

function enhance(wallet) {
    wallet['address'] = wallet.getAddress();
    return wallet;
}

const walletExports = {
    generateRandomWallet,
    generateWalletFromMnemonic,
    generateWalletFromSeed,
    sign,
    verify
};

module.exports = walletExports;