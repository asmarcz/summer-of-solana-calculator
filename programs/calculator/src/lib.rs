use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("H5sGxN4aqyuH3StHo1pBXQZNzBgX56jEK7XRakSWVm41");

#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }
}

#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
