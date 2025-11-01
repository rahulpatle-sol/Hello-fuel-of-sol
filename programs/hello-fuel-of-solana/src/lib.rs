use anchor_lang::prelude::*;

declare_id!("Cxc6wRrQrWzErAEBDLdphVxqZrxbq3iNTbf3bqrRs5jm");

#[program]
pub mod hello_fuel_of_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello  fuel of sol  Build the sol  stuff: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
