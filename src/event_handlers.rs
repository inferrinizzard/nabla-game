use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use super::game::*;
use super::GAME;
use super::{basis::*, cards::*};

pub fn handle_mousedown(id: &String) {
    if id.is_empty() {
        return;
    }

    let game = unsafe { GAME.as_mut().unwrap() };
    let turn = &game.turn;

    match turn {
        Turn {
            number: n,
            phase: phase,
        } if n % 2 == 0 => {
            // even-number turn, player 1

            match phase {
                TurnPhase::IDLE => {
                    if id.starts_with("p1") {
                        let card_id = id.split("=").collect::<Vec<&str>>()[1];
                        let card = game.player_1[card_id.parse::<usize>().unwrap()];
                        match card {
                            Card::BasisCard(_) => {
                                // allow play if empty slot
                                if game.field.iter().any(|b| b.is_none()) {
                                    next_phase(TurnPhase::SELECT);
                                }
                            }
                            Card::DerivativeCard(DerivativeCard::Laplacian)
                            | Card::DerivativeCard(DerivativeCard::Nabla) => {
                                // field select
                                next_phase(TurnPhase::FIELD_SELECT);
                            }
                            Card::AlgebraicCard(AlgebraicCard::Div)
                            | Card::AlgebraicCard(AlgebraicCard::Mult) => {
                                // multiselect
                                next_phase(TurnPhase::MULTISELECT);
                            }
                            _ => {
                                // select
                                next_phase(TurnPhase::SELECT);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Turn {
            number: n,
            phase: p,
        } if n % 2 == 1 => {
            // odd-number turn, player 2
        }
        _ => console::log_1(&JsValue::from(id)),
    }
}

pub fn next_phase(phase: TurnPhase) {
    let game = unsafe { GAME.as_mut().unwrap() };
    game.turn = Turn {
        number: game.turn.number,
        phase: phase,
    };
}

pub fn next_turn() {
    let game = unsafe { GAME.as_mut().unwrap() };
    game.turn = Turn {
        number: game.turn.number + 1,
        phase: TurnPhase::IDLE,
    };
}
