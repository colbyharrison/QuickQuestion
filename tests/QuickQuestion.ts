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

  let bountyMint: spl.Token;
  let bountyTokens: anchor.web3.PublicKey;

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

    bountyTokens = await bountyMint.createAssociatedTokenAccount(program.provider.wallet.publicKey);

    await bountyMint.mintTo(bountyTokens, program.provider.wallet.publicKey, [], 1000);
  });

  it('Bounty made', async () => {
    // Add your test here.
    const tx = await program.rpc.makeBounty({});
    console.log("Your transaction signature", tx);
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
