import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { NodeWallet } from '@project-serum/anchor/dist/cjs/provider';
import * as assert from 'assert';
import { QuickQuestion } from '../target/types/quick_question';
import TransactionFactory from '@project-serum/anchor/dist/cjs/program/namespace/transaction';

describe('QuickQuestion', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QuickQuestion as Program<QuickQuestion>;

  let bountyMint: spl.Token;
  let questionerTokens: anchor.web3.PublicKey;

  let responder = anchor.web3.Keypair.generate();

  let responderMint: spl.Token;
  let responderTokens: anchor.web3.PublicKey;
  let bounty: anchor.web3.Keypair;
  let answer: anchor.web3.Keypair;

  let bountyTokens: anchor.web3.PublicKey;
  let bountiedTokensBump: Number;


  before(async () => {


    const wallet = program.provider.wallet as NodeWallet;
    bountyMint = await spl.Token.createMint(
      program.provider.connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      0,
      spl.TOKEN_PROGRAM_ID
    );

    questionerTokens = await bountyMint.createAssociatedTokenAccount(program.provider.wallet.publicKey);

    await bountyMint.mintTo(questionerTokens, program.provider.wallet.publicKey, [], anchor.web3.LAMPORTS_PER_SOL * 10);


    responderTokens = await bountyMint.createAssociatedTokenAccount(responder.publicKey);
    await bountyMint.mintTo(responderTokens, program.provider.wallet.publicKey, [], anchor.web3.LAMPORTS_PER_SOL * 10);

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        responder.publicKey,
        10000000000
      ),
      "confirmed"
    );
  });

  it('Bounty posted', async () => {

    bounty = anchor.web3.Keypair.generate();

    [bountyTokens, bountiedTokensBump] = await anchor.web3.PublicKey.findProgramAddress(
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
        bountyMint: bountyMint.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [bounty]
    });

    const bnty = await program.account.bounty.fetch(bounty.publicKey);
    console.log(bnty);
    assert.ok(bnty.amount.eq(amount));
    assert.ok(bnty.openTime.eq(timeline));
    // assert.equal(bnty.state, "{ open: {} }");

    assert.equal(bnty.title, title);
    assert.equal(bnty.question, question);
    assert.ok(bnty.questionerKey.equals(program.provider.wallet.publicKey));

    const escrowedTokens = (await bountyMint.getAccountInfo(bountyTokens));
    assert.equal(anchor.web3.LAMPORTS_PER_SOL, escrowedTokens.amount.toNumber());
  });

  it('Bounty closed', async () => {
    // const tx = await program.rpc.closeBounty({});
    // console.log("Your transaction signature", tx);
  });

  it('Answer posted', async () => {
    answer = anchor.web3.Keypair.generate();

    const response = " This is an answer";
    const collateral = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);


    const tx = await program.rpc.postAnswer(response, collateral, {
      accounts: {
        answer: answer.publicKey,
        responder: responder.publicKey,
        bounty: bounty.publicKey,
        responderTokens: responderTokens,
        bountyTokens: bountyTokens,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [answer, responder]
    });
    const answerFetch = await program.account.answer.fetch(answer.publicKey);
    const bnty = await program.account.bounty.fetch(bounty.publicKey);

    assert.ok(bnty.responders[0].answerKey.equals(answer.publicKey));

    assert.ok(!answerFetch.wasAccepted);
    assert.ok(answerFetch.collateralAmount.eq(collateral));
    assert.equal(answerFetch.response, response);
    assert.ok(answerFetch.responderKey.equals(responder.publicKey));
    assert.ok(answerFetch.bountyKey.equals(bounty.publicKey));

    const escrowedTokens = (await bountyMint.getAccountInfo(bountyTokens));
    assert.equal(anchor.web3.LAMPORTS_PER_SOL * 2, escrowedTokens.amount.toNumber());
  });

  it('Answer posted when bounty closed', async () => { });

  it('Answer accepted', async () => {
    // Add your test here.
    const tx = await program.rpc.acceptAnswer({
      accounts: {
        bounty: bounty.publicKey,
        questioner: program.provider.wallet.publicKey,
        answer: answer.publicKey
      }
    });

    const answerFetch = await program.account.answer.fetch(answer.publicKey);
    const bnty = await program.account.bounty.fetch(bounty.publicKey);

    console.log(bnty);
    assert.ok(answerFetch.wasAccepted);
  });

  it('Bounty Responder Account Closed', async () => {
    const tx = await program.rpc.closeResponderAccount({
      accounts: {
        bounty: bounty.publicKey,
        questioner: program.provider.wallet.publicKey,
        responder: responder.publicKey,
        responderTokens: responderTokens,
        bountyTokens: bountyTokens,
        // bountyMint: bountyMint.publicKey, //dont need
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    });
    const bnty = await program.account.bounty.fetch(bounty.publicKey);
    console.log(bnty);

    const escrowedTokens = (await bountyMint.getAccountInfo(bountyTokens));
    assert.equal(0, escrowedTokens.amount.toNumber());
  });

});
