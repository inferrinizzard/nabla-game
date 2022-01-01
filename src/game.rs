use wasm_bindgen::prelude::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::basis::*;
use super::cards::*;

fn get_new_deck() -> Vec<Card> {
    let mut deck = vec![];
    deck.extend(vec![Card::BasisCard(BasisCard::Zero); 2]);
    deck.extend(vec![Card::BasisCard(BasisCard::One); 4 - 2]); // subtract 2 for starting board
    deck.extend(vec![Card::BasisCard(BasisCard::X); 7 - 2]); // subtract 2 for starting board
    deck.extend(vec![Card::BasisCard(BasisCard::X2); 3 - 2]); // subtract 2 for starting board
    deck.extend(vec![Card::BasisCard(BasisCard::Cos); 4]);
    deck.extend(vec![Card::BasisCard(BasisCard::Sin); 4]);
    deck.extend(vec![Card::BasisCard(BasisCard::E); 4]);
    deck.extend(vec![Card::AlgebraicCard(AlgebraicCard::Div); 5]);
    deck.extend(vec![Card::AlgebraicCard(AlgebraicCard::Mult); 5]);
    deck.extend(vec![Card::AlgebraicCard(AlgebraicCard::Sqrt); 5]);
    deck.extend(vec![Card::AlgebraicCard(AlgebraicCard::Inverse); 5]);
    deck.extend(vec![Card::AlgebraicCard(AlgebraicCard::Log); 5]);
    deck.extend(vec![Card::DerivativeCard(DerivativeCard::Derivative); 8]);
    deck.extend(vec![Card::DerivativeCard(DerivativeCard::Integral); 8]);
    deck.extend(vec![Card::DerivativeCard(DerivativeCard::Nabla); 10]);
    deck.extend(vec![Card::DerivativeCard(DerivativeCard::Laplacian); 2]);
    deck.extend(vec![Card::LimitCard(LimitCard::LimPosInf); 2]);
    deck.extend(vec![Card::LimitCard(LimitCard::LimNegInf); 2]);
    deck.extend(vec![Card::LimitCard(LimitCard::Lim0); 2]);
    deck.extend(vec![Card::LimitCard(LimitCard::Liminf); 1]);
    deck.extend(vec![Card::LimitCard(LimitCard::Limsup); 1]);

    return deck;
}

pub fn build_game() -> Game {
    let mut deck = get_new_deck();
    deck.shuffle(&mut thread_rng());

    // deal hands
    // while hand is not valid, shuffle and redeal hands
    // need to perform validation of hand to ensure player does not receive 7 BasisCards, if so - mulligan and re draw
    // hand = deck.split_off(deck.len() - 7)
    // if hand.all(|card| card is BasisCard) = invalid & redraw
    //     deck.chain(hand) // add hand(s) back to deck and reshuffle

    let game = Game {
        turn_number: 0,
        player_1: Player {
            board: [
                Basis::BasisCard(BasisCard::One),
                Basis::BasisCard(BasisCard::X),
                Basis::BasisCard(BasisCard::X2),
            ],
            hand: deck.split_off(deck.len() - 7), // deal last 7 cards of deck
        },
        player_2: Player {
            board: [
                Basis::BasisCard(BasisCard::One),
                Basis::BasisCard(BasisCard::X),
                Basis::BasisCard(BasisCard::X2),
            ],
            hand: deck.split_off(deck.len() - 7), // deal last 7 cards of deck
        },
        deck: deck,
    };

    return game;
}

// #[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
    pub turn_number: i32, // turn counter
    pub player_1: Player,
    pub player_2: Player,
    pub deck: Vec<Card>,
}

#[derive(Debug)]
pub struct Player {
    pub board: [Basis; 3], // 3 cards on the field, may want to move up to Game level
    pub hand: Vec<Card>,   // up to 7 cards in hand (<7 if deck running low)
}

// TODO: move this to util file
// trait that defines string transforms for enums
pub trait EnumStr<T> {
    fn from_str(s: &str) -> Option<T>;
    fn to_str(&self) -> &'static str;
}
