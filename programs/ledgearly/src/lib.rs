use anchor_lang::prelude::*;

declare_id!("4WnHBRqFC12JgAfXAf4QfsgaWum5GDF3UYWfcDkj9dJd");

#[program]
pub mod ledgearly {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
