use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::ProgramError::InvalidArgument;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("H5sGxN4aqyuH3StHo1pBXQZNzBgX56jEK7XRakSWVm41");

#[program]
pub mod calculator {
    use BinaryOperator::*;
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn execute_bin_op(
        ctx: Context<BinaryOperation>,
        operator: BinaryOperator,
        lhs: i64,
        rhs: i64,
    ) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = match operator {
            Division => {
                if rhs == 0 {
                    return Err(InvalidArgument);
                } else { lhs / rhs }
            }
            Minus => lhs - rhs,
            Multiplication => lhs * rhs,
            Plus => lhs + rhs,
        };
        Ok(())
    }
}

#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum BinaryOperator {
    Division,
    Minus,
    Multiplication,
    Plus,
}

#[derive(Accounts)]
pub struct BinaryOperation<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
