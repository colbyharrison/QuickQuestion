import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { QuickQuestion } from '../target/types/quick_question';

describe('QuickQuestion', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QuickQuestion as Program<QuickQuestion>;

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
