const solanaWeb3 = require('@solana/web3.js');
const splToken = require('@solana/spl-token');
const {AuthorityType} = require("@solana/spl-token");
const {getNatixTokenId, getStakeAccount, getStakePrivKey, getProgramId, getPayer} = require("./constants");
(async () => {
    const connection = new solanaWeb3.Connection("https://api.devnet.solana.com", "confirmed");
    const token_id = getNatixTokenId();
    const stakeAccount = getStakeAccount();
    const stakePrivKey = getStakePrivKey();
    const programId = getProgramId();
    const payer = getPayer();
    const pdaAccount = solanaWeb3.PublicKey.findProgramAddressSync(['NATIX'], programId)[0];

    console.log(`pdaAccount is ${pdaAccount}`);

    const instruction = splToken.createSetAuthorityInstruction(
        stakeAccount,
        stakePrivKey.publicKey,
        AuthorityType.AccountOwner,
        pdaAccount,
        [],
        token_id
    )



    const transaction = new solanaWeb3.Transaction().add(instruction);
    console.log('sending transaction...')
    const signature = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, stakePrivKey]
    );

    console.log(`Transaction confirmed with signature: ${signature}`);
})();
