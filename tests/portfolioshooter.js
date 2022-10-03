const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Portfolioshooter;
  const baseAccount = anchor.web3.Keypair.generate();
  const secondUserAccount = anchor.web3.Keypair.generate();

  console.log("📝 Before base account init");

  let tx = await program.rpc.initBaseAccount({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  
  console.log("📝 Before user account init");

  tx = await program.rpc.initUserAccount({
    accounts: {
      userAccount: provider.wallet.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Before second user account init");

  tx = await program.rpc.initUserAccount({
    accounts: {
      userAccount: secondUserAccount.publicKey,
      user: secondUserAccount.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  
  console.log("📝 Your transaction signature", tx);

  tx = await program.rpc.addEnemy({
    accounts: {
      baseAccount: baseAccount.publicKey,
      userAccount: provider.wallet.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  let userAccount = await program.account.userAccount.fetch(provider.wallet.publicKey);
  let userSecondAccount = await program.account.userAccount.fetch(secondUserAccount.publicKey);


  console.log('👀 Enemies Count', account.enemies.toString())
  console.log('👀 Enemies added by 1st user', userAccount.enemiesAdded.toString())
  console.log('👀 Enemies added by second user', userSecondAccount.enemiesAdded.toString())



  await program.rpc.addEnemy({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Enemies Count', account.enemies.toString())
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();