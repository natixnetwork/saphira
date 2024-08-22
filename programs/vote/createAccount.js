const {
    Connection,
    PublicKey,
    Keypair,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,

} = require('@solana/web3.js');
const {getProgramAccount, getProgramId, getPayer, connectionUrl} = require("./constants");

(async () => {
    const connection = new Connection(connectionUrl, "confirmed");
    const newAccount = getProgramAccount();
    const programPublicKey = getProgramId();
    const length = 1024
    const payer = getPayer();
    console.log(newAccount.publicKey);
    let transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: newAccount.publicKey,
            lamports: await connection.getMinimumBalanceForRentExemption(length),
            space: length,
            programId: programPublicKey,
        }),
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [payer, newAccount]);

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
