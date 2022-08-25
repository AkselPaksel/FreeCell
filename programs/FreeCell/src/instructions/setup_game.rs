use anchor_lang::{prelude::{Context, Account, Signer, Program, Rent}, Accounts, system_program::System};
use crate::state::game::*;


pub fn setup_game(ctx: Context<SetupGame>) -> () {
    ctx.accounts
        .game
        .start();
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = authority, space = 1)]
    pub deck: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}