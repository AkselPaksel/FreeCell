use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;
    

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// This is the API that can be reached by Typescript clientell
#[program]
pub mod freecell {
    
    pub fn setup_game() -> Result<()> {
        instructions::setup_game::setup_game(ctx);
    }
    pub fn _start_game() -> Result<()> {
    }
}