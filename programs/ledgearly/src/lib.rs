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
pub fn add_expense(
    ctx: Context<AddExpense>,
    name: String,
    cost: u64,
) -> Result<()> {
    let list = &mut ctx.accounts.expense_list;

    // limit check
    require!(list.expenses.len() < 10, ErrorCode::ListFull);

    list.expenses.push(Expense { name, cost });

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
    space = 600, //todo change later
    seeds = [b"expense_list", user.key().as_ref()],
    bump
)]
#[derive(Accounts)]
pub struct InitializeList<'info> {
    #[account(
        init,
        payer = user,
        space = 600,
        seeds = [b"expense_list", user.key().as_ref()],
        bump
    )]
    pub expense_list: Account<'info, ExpenseList>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("You have reached the maximum of 10 expenses")]
    ListFull,
}

pub expense_list: Account<'info, ExpenseList>,