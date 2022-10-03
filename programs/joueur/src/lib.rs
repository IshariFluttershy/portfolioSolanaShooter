use anchor_lang::prelude::*;

declare_id!("9xrk3h285DTjDVLewTfwLekYibvpRSTq2a8GjCmF4gqi");

#[program]
pub mod joueur {
    use super::*;

    pub fn init_user_account(ctx: Context<InitUserAccount>) -> Result <()> {
        let user_account = &mut ctx.accounts.user_account;
    
        user_account.enemies_added = 0;
        user_account.bump = *ctx.bumps.get("user_account").unwrap();
    
        Ok(())
    }

    pub fn add_enemy_stat(ctx: Context<AddEnemyStat>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.enemies_added += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUserAccount<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1, seeds = [b"user_account", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddEnemyStat<'info> {
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"user_account", authority.key().as_ref()], bump = user_account.bump)]
    pub user_account: Account<'info, UserAccount>,
}

#[account]
pub struct UserAccount {
    pub saved_score: u32,
    pub enemies_added: u32,
    pub bump: u8,
}
