use crate::state::game::*;

pub fn setup_game(ctx: Context<SetupGame>) -> Result<()> {
    ctx.accounts
        .game
        .start();
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = authority,space = )]
    pub deck: Account<'info, Game>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
