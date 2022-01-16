// use wasm_bindgen::prelude::*;

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

fn create_players(deck: &mut Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    // deal initial hands
    let mut hand_1 = deck.split_off(deck.len() - 7);
    let mut hand_2 = deck.split_off(deck.len() - 7);

    // check for edge case where a player receives all basis cards
    while hand_1.iter().all(|card| card.card_type() == "BASIS_CARD")
        && hand_2.iter().all(|card| card.card_type() == "BASIS_CARD")
    {
        // mulligan and re shuffle
        deck.append(&mut hand_1);
        deck.append(&mut hand_2);
        deck.shuffle(&mut thread_rng());
        hand_1 = deck.split_off(deck.len() - 7);
        hand_2 = deck.split_off(deck.len() - 7);
    }

    (hand_1, hand_2)
}

#[derive(Debug)]
pub struct Game {
    pub turn: Turn,                // turn counter
    pub field: [Option<Basis>; 6], // [0-2] for player_1, [3-5] for player_2
    pub player_1: Vec<Card>,       // up to 7 cards in hand (<7 if deck running low)
    pub player_2: Vec<Card>,
    pub deck: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = get_new_deck();
        deck.shuffle(&mut thread_rng());

        let (player_1, player_2) = create_players(&mut deck);
        return Game {
            turn: Turn {
                number: 0,
                phase: TurnPhase::IDLE,
            },
            field: [
                Some(Basis::BasisCard(BasisCard::One)),
                Some(Basis::BasisCard(BasisCard::X)),
                Some(Basis::BasisCard(BasisCard::X2)),
                Some(Basis::BasisCard(BasisCard::One)),
                Some(Basis::BasisCard(BasisCard::X)),
                Some(Basis::BasisCard(BasisCard::X2)),
            ],
            player_1: player_1,
            player_2: player_2,
            deck: deck,
        };
    }
}

#[derive(Debug)]
pub struct Turn {
    pub number: u32,
    pub phase: TurnPhase,
}

// IDLE → SELECTED →
#[derive(Debug)]
pub enum TurnPhase {
    IDLE,
    SELECT,       // single-basis operators or playing new operators with a blank slot
    FIELD_SELECT, // nabla or laplacian
    MULTISELECT,  // mult or div
}
