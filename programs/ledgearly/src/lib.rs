use anchor_lang::prelude::*;

declare_id!("4WnHBRqFC12JgAfXAf4QfsgaWum5GDF3UYWfcDkj9dJd");

#[program]
pub mod ledgearly {
    use super::*;
    pub fn initialize_list(ctx: Context<InitializeList>) -> Result<()> {
    let list = &mut ctx.accounts.expense_list;
    list.expenses = Vec::new();
    Ok(())
}

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// pub struct Initialize {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Expense {
    pub name: String,
    pub cost: u64,
}

#[account]
pub struct ExpenseList {
    pub expenses: Vec<Expense>,
}
#[account(
    init,
    payer = user,
    space = 600, // safe for 10 expenses
    seeds = [b"expense_list", user.key().as_ref()],
    bump
)]
pub expense_list: Account<'info, ExpenseList>,