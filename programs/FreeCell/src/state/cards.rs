use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use anchor_lang::prelude::{*, borsh::{BorshDeserialize, BorshSerialize}};


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
    stack: Vec<Card>,
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
    pub fn shuffle_cards(mut deck: Deck) -> Vec<Card> {
        let mut rng = rand::thread_rng();
        deck.stack.shuffle(&mut rng);
        deck.stack
    }
}


#[test]
fn print_stack() {
    let mut deck= Deck::new();
    // let mut secret_num: Vec<_> = (48..58).collect();
    let mut rng = rand::thread_rng();
    deck.stack.shuffle(&mut rng);

    println!("{:?}",  deck) // Has to be run as "cargo test -- --nocapture"
}