use crate::*;
use anchor_spl::{ 
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount }
};
use pyth_sdk_solana::state::SolanaPriceAccount;
use solana_program::native_token::LAMPORTS_PER_SOL;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,

    //  Global state
    #[account(
        mut,
        seeds = [GLOBAL_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    
    #[account(
        mut
    )]
    /// CHECK: vault address(multi-sig wallet)
    pub vault: AccountInfo<'info>,

    //  store tokens to be sold
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = global_state,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    //  ata of user
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user,
    )]
    pub token_user: Box<Account<'info, TokenAccount>>,

    // token address
    #[account(
        constraint = global_state.token == token.key() @PresaleError::InvalidToken
    )]
    /// CHECK: 
    pub token: Account<'info, Mint>,

    
    #[account(address = SOL_USD_FEED @PresaleError::InvalidPriceFeed)]
    /// CHECK:
    pub price_feed: AccountInfo<'info>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    //  Needed to init new account
    pub system_program: Program<'info, System>,
}

impl Buy<'_> {
    pub fn process_instruction(ctx: Context<Self>, mut sol_amount: u64) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        let stage_iterator = global_state.stage_iterator;

        //  check presale is live
        if global_state.is_live == false {
            match stage_iterator {
                0 => return Err(error!(PresaleError::PresaleNotStarted)),
                stage if stage > NUM_STAGES => return Err(error!(PresaleError::PresaleEnded)),
                _ => return Err(error!(PresaleError::PresalePaused)),
            }
        }


        //  get SOL price
        // Retrieve Pyth price
        let price_account_info = &ctx.accounts.price_feed;
        let price_feed: pyth_sdk_solana::PriceFeed = SolanaPriceAccount::account_info_to_feed(price_account_info).unwrap();
        let timestamp = Clock::get()?.unix_timestamp;
        let asset_price = price_feed.get_price_no_older_than(timestamp, 60).unwrap().price;
        
        // Scale price to expected decimals
        // let asset_expo = asset_price.expo;
        // asset_price = asset_price.scale_to_exponent(asset_expo).unwrap();

        msg!("SOL/USD price: {}", asset_price);

        //  calculate token amount from sol_amount and token price
        let mut token_amount = (asset_price as f64 / 100.0 / STAGES[stage_iterator as usize].price as f64 * (sol_amount / LAMPORTS_PER_SOL) as f64) as u64;
        msg!("token amount: {}", token_amount);


        //  check if enough token remain
        if global_state.remain_tokens[stage_iterator as usize] < token_amount {
            msg!("not enough tokens in this stage");

            //  fix token_amount and sol_amount
            token_amount = global_state.remain_tokens[stage_iterator as usize];
            sol_amount = (STAGES[stage_iterator as usize].price as f64 / (asset_price as f64 / 100.0) * (LAMPORTS_PER_SOL * token_amount) as f64) as u64;

            //  go to next stage
            global_state.stage_iterator = stage_iterator + 1;

            //  end presale if current stage is the last one
            if stage_iterator == NUM_STAGES {
                global_state.is_live = false;
            }
        }


        //  minus remain tokens in the current stage
        global_state.remain_tokens[stage_iterator as usize] -= token_amount;

        //  add total tokens sold
        global_state.token_sold += token_amount;

        //  add total USD received
        global_state.token_sold_usd += asset_price as u64 * sol_amount;
        

        //  transfer SOL to vault
        sol_transfer_user(
            ctx.accounts.user.to_account_info().clone(),
            ctx.accounts.vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            sol_amount
        )?;

        //  transfer token to user
        token_transfer_with_signer(
            ctx.accounts.token_vault.to_account_info(),
            ctx.accounts.global_state.to_account_info(),
            ctx.accounts.token_user.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            &[&[GLOBAL_SEED.as_ref(), &[ctx.bumps.global_state]]],
            token_amount * TOKEN_DECIMALS,
        )?;

        Ok(())
    }
}
