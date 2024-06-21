const anchor = require('@project-serum/anchor');
const { SystemProgram } = require('@solana/web3.js');

describe('My Solana Program', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider("https://api.devnet.solana.com");

  it('Performs some action', async () => {
    const program = anchor.workspace.MySolanaProgram; // Adjust for your program
    const testAccount = anchor.web3.Keypair.generate();

    // Example: Create a new account
    await program.rpc.create({
      accounts: {
        myAccount: testAccount.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [testAccount],
    });

    // Fetch the newly created account
    const account = await program.account.myAccount.fetch(testAccount.publicKey);

    console.log(account);
  });
});

