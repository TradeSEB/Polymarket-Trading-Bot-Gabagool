use crate::*;
use anchor_spl::{ 
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    //  Global state
    #[account(
        init,
        space = 8 + std::mem::size_of::<GlobalState>(),
        seeds = [GLOBAL_SEED],
        bump,
        payer = admin
    )]
    pub global_state: Account<'info, GlobalState>,

    //  store tokens to be sold
    #[account(
        init, 
        payer = admin,
        associated_token::mint = token,
        
        //  Authority is set to global_state
        associated_token::authority = global_state,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    pub token: Account<'info, Mint>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    //  Needed to init new account
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn process_instruction(ctx: Context<Self>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        msg!("{}", global_state.key());

        //  initialize global state
        global_state.admin = ctx.accounts.admin.key();
        global_state.vault = Pubkey::default();

        global_state.token = ctx.accounts.token.key();
        global_state.total_amount = 0;
        
        global_state.token_sold = 0;
        global_state.token_sold_usd = 0;
        
        global_state.is_live = false;

        global_state.stage_iterator = 0;

        
        global_state.remain_tokens = [0; NUM_STAGES as usize];

        // Update remain_tokens from STAGES
        global_state.update_remain_tokens();

        Ok(())
    }
}
