use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

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
        || hand_2.iter().all(|card| card.card_type() == "BASIS_CARD")
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

// #[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub turn_number: i32,          // turn counter
    pub field: [Option<Basis>; 6], // [0-2] for player_1, [3-5] for player_2
    pub player_1: Vec<Card>,       // up to 7 cards in hand (<7 if deck running low)
    pub player_2: Vec<Card>,
    pub deck: Vec<Card>,
}

#[wasm_bindgen(typescript_custom_section)]
const IGAME: &'static str = r#"
export interface Game {
    turn_number: number;
    field: number[];
    player_1: Card[];
    player_2: Card[];
    deck: Card[];
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IGame")]
    pub type IGame;
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsValue, JsValue> {
        let mut deck = get_new_deck();
        deck.shuffle(&mut thread_rng());

        let (player_1, player_2) = create_players(&mut deck);
        let game = Game {
            turn_number: 0,
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

        let js_game = serde_wasm_bindgen::to_value(&game)?;

        Ok(js_game)
    }
}

// TODO: move this to util file
// trait that defines string transforms for enums
pub trait EnumStr<T> {
    fn from_str(s: &str) -> Option<T>;
    fn to_str(&self) -> &'static str;
}
