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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Bounty {
    pub title: String,    //limit to 50 chars
    pub question: String, //limit to 2500 chars
    pub amount: u64,
    pub open_time: u64,
    pub answers: Vec<Answer>,
    pub state: State,
    pub questioner_key: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Answer {
    pub response: String, //limit to 2500 chars
    pub reponder_key: Pubkey,
    was_accepted: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum State {
    open,
    closed,
}