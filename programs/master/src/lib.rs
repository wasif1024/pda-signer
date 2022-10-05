use anchor_lang::prelude::*;
use childmasterpda::cpi::accounts::SetData;
use childmasterpda::program::Childmasterpda;
use childmasterpda::{self,Data};
declare_id!("2oQLML5jy1ydfaXAJAHoqr661yeumX6U4N3ESrDg1vdJ");

#[program]
pub mod master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, bump: u8, data: u64) -> Result<()> {
        /*let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            child: ctx.accounts.data.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        //puppet::cpi::set_data(cpi_ctx, data)
        child::cpi::set_data(cpi_ctx, data);*/
        let bump = &[bump][..];
        childmasterpda::cpi::set_data(ctx.accounts.set_data_ctx().with_signer(&[&[bump][..]]), data)?;
        //This reload will load latest data of accounts after setting
        
        ctx.accounts.child.reload()?;
        if ctx.accounts.child.data != 42 {
            panic!();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub child: Account<'info, Data>,
    pub child_program: Program<'info, Childmasterpda>,
     // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    /// CHECK:not infer signers from programs called via CPIs
    pub authority: UncheckedAccount<'info>,
}
impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.child_program.to_account_info();
        let cpi_accounts = SetData {
            child: self.child.to_account_info(),
            authority:self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

