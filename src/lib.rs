use wasm_bindgen::prelude::*;
use web_sys::console;

mod basis;
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

    let mut raw_deck = vec![];
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::Zero); 2]);
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::One); 4 - 2]); // subtract 2 for starting board
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::X); 7 - 2]); // subtract 2 for starting board
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::X2); 3 - 2]); // subtract 2 for starting board
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::Cos); 4]);
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::Sin); 4]);
    raw_deck.extend(vec![Card::BasisCard(basis::BasisCard::E); 4]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::AlgebraicCard(
            AlgebraicCard::Div
        ));
        5
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::AlgebraicCard(
            AlgebraicCard::Mult
        ));
        5
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::AlgebraicCard(
            AlgebraicCard::Sqrt
        ));
        5
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::AlgebraicCard(
            AlgebraicCard::Inverse
        ));
        5
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::AlgebraicCard(
            AlgebraicCard::Log
        ));
        5
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::DerivativeCard(
            DerivativeCard::Derivative
        ));
        8
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::DerivativeCard(
            DerivativeCard::Integral
        ));
        8
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::DerivativeCard(
            DerivativeCard::Nabla
        ));
        10
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::DerivativeCard(
            DerivativeCard::Laplacian
        ));
        2
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::LimitCard(
            LimitCard::LimPosInf
        ));
        2
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::LimitCard(
            LimitCard::LimNegInf
        ));
        2
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::LimitCard(
            LimitCard::Lim0
        ));
        2
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::LimitCard(
            LimitCard::Liminf
        ));
        1
    ]);
    raw_deck.extend(vec![
        Card::OperatorCard(OperatorCard::LimitCard(
            LimitCard::Limsup
        ));
        1
    ]);

    let game = Game {
        turn_number: 0,
        player_1: Player {
            board: [
                basis::Basis::BasisCard(basis::BasisCard::One),
                basis::Basis::BasisCard(basis::BasisCard::X),
                basis::Basis::BasisCard(basis::BasisCard::X2),
            ],
            hand: vec![],
        },
        player_2: Player {
            board: [
                basis::Basis::BasisCard(basis::BasisCard::One),
                basis::Basis::BasisCard(basis::BasisCard::X),
                basis::Basis::BasisCard(basis::BasisCard::X2),
            ],
            hand: vec![],
        },
        deck: raw_deck,
    };

    console::log_1(&JsValue::from_str(&format!("{:?}", &game)));

    Ok(())
}
