use anchor_lang::prelude::*;
use joueur::cpi::accounts::AddEnemyStat;
use joueur::program::Joueur;
use joueur::{self, UserAccount};

declare_id!("4jj12kCUbesxijcoXvfsm5KnsHAB1UFyLCTwH9vmLXBr");

#[program]
pub mod portfolioshooter {
  use super::*;
  pub fn init_base_account(ctx: Context<InitBaseAccount>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;

    base_account.enemies = 1;

    Ok(())
  }

  pub fn add_enemy(ctx: Context<AddEnemy>, bump: u8) -> Result <()> {
    joueur::cpi::add_enemy_stat(ctx.accounts.add_enemy_stat_ctx());
    let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;

    base_account.enemies += 1; 

    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitBaseAccount<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddEnemy<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user_account: Account<'info, UserAccount>,
  pub player_datas_program: Program<'info, Joueur>,
  
  pub authority: Signer<'info>,
}

impl<'info> AddEnemy<'info> {
  pub fn add_enemy_stat_ctx(&self) -> CpiContext<'_, '_, '_, 'info, AddEnemyStat<'info>> {
      let cpi_program = self.player_datas_program.to_account_info();
      let cpi_accounts = AddEnemyStat {
          user_account: self.user_account.to_account_info(),
          authority: self.authority.to_account_info(),
      };
      CpiContext::new(cpi_program, cpi_accounts)
  }
}

#[account]
pub struct BaseAccount {
    pub enemies: u32,
}