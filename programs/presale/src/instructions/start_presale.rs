use crate::*;
use anchor_spl::token::{ Mint, TokenAccount };

#[derive(Accounts)]
pub struct StartPresale<'info> {
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

    //  store tokens to be sold
    #[account(
        associated_token::mint = token,
        associated_token::authority = global_state,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    // token address
    #[account(
        constraint = global_state.token == token.key() @PresaleError::InvalidToken
    )]
    /// CHECK: 
    pub token: Account<'info, Mint>,
}

impl StartPresale<'_> {
    pub fn process_instruction(ctx: Context<Self>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        //  check total_amount is enough for those stages
        let sum: u64 = STAGES.iter().map(|stage| stage.amount).sum();
        require!(global_state.total_amount >= sum, PresaleError::NotEnoughToken);
        
        //  set is_live and stage_iterator
        global_state.is_live = true;
        global_state.stage_iterator = 0;

        Ok(())
    }
}
