const {
    PublicKey,
    Keypair,
} = require('@solana/web3.js');
const solanaWeb3 = require("@solana/web3.js");
function getProgramAccount() {
    return Keypair.fromSecretKey(new Uint8Array([175,90,168,178,13,150,190,36,119,213,134,79,104,142,147,243,101,33,15,117,233,248,65,72,210,77,194,78,154,148,187,82,170,210,196,6,250,19,56,175,84,51,255,125,44,227,55,34,114,76,113,49,145,124,70,112,59,136,157,172,255,95,235,59]));
}

function getProgramId() {
    return new PublicKey('4MKGvXh2z5okfY1SiZxC6CETdU269nChdKw1oesvVjYs');
}

function getPayer() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]));
}

function getProgramAuth() {
    return Keypair.fromSecretKey(new Uint8Array([95,50,127,249,105,107,51,102,133,158,83,49,196,71,93,165,211,12,116,105,146,160,155,82,114,242,82,219,97,232,142,162,119,162,218,13,22,241,107,228,47,142,106,252,165,37,77,208,14,207,27,217,246,213,231,59,191,163,16,83,82,174,210,149]));
}

function getStakeAccount() { // stake account not the solana address
    return new PublicKey('EsA9N5kWkhvB2o3TbmhxdYw5i25hiXBBS69h1tA9q2rq');
}

function getStakePrivKey() {
    return Keypair.fromSecretKey(new Uint8Array([187,0,140,92,161,252,73,79,1,145,202,84,62,39,189,156,37,31,246,228,206,14,46,200,21,151,132,147,88,36,24,64,196,226,139,144,58,132,110,39,128,26,36,155,74,233,195,56,184,26,73,16,32,237,19,103,193,16,222,247,133,45,54,101]));
}

function getNatixTokenId() {
    return new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
}

function getNatixTokenAddress() {
    return new PublicKey('7uyUK8FbXLcZWXPUXUmPSy5Q4C19U6q4kWDUFcV9TtDc');
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
    getTestReceiverUserTokenAccountPriv,
    getNatixTokenAddress
}
