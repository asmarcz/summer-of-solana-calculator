use anchor_lang::prelude::*;

declare_id!("H5sGxN4aqyuH3StHo1pBXQZNzBgX56jEK7XRakSWVm41");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
