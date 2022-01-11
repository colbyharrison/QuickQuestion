import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { QuickQuestion } from '../target/types/quick_question';

describe('QuickQuestion', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.QuickQuestion as Program<QuickQuestion>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
