const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getStakeAccount, getNatixTokenId, getPayer, getProgramId, getProgramAuth} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const programSecretKey = getProgramAccount();
    const stakeAccount = getStakeAccount();
    const natixToken = getNatixTokenId();
    const programId = getProgramId();
    const auth = getProgramAuth();
    const payer = getPayer();


    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programSecretKey.publicKey, isSigner: false, isWritable: true},
            {pubkey: stakeAccount, isSigner: false, isWritable: false},
            {pubkey: natixToken, isSigner: false, isWritable: false},
            {pubkey: auth.publicKey, isSigner: true, isWritable: false},
        ],
        programId,
        data: Buffer.from(getInstruction(6, 0)),
    });


    const transaction = new solanaWeb3.Transaction().add(instruction);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, auth]
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
