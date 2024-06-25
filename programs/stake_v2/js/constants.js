const {
    PublicKey,
    Keypair,
} = require('@solana/web3.js');
const solanaWeb3 = require("@solana/web3.js");
function getProgramAccount() {
    return Keypair.fromSecretKey(new Uint8Array([96,11,184,53,26,224,205,237,72,153,249,61,107,191,142,210,0,118,165,224,217,140,129,178,207,128,168,251,198,49,182,13,95,164,100,202,43,3,99,35,230,38,97,238,157,80,188,20,153,202,131,1,154,8,100,67,233,203,123,17,151,95,38,29]));
}

function getProgramId() {
    return new PublicKey('8ST6UCNAwcZXs2LeZ5sTKoqzct7ksiK63CYDK2mtGpbR');
}

function getPayer() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]));
}

function getProgramAuth() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]));
}

function getStakeAccount() { // stake account not the solana address
    return new PublicKey('7c6V58Epor9n8J554yqP8QW3eumeVHjnhSQbRmFVcz37');
}

function getStakePrivKey() {
    return Keypair.fromSecretKey(new Uint8Array([0,202,152,161,247,227,200,125,183,146,64,251,46,32,200,123,86,122,141,149,135,49,192,159,172,1,114,164,67,192,27,15,75,82,29,218,103,228,215,17,11,229,11,84,186,188,0,114,54,249,233,161,134,19,188,238,131,176,113,27,96,99,45,0]));
}

function getNatixTokenId() {
    return new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
}

function getTestReceiverUserTokenAccount() {
    return new PublicKey('EM97s5HgGNY5LcFVPUy5RshboRpMEHfpVUQuqEAxd8Zm');
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
    getStakePrivKey,
    getTestReceiverUserTokenAccount,
    getTestReceiverUserTokenAccountPriv
}
