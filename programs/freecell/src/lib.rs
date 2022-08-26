use anchor_lang::{AnchorSerialize, AnchorDeserialize, program, declare_id, prelude::{AccountInfo, borsh, Context, Pubkey, Rent}};
use std::io::Result;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod instructions;
pub mod state;
// This is the API that can be reached by Typescript client
#[program]
mod freecell {

    use super::*;

    pub fn _setup_game(ctx: Context<instructions::setup_game::SetupGame>, random_number: f32) -> Result<()> {
        instructions::setup_game::setup_game(ctx, random_number);
        Ok(())

    }
}