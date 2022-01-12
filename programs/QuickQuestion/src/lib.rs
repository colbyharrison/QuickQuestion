use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("3nwtVSUXMNeFYDA4dXrSLb8qYwMXwm1CaEoho5PRW2aH");

#[program]
pub mod quick_question {
    use super::*;
    pub fn make_bounty(ctx: Context<MakeBounty>) -> ProgramResult {
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
pub struct MakeBounty<'info> {
    #[account(init, payer = questioner, space = 28000)]
    bounty: AccountLoader<'info, Bounty>,
    #[account(signer)]
    questioner: Account<'info, Questioner>,
    #[account(
        init,
        payer = questioner,
        seeds = [bounty.key().as_ref()],
        bump = bounty_tokens_bump,
        token::mint = bounty_mint,
        token::authority = bounty_tokens,
    )]
    bounty_tokens: Account<'info, TokenAccount>, //here we store the bounty tokens
    bounty_mint: Account<'info, Mint>, //the minter for the token, not sure this is really needed...
    token_program: Program<'info, Token>,
    system_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
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
