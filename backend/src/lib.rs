use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites() -> Result<()> {

    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}