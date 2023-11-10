use anchor_lang::prelude::*;

declare_id!("6oTDeVZoZCjXvkPMcKnmi6Gn37rKiSSvwRUQKTENbLs8");

#[program]
pub mod exp_one {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
