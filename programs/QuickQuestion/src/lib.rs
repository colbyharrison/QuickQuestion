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
        bounty_tokens: u64,
        bounty_timeline: u64, //TODO this needs to be time
    ) -> ProgramResult {
        let bounty = &mut ctx.accounts.bounty;
        bounty.title = title;
        bounty.responders = Vec::new();
        bounty.question = question;
        bounty.amount = bounty_tokens;
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
            bounty_tokens,
        )
    }

    pub fn close_bounty(ctx: Context<CloseBounty>) -> ProgramResult {
        let bounty = &ctx.accounts.bounty;

        require!(
            bounty.state == BountyState::Open || bounty.state == BountyState::Accepted,
            BountyError::CantCloseClosedBounty
        );

        if bounty.state == BountyState::Open { //This should only occur @ or after deadline
        }

        if bounty.state == BountyState::Accepted { //Closing after accepting answer

            //find accounts by pubkey?
        }

        //1 check state of bounty
        //2 send collateral back to resp
        //3 send bounty to winner
        //4 send fees to me
        //5 close accounts
        // for responder in bounty.answers.iter() {
        //     anchor_spl::token::transfer(
        //         CpiContext::new(
        //             ctx.accounts.token_program.to_account_info(),
        //             anchor_spl::token::Transfer {
        //                 from: ctx.accounts.questioner_tokens.to_account_info(),
        //                 to: ctx.accounts.bounty_tokens.to_account_info(),
        //                 authority: ctx.accounts.questioner.to_account_info(),
        //             },
        //         ),
        //         bounty_tokens,
        //     )
        // }

        Ok(())
    }

    pub fn post_answer(
        ctx: Context<PostAnswer>,
        response: String,
        collateral_amount: u64,
    ) -> ProgramResult {
        let answer = &mut ctx.accounts.answer;
        let bounty = &mut ctx.accounts.bounty;

        require!(
            bounty.state == BountyState::Open,
            BountyError::CantAnswerClosedBounty
        );
        answer.response = response;
        answer.responder_key = ctx.accounts.responder.key();
        answer.was_accepted = false;
        answer.collateral_amount = collateral_amount;
        answer.bounty_key = bounty.key();

        let responder = ResponderInfo {
            responder_key: answer.responder_key,
            answer_key: answer.key(),
            collateral_amount: collateral_amount,
            was_accepted: false,
        };

        bounty.responders.push(responder);
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
            collateral_amount,
        )
    }

    pub fn accept_answer(ctx: Context<AcceptAnswer>) -> ProgramResult {
        let accepted_answer = &mut ctx.accounts.answer;
        let bounty = &mut ctx.accounts.bounty;

        require!(
            bounty.state == BountyState::Open,
            BountyError::CantAcceptClosedBounty
        );

        let responder = bounty
            .responders
            .iter_mut()
            .find(|&&mut x| x == accepted_answer.key());

        require!(responder.is_some(), BountyError::AnswerNotFound);
        responder.unwrap().was_accepted = true; //already explicitly handled none case
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
    #[account(init, payer = questioner, space = 4758)]
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

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAnswer<'info> {
    #[account(mut)]
    bounty: Account<'info, Bounty>,
    questioner: Signer<'info>,
    #[account(mut)]
    answer: Account<'info, Answer>,
}

#[derive(Accounts)]
pub struct CloseBounty<'info> {
    #[account(mut)]
    bounty: Account<'info, Bounty>,
    questioner: Signer<'info>, //I want to be able to close the account without questioner needing to approve
    #[account(
        mut,
        seeds = [bounty.key().as_ref()],
        bump = bounty.bounty_tokens_bump,
    )]
    bounty_tokens: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
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
    //total bytes 50 + 1000 +8 +8 +2000 + 1 + 32 + 1 = 4750
    title: String,    //limit to 50 chars
    question: String, //limit to 1000 chars
    amount: u64,
    open_time: u64,
    responders: Vec<ResponderInfo>, //50 answers total 50 * 73 = 3650
    state: BountyState,
    questioner_key: Pubkey,
    bounty_tokens_bump: u8,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ResponderInfo {
    responder_key: Pubkey,
    answer_key: Pubkey,
    collateral_amount: u64,
    was_accepted: bool,
}

impl PartialEq<Pubkey> for ResponderInfo {
    fn eq(&self, other: &Pubkey) -> bool {
        self.answer_key == other.key()
    }
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum BountyState {
    Open,
    Closed,
    Accepted,
}

#[error]
pub enum BountyError {
    #[msg("Can't Accept Closed Bounty")]
    CantAcceptClosedBounty,
    #[msg("Can't Close Closed Bounty")]
    CantCloseClosedBounty,
    #[msg("Cant Answer Closed Bounty")]
    CantAnswerClosedBounty,
    #[msg("Answer Not Found")]
    AnswerNotFound,
}
