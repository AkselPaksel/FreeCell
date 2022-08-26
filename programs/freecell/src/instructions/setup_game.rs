use anchor_lang::{prelude::*, Accounts, system_program::System};
use crate::state::game::Game;


pub fn setup_game(ctx: Context<SetupGame>, random_number: f32) -> () {
    ctx.accounts
        .game
        .start();
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = authority,space =1 )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}