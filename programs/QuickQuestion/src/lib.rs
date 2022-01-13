use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("G2HtxxNKWLSLjxL6dGf3Y9SgQtmaANMSZqUmPAugHu1r");

#[program]
pub mod quick_question {
    use super::*;
    pub fn post_bounty(ctx: Context<PostBounty>, bump: u8) -> ProgramResult {
        Ok(())
    }

    pub fn create_bounty_history(ctx: Context<CreateBountyHistory>) -> ProgramResult {
        Ok(())
    }

    pub fn close_bounty(ctx: Context<CloseBounty>) -> ProgramResult {
        Ok(())
    }

    pub fn accept_answer(ctx: Context<AcceptAnswer>) -> ProgramResult {
        Ok(())
    }

    pub fn post_answer(ctx: Context<PostAnswer>) -> ProgramResult {
        Ok(())
    }

    // TODO: Not completely trustless if I need to rely on the client to call
    // close when time is up. See lockup example it has a reference to time
}

#[derive(Accounts)]
#[instruction(bounty_tokens_bump: u8)]
pub struct PostBounty<'info> {
    #[account(init, payer = questioner, space = 500)]
    bounty: Account<'info, Questioner>,
    #[account(mut)]
    questioner: Signer<'info>,
    questioner_tokens: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = questioner,
        seeds = [bounty.key().as_ref()],
        bump = bounty_tokens_bump,
        token::mint = bounty_mint,
        token::authority = bounty_tokens, //we're using the token account as authority over itself...
    )]
    bounty_tokens: Account<'info, TokenAccount>, //here we store the bounty tokens
    bounty_mint: Account<'info, Mint>, //the minter for the token, not sure this is really needed...
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateBountyHistory<'info> {
    #[account(zero)]
    bounty: AccountLoader<'info, Bounty>,
}

#[derive(Accounts)]
pub struct CloseBounty {}

#[derive(Accounts)]
pub struct AcceptAnswer {}

#[derive(Accounts)]
pub struct PostAnswer {}

#[account]
pub struct Questioner {
    name: String,
    authority: Pubkey,
    bump: u8,
}

#[account]
#[derive(Default)]
pub struct Responder {
    name: String,
    authority: Pubkey,
    bump: u8,
}

//as these accounts will be quite large we must use zero-copy as to not violate
//stack and heap limitations. 4KB Stack and 32 KB heap
#[account(zero_copy)]
pub struct Bounty {
    pub title: [u8; 50],      //limit to 50 chars
    pub question: [u8; 2500], //limit to 2500 chars
    pub amount: u64,
    pub open_time: u64,
    pub answers: [Answer; 10], //10 answers total 25,330
    pub is_open: bool,
    pub questioner_key: Pubkey,
    pub bounty_tokens_bump: u8,
}

#[zero_copy]
pub struct Answer {
    //byte requirement = 2500+32+1 = 2533
    pub response: [u8; 2500], //limit to 2500 chars
    pub reponder_key: Pubkey,
    was_accepted: bool,
}
