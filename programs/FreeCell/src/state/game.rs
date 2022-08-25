use anchor_lang::account;


#[account]
pub struct Game {
    cells: Cells,
    cards: Card,
    state: GameState,
}

impl Game {

    pub const MAXIMUM_SIZE: usize = 1;

    pub fn initialize() {
        
    }

    pub fn start(&mut self){
        
    }
}