const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {getInstruction} = require("./programInstruction");
const {getProgramAccount, getStakeAccount, getNatixTokenId, getProgramId, getPayer, getProgramAuth} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const programAccount = getProgramAccount().publicKey;
    const programSecretKey = getProgramAccount();
    const auth = getProgramAuth();
    const stakeAccount = getStakeAccount();
    const natixToken = getNatixTokenId();
    const programId = getProgramId();
    const payer = getPayer();

    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [
            {pubkey: programAccount, isSigner: false, isWritable: true},
            {pubkey: stakeAccount, isSigner: false, isWritable: false},
            {pubkey: natixToken, isSigner: false, isWritable: false},
            {pubkey: auth.publicKey, isSigner: true, isWritable: false},
        ],
        programId,
        data: Buffer.from(getInstruction(8, 0, {
            min_stake_amount: 0,
            max_stake_amount: 100000000000,
            cooldown: 90,
            instant_penalty: 0.3,
            maximum_stakers: 1000,
            compound_period: 7,
            forfeit_amount: 0,
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
