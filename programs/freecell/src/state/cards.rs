
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use anchor_lang::{AnchorDeserialize, AnchorSerialize, prelude::{Pubkey, borsh, account, borsh::{BorshDeserialize, BorshSerialize}}};


 #[allow(dead_code)]
#[derive(Debug, PartialEq, EnumIter, Copy, Clone, BorshSerialize, BorshDeserialize)]
pub enum CardSuits {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Copy, Clone, BorshSerialize, BorshDeserialize)]
pub struct Card {
    pub suit: CardSuits,
    pub number: u32,
}

impl Card {
    pub fn new(new_suit: CardSuits, new_number: u32) -> Self {
        Card {
            suit: new_suit,
            number: new_number,
        }
    }
}

#[derive(Debug)]
#[account]
pub struct Deck {
    stack: [Option<Card>; 20],
}

impl Deck{
    pub fn new() -> Self {
        let cardsuits = CardSuits::iter();
        let mut stackvector = Vec::<Card>::new();
        for cardsuit in cardsuits {
            for number in (1..=13).into_iter() {
                stackvector.push(Card::new(cardsuit, number))
            }
        }
        Self {
            stack: stackvector
        }
    }
    pub fn shuffle_cards(mut deck: Deck, random_number: i32) -> [Option<Card>; 20] {
        deck.stack.shuffle(random_number);
        deck.stack
    }
}


#[test]
fn print_stack() {
    let mut deck= Deck::new();

    let random_number = 1;
    deck.stack.shuffle(random_number);

    println!("{:?}",  deck) // Has to be run as "cargo test -- --nocapture"
}