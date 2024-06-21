const web3 = require('@solana/web3.js');
const secretKey = [129,36,13,18,86,252,173,177,44,177,133,119,167,113,82,162,205,32,125,203,227,228,144,128,113,109,215,198,233,85,46,188,185,191,179,119,160,186,97,62,25,35,231,125,25,60,232,164,179,202,219,38,193,3,8,143,170,28,240,113,32,149,101,200];

const programPublicKey = '97G6dkNXUL3CWTPvnLGBA7pDcKSYrWjZWs2A3SFk3JSh';

const space = 3000000;

const connection = new web3.Connection(
  web3.clusterApiUrl('devnet'),
  'confirmed',
);

const programId = new web3.PublicKey(programPublicKey);

const payer = web3.Keypair.fromSecretKey(
  Uint8Array.from(secretKey)
);


async function createAndInitializeAccount() {
  const newAccount = web3.Keypair.generate();
  const lamports = await connection.getMinimumBalanceForRentExemption(space);
  console.log(`lamports is ${lamports}`);
/*  const transaction = new web3.Transaction().add(
    web3.SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: newAccount.publicKey,
      lamports: lamports,
      space: space,
      programId: programId,
    }),
    new web3.TransactionInstruction({
      keys: [{pubkey: newAccount.publicKey, isSigner: false, isWritable: true}],
      programId,
      data: Buffer.from([]),
    })
  );

  let signature = await web3.sendAndConfirmTransaction(
    connection,
    transaction,
    [payer, newAccount]
  );

  console.log(`Account ${newAccount.publicKey} created and initialized. Transaction Signature: ${signature}`);
*/
}

createAndInitializeAccount().catch(console.error);

