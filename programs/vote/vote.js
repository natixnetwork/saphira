const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getStakeAccount, getNatixTokenId, getProgramId, getTestReceiverUserTokenAccount, getTestReceiverUserTokenAccountPriv, getStakeProgramId, getStakeProgramAccountPubKey, connectionUrl} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection(connectionUrl, "confirmed");
    const programAccount = getProgramAccount().publicKey;    
    const stakeAccount = getStakeAccount();
    const natixToken = getNatixTokenId();
    const programId = getProgramId();
    const payer = getTestReceiverUserTokenAccountPriv();
    const userAccount = getTestReceiverUserTokenAccount();
    const stakeProgramId = getStakeProgramId();
    const stakeProgramAccount = getStakeProgramAccountPubKey();
    const pdaAccount = solanaWeb3.PublicKey.findProgramAddressSync(['NATIX'], programId)[0];
    

    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programAccount, isSigner: false, isWritable: true},            
            {pubkey: userAccount, isSigner: false, isWritable: true},            
            {pubkey: payer.publicKey, isSigner: true, isWritable: true},
            {pubkey: stakeProgramId, isSigner: false, isWritable: false},
            {pubkey: natixToken, isSigner: false, isWritable: true},
            {pubkey: stakeProgramAccount, isSigner: false, isWritable: true},
            {pubkey: stakeAccount, isSigner: false, isWritable: true},
            {pubkey: pdaAccount, isSigner: false, isWritable: true},
        ],
        programId,
        data: Buffer.from(getInstruction(2, "test", "1", undefined)),
    });

    const computeBudgetIx = solanaWeb3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 300000, 
        additionalFee: 0, 
      });


    const transaction = new solanaWeb3.Transaction().add(instruction).add(computeBudgetIx);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payer],
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
