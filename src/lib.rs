use wasm_bindgen::prelude::*;
use web_sys::console;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod basis;
use basis::*;
mod cards;
use cards::*;
mod structs;
use structs::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

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

    deck.shuffle(&mut thread_rng());

    let game = Game {
        // move setup items to constructor
        turn_number: 0,
        player_1: Player {
            board: [
                Basis::BasisCard(BasisCard::One),
                Basis::BasisCard(BasisCard::X),
                Basis::BasisCard(BasisCard::X2),
            ],
            // need to perform validation of hand to ensure player does not receive 7 BasisCards, if so - mulligan and re draw
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

    console::log_1(&JsValue::from_str(&format!("{:?}", &game)));

    Ok(())
}
