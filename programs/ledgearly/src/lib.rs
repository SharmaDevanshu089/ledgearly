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

    pub fn add_expense(ctx: Context<AddExpense>, name: String, cost: u64) -> Result<()> {
        let list = &mut ctx.accounts.expense_list;

        // Limit check: maximum 10 expenses
        require!(list.expenses.len() < 10, ErrorCode::ListFull);

        list.expenses.push(Expense { name, cost });

        Ok(())
    }
}

// Data Structure for a single expense
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Expense {
    pub name: String,
    pub cost: u64,
}

// Account Structure (The "Storage")
#[account]
pub struct ExpenseList {
    pub expenses: Vec<Expense>,
}

// Context for initializing the list
#[derive(Accounts)]
pub struct InitializeList<'info> {
    #[account(
        init,
        payer = user,
        space = 600, // Consider calculating strictly: 8 discriminator + 4 vec prefix + (item_size * 10)
        seeds = [b"expense_list", user.key().as_ref()],
        bump
    )]
    pub expense_list: Account<'info, ExpenseList>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Context for adding an expense
#[derive(Accounts)]
pub struct AddExpense<'info> {
    #[account(
        mut,
        seeds = [b"expense_list", user.key().as_ref()],
        bump
    )]
    pub expense_list: Account<'info, ExpenseList>,

    pub user: Signer<'info>,
}

// Custom Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("You have reached the maximum of 10 expenses")]
    ListFull,
}