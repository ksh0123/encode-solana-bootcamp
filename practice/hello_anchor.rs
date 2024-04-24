use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
// Imports the TokenAccount struct, used to interact with token accounts on Solana.

declare_id!("111111111111111111111111");

#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        // If token_account balance is greater than 0, update the data field in MyAccount struct
        if ctx.accounts.token_account.amount > 0 {
            ctx.accounts.my_account.data = data;
        }

        Ok(())
    }
}

#[account]
// Defines the data structure saved in Solana's account storage
#[derive(Default)]
pub struct MyAccount {
    data: u64,
    // Specifies the minted token account
    mint: Pubkey,
}

// Prepares SetData struct to specify the account configurations needed for the set_data function
#[derive(Accounts)]
pub struct SetData<'info> {
    // Makes my_account mutable
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(
        // Ensures my_account and token_account are handling the same token account
        // Before an data is modified, all the constraints are checked
        constraint = my_account.mint == token_account.mint,
        // Identifier of an account or field within the contract that the account being defined is expected to be associated with.
        // Only the actual owner of the token_account can execute operations that involve this account
        has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    // Indicates owner is a signing participant in the transaction and provides necessary permissions
    pub owner: Signer<'info>
}
