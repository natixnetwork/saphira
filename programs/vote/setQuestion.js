const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getProgramId, getPayer, getProgramAuth, connectionUrl} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection(connectionUrl, "confirmed");
    const programAccount = getProgramAccount().publicKey;
    const auth = getProgramAuth();    
    const programId = getProgramId();
    const payer = getPayer();

    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programAccount, isSigner: false, isWritable: true},            
            {pubkey: auth.publicKey, isSigner: true, isWritable: false},
        ],
        programId,
        data: Buffer.from(getInstruction(1, undefined, undefined, {
            id: "test",
            answers: ["1", "2", "3", "4"],
            stats: [0, 0, 0, 0],
            expired_at: 1749302641000, 
        })),
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
