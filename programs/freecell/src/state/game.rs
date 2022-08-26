use anchor_lang::account;
use super::{cards::Card, cells::Cells};
use anchor_lang::{AnchorDeserialize, AnchorSerialize, prelude::{Pubkey, borsh, borsh::{BorshDeserialize, BorshSerialize}}};


#[account]
pub struct Game {
    cells: Cells,
    cards: Card,
    state: GameState,
}

#[derive(Debug, PartialEq, Copy, Clone, BorshSerialize, BorshDeserialize)]
struct GameState {
    
}


impl Game {

    pub const MAXIMUM_SIZE: usize = 1;

    pub fn initialize() {
        
    }

    pub fn start(&mut self){

    }
}


