const {
    PublicKey,
    Keypair,
} = require('@solana/web3.js');
const solanaWeb3 = require("@solana/web3.js");


const connectionUrl = "https://api.devnet.solana.com";

function getProgramAccount() {
    return Keypair.fromSecretKey(new Uint8Array([183,192,219,225,14,252,115,43,180,102,49,30,182,142,52,50,47,248,250,40,240,15,240,255,231,195,145,133,177,236,42,109,86,47,180,188,100,52,39,60,182,245,203,75,53,226,7,7,54,112,231,157,0,23,206,123,92,209,151,207,98,41,222,236]));
}

function getStakeProgramAccountPubKey() {
    return new PublicKey('3nnPBvMtSCbcr2LYWHmpoqnHVGqR39Xw9JkvbSSeFNp7');
}

function getProgramId() {
    return new PublicKey('HSRa6YQVkiW24qqmaTWUQ8EqnYeVCrB6MRVAGJVxjEnk');
}

function getStakeProgramId() {
    return new PublicKey('FqTzWJoJSAqG6fPwo4cucDtaK8MowBUH82TmxSXhLxbJ');
}

function getPayer() {
    return Keypair.fromSecretKey(new Uint8Array([62,27,180,32,130,181,237,144,125,107,211,75,219,77,12,164,252,65,58,55,162,165,117,224,23,107,196,1,30,238,175,110,216,36,249,165,81,94,11,255,139,2,228,171,186,176,230,36,83,113,102,4,59,66,129,23,238,129,253,63,23,138,237,18]));
}

function getProgramAuth() {
    return Keypair.fromSecretKey(new Uint8Array([62,27,180,32,130,181,237,144,125,107,211,75,219,77,12,164,252,65,58,55,162,165,117,224,23,107,196,1,30,238,175,110,216,36,249,165,81,94,11,255,139,2,228,171,186,176,230,36,83,113,102,4,59,66,129,23,238,129,253,63,23,138,237,18]));
}

function getStakeAccount() { // stake account not the solana address
    return new PublicKey('GeqHEVxXzALY6eGKax9EvauQj9QriaE53CtdWugvz91k');
}

function getNatixTokenId() {
    return new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
}

function getTestReceiverUserTokenAccount() {
    return new PublicKey('8JEyDoMUoa5534LqB2VLTi3A2NrB6HSCjgqc3LE1jB2k');
}

function getTestReceiverUserTokenAccountPriv() {
    return Keypair.fromSecretKey(new Uint8Array([62,27,180,32,130,181,237,144,125,107,211,75,219,77,12,164,252,65,58,55,162,165,117,224,23,107,196,1,30,238,175,110,216,36,249,165,81,94,11,255,139,2,228,171,186,176,230,36,83,113,102,4,59,66,129,23,238,129,253,63,23,138,237,18]))
}

module.exports = {
    getProgramAuth,
    getProgramAccount,
    getProgramId,
    getPayer,
    getStakeAccount,
    getNatixTokenId,    
    getTestReceiverUserTokenAccount,
    getTestReceiverUserTokenAccountPriv,
    getStakeProgramId,
    getStakeProgramAccountPubKey,
    connectionUrl,
}
