use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
    

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// This is the API that can be reached by Typescript client
#[program]
pub mod freecell {
    use super::*;

    pub fn setup_game() -> () {
        instructions::setup_game::setup_game(ctx);
    }
    pub fn _start_game() -> () {
    }
}