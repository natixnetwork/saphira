const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getStakeAccount, getNatixTokenId, getProgramId, getPayer, getProgramAuth, getTestReceiverUserTokenAccount, getTestReceiverUserTokenAccountPriv, getStakeProgramId, getStakeProgramAccount} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const programAccount = getProgramAccount().publicKey;
    const programSecretKey = getProgramAccount();
    const auth = getProgramAuth();
    const stakeAccount = getStakeAccount();
    const natixToken = getNatixTokenId();
    const programId = getProgramId();
    const payer = getTestReceiverUserTokenAccountPriv();
    const userAccount = getTestReceiverUserTokenAccount();
    const stakeProgramId = getStakeProgramId();
    const stakeProgramAccount = getStakeProgramAccount().publicKey;
    
    

    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programAccount, isSigner: false, isWritable: true},            
            {pubkey: userAccount, isSigner: false, isWritable: true},            
            {pubkey: payer.publicKey, isSigner: true, isWritable: true},
            {pubkey: stakeProgramId, isSigner: false, isWritable: false},
            {pubkey: natixToken, isSigner: false, isWritable: true},
            {pubkey: stakeProgramAccount, isSigner: false, isWritable: true},
            {pubkey: stakeAccount, isSigner: false, isWritable: true},

        ],
        programId,
        data: Buffer.from(getInstruction(2, "test", "1", undefined)),
    });


    const transaction = new solanaWeb3.Transaction().add(instruction);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payer]
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
