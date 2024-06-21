const web3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');

async function findAssociatedTokenAddress(walletAddress, tokenMintAddress) {

    const connection = new web3.Connection(web3.clusterApiUrl('mainnet-beta'), 'confirmed');


    const walletPublicKey = new web3.PublicKey(walletAddress);
    const tokenMintPublicKey = new web3.PublicKey(tokenMintAddress);


    const associatedTokenAddress = await splToken.getAssociatedTokenAddress(
        tokenMintPublicKey,
        walletPublicKey,
    );

    console.log("Associated Token Address:", associatedTokenAddress.toString());

}


findAssociatedTokenAddress('FYjmJGBULnCUWHS27g7n5jSvhrvxVHnp4X7h6NUHwVJy', '2Ki2WgSXZd3V3nucYYP4WQx365EVPKZii3hVXWGNMKJe').catch(console.error);
