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
    #[account(init, payer = questioner, space = 6780)]
    //TODO Future improvement to move history offchain
    bounty: Account<'info, Bounty>,
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
    bounty_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
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
#[derive(Default)]
pub struct Responder {
    name: String,
    authority: Pubkey,
    bump: u8,
}

#[account]
pub struct Answer {
    //total 283
    pub response: String, //limit to 250 chars
    pub reponder_key: Pubkey,
    was_accepted: bool,
}

#[account]
pub struct Bounty {
    //total bytes 50 + 1000 +8 +8 +5660 + 1 + 32 + 1 = 6760
    pub title: String,    //limit to 50 chars
    pub question: String, //limit to 1000 chars
    pub amount: u64,
    pub open_time: u64,
    pub answers: Vec<String>, //20 answers total 5660
    pub is_open: bool,
    pub questioner_key: Pubkey,
    pub bounty_tokens_bump: u8,
}

// pub struct Bounty {
//     pub title: [u8; 50],     //limit to 50 chars
//     pub question: [u8; 500], //limit to 1000 chars
//     pub amount: u64,
//     pub open_time: u64,
//     pub answers: [Answer; 20], //20 answers total 5660
//     pub is_open: bool,
//     pub questioner_key: Pubkey,
//     pub bounty_tokens_bump: u8,
// }
// //Only Size Enough for 20 answers @ 250 chars each
// pub struct Answer {
//     //byte requirement = 250+32+1 = 283
//     pub response: [u8; 250], //limit to 250 chars
//     pub reponder_key: Pubkey,
//     was_accepted: bool,
// }
