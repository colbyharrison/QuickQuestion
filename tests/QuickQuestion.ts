import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { NodeWallet } from '@project-serum/anchor/dist/cjs/provider';
import * as assert from 'assert';
import { QuickQuestion } from '../target/types/quick_question';

describe('QuickQuestion', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QuickQuestion as Program<QuickQuestion>;

  let questionerMint: spl.Token;
  let questionerTokens: anchor.web3.PublicKey;

  before(async () => {

    const wallet = program.provider.wallet as NodeWallet;
    questionerMint = await spl.Token.createMint(
      program.provider.connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      0,
      spl.TOKEN_PROGRAM_ID
    );

    questionerTokens = await questionerMint.createAssociatedTokenAccount(program.provider.wallet.publicKey);

    await questionerMint.mintTo(questionerTokens, program.provider.wallet.publicKey, [], anchor.web3.LAMPORTS_PER_SOL * 10);
  });

  it('Bounty posted', async () => {

    const bounty = anchor.web3.Keypair.generate();
    const [bountyTokens, bountiedTokensBump] = await anchor.web3.PublicKey.findProgramAddress(
      [bounty.publicKey.toBuffer()],
      program.programId
    );

    const title = "Example Title";
    const question = "Example question";
    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);
    const timeline = new anchor.BN(200);

    const tx = await program.rpc.postBounty(
      bountiedTokensBump, title, question, amount, timeline, {
      accounts: {
        bounty: bounty.publicKey,
        questioner: program.provider.wallet.publicKey,
        questionerTokens: questionerTokens,
        bountyTokens: bountyTokens,
        questionerMint: questionerMint.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [bounty]
    });

    const bnty = await program.account.bounty.fetch(bounty.publicKey);

    assert.ok(bnty.amount.eq(amount));
    assert.ok(bnty.openTime.eq(timeline));
    assert.ok(bnty.isOpen);
    assert.equal(bnty.title, title);
    assert.equal(bnty.question, question);
    assert.ok(bnty.questionerKey.equals(program.provider.wallet.publicKey))

    const escrowedTokens = (await questionerMint.getAccountInfo(bountyTokens))
    assert.equal(anchor.web3.LAMPORTS_PER_SOL, escrowedTokens.amount.toNumber())
  });

  it('Bounty closed', async () => {
    // Add your test here.
    const tx = await program.rpc.closeBounty({});
    console.log("Your transaction signature", tx);
  });

  it('Answer accepted', async () => {
    // Add your test here.
    const tx = await program.rpc.acceptAnswer({});
    console.log("Your transaction signature", tx);
  });

  it('Answer posted', async () => {
    // Add your test here.
    const tx = await program.rpc.postAnswer({});
    console.log("Your transaction signature", tx);
  });

});
