use anchor_lang::prelude::*;

declare_id!("3nwtVSUXMNeFYDA4dXrSLb8qYwMXwm1CaEoho5PRW2aH");

#[program]
pub mod quick_question {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

pub struct Bounty {
    pub question: String,
    pub amount: u64,
    pub open_time: u64,
    pub answers: Vec<Answer>,
    pub state: State,
    pub questioner_key: Pubkey,
}

pub struct Answer {
    pub response: String,
    pub reponder_key: Pubkey,
    was_accepted: bool,
}

pub enum State {
    open,
    closed,
}
