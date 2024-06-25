const {
    PublicKey,
    Keypair,
} = require('@solana/web3.js');
const solanaWeb3 = require("@solana/web3.js");
function getProgramAccount() {
    return Keypair.fromSecretKey(new Uint8Array([177,150,241,96,186,62,225,25,161,138,149,106,44,245,10,207,241,245,146,108,145,164,29,238,135,91,9,97,239,78,142,33,95,225,72,48,71,115,22,91,210,166,84,89,24,184,192,159,225,72,125,58,143,176,120,97,124,122,26,19,102,87,192,186]));
}

function getStakeProgramAccount() {
    return Keypair.fromSecretKey(new Uint8Array([34,24,251,80,58,241,187,130,213,51,89,204,70,218,169,160,143,21,58,68,8,39,220,155,9,230,94,210,53,40,250,152,218,71,14,178,137,23,193,230,243,76,79,3,106,167,199,179,161,8,214,211,158,198,119,213,211,44,255,23,77,3,245,58]));
}

function getProgramId() {
    return new PublicKey('7JhpuF2fspdB7UrewW7T5tAg48ZorDatU7dzobuw21e7');
}

function getStakeProgramId() {
    return new PublicKey('BvALA9ELGX3xJek1kk1kXKHhGr3CB7e7NWNFVLaqzc8x');
}

function getPayer() {
    return Keypair.fromSecretKey(new Uint8Array([129,36,13,18,86,252,173,177,44,177,133,119,167,113,82,162,205,32,125,203,227,228,144,128,113,109,215,198,233,85,46,188,185,191,179,119,160,186,97,62,25,35,231,125,25,60,232,164,179,202,219,38,193,3,8,143,170,28,240,113,32,149,101,200]));
}

function getProgramAuth() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]));
}

function getStakeAccount() { // stake account not the solana address
    return new PublicKey('2JbN4E2sE3nBpp33UXFrRNUtxH1qkGGQ9ix7M5ZDzr9P');
}

function getStakePrivKey() {
    return Keypair.fromSecretKey(new Uint8Array([4,216,213,201,108,211,252,86,150,27,175,111,52,97,83,213,52,138,207,161,29,6,214,42,86,214,16,185,31,208,26,233,48,72,121,181,10,28,0,212,79,226,42,109,247,97,1,229,80,2,83,181,243,114,199,7,182,7,108,162,49,252,128,254]));
}

function getNatixTokenId() {
    return new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
}

function getTestReceiverUserTokenAccount() {
    return new PublicKey('EM97s5HgGNY5LcFVPUy5RshboRpMEHfpVUQuqEAxd8Zm');
}

function getTestReceiverUserTokenAccountPriv() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]))
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
    getTestReceiverUserTokenAccountPriv,
    getStakeProgramId,
    getStakeProgramAccount

}
