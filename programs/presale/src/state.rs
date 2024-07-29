use crate::*;

/**
 * Stores global state of the program
 */
#[account]
#[derive(Default)]
pub struct GlobalState {
    //  admin address of this program
    pub admin: Pubkey,

    //  address of multi sig wallet
    pub vault: Pubkey,


    //  total tokens sold
    pub token_sold: u64,

    //  total USD recived
    pub token_sold_usd: u64,


    // presale is live or not?
    pub is_live: bool,


    // stage number of current stage
    pub stage_iterator: u8,

    //  remain token amounts on each stage
    pub remain_tokens: [u64; NUM_STAGES as usize]
}

impl GlobalState {
    // Function to update remain_tokens from STAGES
    pub fn update_remain_tokens(&mut self) {
        for (i, stage) in STAGES.iter().enumerate() {
            self.remain_tokens[i] = stage.amount;
        }
    }
}
