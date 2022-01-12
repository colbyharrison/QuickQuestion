use anchor_lang::prelude::*;

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
}

#[derive(Accounts)]
pub struct MakeBounty {}

#[derive(Accounts)]
pub struct CloseBounty {}

#[derive(Accounts)]
pub struct AcceptAnswer {}

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
}

#[account(zero_copy)]
pub struct Answer {
    //byte requirement = 2500+32+1 = 2533
    pub response: [u8; 2500], //limit to 2500 chars
    pub reponder_key: Pubkey,
    was_accepted: bool,
}
