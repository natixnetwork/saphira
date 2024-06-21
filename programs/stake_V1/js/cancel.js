const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getStakeAccount, getNatixTokenId, getProgramId, getTestReceiverUserTokenAccount, getPayer,
    getTestReceiverUserTokenAccountPriv
} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const programAccount = getProgramAccount().publicKey;
    const stakeAccount = getStakeAccount();
    const natixToken = getNatixTokenId();
    const programId = getProgramId();
    const userAccount = getTestReceiverUserTokenAccount()
    const userSolana = getTestReceiverUserTokenAccountPriv();


    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programAccount, isSigner: false, isWritable: true},
            {pubkey: stakeAccount, isSigner: false, isWritable: true},
            {pubkey: natixToken, isSigner: false, isWritable: false},
            {pubkey: userAccount, isSigner: false, isWritable: true},
            {pubkey: userSolana.publicKey, isSigner: true, isWritable: false},
        ],
        programId,
        data: Buffer.from(getInstruction(9, 0)),
    });

    // const i = await splToken.transferChecked(connection, payer, userAccount, natixToken, stakeAccount, payer, 1, 9);
    // console.log(i);
    const transaction = new solanaWeb3.Transaction().add(instruction);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [userSolana]
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
