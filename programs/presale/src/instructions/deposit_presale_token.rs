use crate::*;
use anchor_spl::{ 
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};

#[derive(Accounts)]
pub struct DepositPresaleToken<'info> {
    #[account(
        mut,
        constraint = global_state.admin == *admin.key @PresaleError::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    //  Global state
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    //  presale token ata of admin
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = admin,
    )]
    pub token_admin: Box<Account<'info, TokenAccount>>,

    //  store tokens to be sold
    #[account(
        mut, 
        associated_token::mint = token,
        associated_token::authority = global_state,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: 
    pub token: Account<'info, Mint>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl DepositPresaleToken<'_> {
    pub fn process_instruction(ctx: Context<Self>, amount: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        global_state.total_amount = global_state.total_amount + amount;

        //  transfer presale token from admin ata to program ata
        token_transfer_user(
            ctx.accounts.token_admin.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.token_vault.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            amount,
        )
    }
}
