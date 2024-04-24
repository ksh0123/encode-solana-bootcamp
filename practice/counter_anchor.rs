use anchor_lang::prelude::*;

declare_id!("111111111111111111111111");

// The Counter Program - defines the business logic only, states are not stored
// The accounts to perform the logic on are passed to the program
#[program]
pub mod counter_anchor {
    use super::*;

    // In Solidity, equal to `int private count = 0`
    // Endpoint is connected to the corresponding account through the `ctx` argument in the endpoint
    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        OK(())
    }

    // Endpoint is connected to the corresponding account through the `ctx` argument in the endpoint
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
        OK(())
    }
}

// The Counter Accounts
// Separate accounts that store the states outside of the program
#[derive(Accounts)]
// `'info` indicates the lifetime of the information
// Creates an account with a count of 0
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        space = 8 + Counter::INIT_SPACE),
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
}