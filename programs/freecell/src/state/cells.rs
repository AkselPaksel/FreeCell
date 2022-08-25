
use anchor_lang::prelude::*;

use super::cards::Card;

#[account]
pub struct Cells {
    freecells: ( [Option<Card>; 4] ),
    targetcells: ( [Option<Card>; 4] ),
    cardcells: ( [Option<[Option<Card>; 20]>; 8] ),
}

impl Cells {
    pub fn new (deck: [Option<Card>; 20]) -> Cells {
        let mut xs: [Option<[Option<Card>; 20]>; 8] = Default::default();
        let mut cell_increment = 7;
        // Adding cards from the deck to each of the 8 CardCells in order
        deck.iter().for_each(|card|{
            cell_increment=(cell_increment + 1)%8;
            match xs[cell_increment].clone() {
                Some(mut x) => {
                    x.push(*card);
                    xs[cell_increment] = Some(x);
                },
                None => {
                    let cardcell = Vec::<Card>::new();
                    xs[cell_increment] = Some(cardcell);
                    xs[cell_increment].clone().unwrap().push(*card)
                },
            }
            // xs[cell_increment].clone().unwrap().push(*card);
        });

        // Now we have laid all cards out in random order in 8 rows
        Cells {
            freecells : [None; 4],
            targetcells: [None; 4],
            cardcells: xs,
        }
    }
    // fn cards_in_cell(cards: [Option<Card>; 20]) -> Option<[Option<Card>; 20]> {
    //     if cards.len() != 0 {
    //         Some()
    //     } else {
    //         None
    //     }
    // }

    // pub fn find_first_card(&self) {
    // }
    // // Parameter move_from describes position for Card in cell-vector
    // pub fn move_cards(&self, move_from: [Option<Card>; 20], move_to: [Option<Card>; 20]) -> () {
    // }
}

#[cfg(test)]
mod tests {
    use crate::state::cards::Deck;

    use super::*;
    #[test]
    fn deal_cards(){
        let deck= Deck::new();
        let shuffled_deck = Deck::shuffle_cards(deck);
        let cells = Cells::new(shuffled_deck);
        println!("{:?}", cells);
    }
}
