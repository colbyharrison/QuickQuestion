use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("G2HtxxNKWLSLjxL6dGf3Y9SgQtmaANMSZqUmPAugHu1r");

#[program]
pub mod quick_question {
    use super::*;
    pub fn post_bounty(
        ctx: Context<PostBounty>,
        bump: u8,
        title: String,
        question: String,
        bounty_lamports: u64,
        bounty_timeline: u64,
    ) -> ProgramResult {
        let bounty = &mut ctx.accounts.bounty;
        bounty.title = title;
        bounty.question = question;
        bounty.amount = bounty_lamports;
        bounty.open_time = bounty_timeline;
        bounty.state = BountyState::Open;
        bounty.questioner_key = ctx.accounts.questioner.key();
        bounty.bounty_tokens_bump = bump;

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.questioner_tokens.to_account_info(),
                    to: ctx.accounts.bounty_tokens.to_account_info(),
                    authority: ctx.accounts.questioner.to_account_info(),
                },
            ),
            bounty_lamports,
        )
    }

    pub fn close_bounty(ctx: Context<CloseBounty>) -> ProgramResult {
        Ok(())
    }

    pub fn post_answer(
        ctx: Context<PostAnswer>,
        response: String,
        collateral_lamports: u64,
    ) -> ProgramResult {
        let answer = &mut ctx.accounts.answer;
        let bounty = &mut ctx.accounts.bounty;

        require!(bounty.state == BountyState::Open, BountyError::BountyClosed);
        answer.response = response;
        answer.responder_key = ctx.accounts.responder.key();
        answer.was_accepted = false;
        answer.collateral_amount = collateral_lamports;
        //answer.answer_tokens_bump = bump;

        bounty.answers.push(answer.key());
        answer.bounty_key = bounty.key();
        msg!["We got here solana logs ftw"];

        // transfer responder collateral into "escrow"
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.responder_tokens.to_account_info(),
                    to: ctx.accounts.bounty_tokens.to_account_info(),
                    authority: ctx.accounts.responder.to_account_info(),
                },
            ),
            collateral_lamports,
        )
    }

    pub fn accept_answer(ctx: Context<AcceptAnswer>) -> ProgramResult {
        let accepted_answer = &mut ctx.accounts.answer;
        let bounty = &mut ctx.accounts.bounty;

        require!(bounty.state == BountyState::Open, BountyError::BountyClosed);
        require!(
            bounty.answers.contains(&accepted_answer.key()),
            BountyError::AnswerNotFound
        );

        accepted_answer.was_accepted = true;
        bounty.state = BountyState::Accepted;

        Ok(())
    }

    // TODO: Not completely trustless if I need to rely on the client to call
    // close when time is up. See lockup example it has a reference to time
}

#[derive(Accounts)]
#[instruction(bounty_tokens_bump: u8)]
pub struct PostBounty<'info> {
    #[account(init, payer = questioner, space = 1708)]
    bounty: Account<'info, Bounty>,
    #[account(mut)]
    questioner: Signer<'info>,
    #[account(mut, constraint = questioner_tokens.mint == bounty_mint.key())]
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
//There must be some financial disincentive to posting willy nilly. The responder must pay some sol to answer
pub struct PostAnswer<'info> {
    #[account(init, payer = responder, space = 338)]
    answer: Account<'info, Answer>,
    #[account(mut)]
    responder: Signer<'info>,
    #[account(mut)]
    bounty: Account<'info, Bounty>,
    #[account(mut)]
    responder_tokens: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [bounty.key().as_ref()],
        bump = bounty.bounty_tokens_bump,
    )]
    bounty_tokens: Account<'info, TokenAccount>, //here we store the bounty tokens(from responder)
    bounty_mint: Account<'info, Mint>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CloseBounty {}

#[derive(Accounts)]
pub struct AcceptAnswer<'info> {
    #[account(mut)]
    bounty: Account<'info, Bounty>,
    questioner: Signer<'info>,
    #[account(mut)]
    answer: Account<'info, Answer>,
}

#[account]
pub struct Answer {
    //total 250 + 32 + 8 + 8 + 32 = 330
    response: String, //limit to 250 chars
    responder_key: Pubkey,
    was_accepted: bool,
    collateral_amount: u64,
    bounty_key: Pubkey,
}

#[account]
pub struct Bounty {
    //total bytes 50 + 1000 +8 +8 +1600 + 1 + 32 + 1 = 1700
    title: String,    //limit to 50 chars
    question: String, //limit to 1000 chars
    amount: u64,
    open_time: u64,
    answers: Vec<Pubkey>, //50 answers total 50 * 32 = 1600
    state: BountyState,
    questioner_key: Pubkey,
    bounty_tokens_bump: u8,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum BountyState {
    Open,
    Closed,
    Accepted,
}

#[error]
pub enum BountyError {
    #[msg("Bounty Closed")]
    BountyClosed,
    #[msg("Answer Not Found")]
    AnswerNotFound,
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
