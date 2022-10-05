use anchor_lang::prelude::*;

declare_id!("4KHdwrNqmC7gMeeGpWYKdxyjScArBnmP2wsgg8cAY1aJ");

#[program]
pub mod childmasterpda {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        _ctx.accounts.child.authority=authority;
        Ok(())
    }


    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let child = &mut ctx.accounts.child;
        child.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8+32)]
    pub child: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub child: Account<'info, Data>,
    pub authority: Signer<'info>
}


#[account]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey
}

