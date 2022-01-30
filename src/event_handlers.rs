use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use super::game::*;
use super::render;
use super::GAME;
use super::{basis::*, cards::*};

pub fn handle_mousedown(id: &String) {
    if id.is_empty() {
        return;
    }

    let game = unsafe { GAME.as_mut().unwrap() };
    let turn = &game.turn;

    match turn {
        Turn { number: n, .. } if n % 2 == 0 => {
            // even-number turn, player 1
            branch_turn_phase(id, 1);
        }
        Turn { number: n, .. } if n % 2 == 1 => {
            // odd-number turn, player 2
            branch_turn_phase(id, 2);
        }
        _ => console::log_1(&JsValue::from(id)),
    }
}

pub fn split_id(id: &String) -> (&str, usize) {
    let kvp = id.split("=").collect::<Vec<&str>>();
    (kvp[0], kvp[1].parse::<usize>().unwrap())
}

pub fn branch_turn_phase(id: &String, player_num: u32) {
    let game = unsafe { GAME.as_mut().unwrap() };
    let turn = &game.turn;
    let player = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    let (id_key, id_val) = split_id(id);

    if id_key == "x" {
        next_phase(TurnPhase::IDLE);
        return;
    }

    match turn.phase {
        TurnPhase::IDLE if id_key == format!("p{}", player_num) => {
            // match against current card in player hand
            match player[id_val] {
                Card::BasisCard(basis_card) => {
                    // allow play if empty slot
                    if game.field.iter().any(|b| b.basis.is_none()) {
                        next_phase(TurnPhase::SELECT(Card::BasisCard(basis_card)));
                    }
                }
                Card::DerivativeCard(derivative_card)
                    if matches!(
                        derivative_card,
                        DerivativeCard::Laplacian | DerivativeCard::Nabla
                    ) =>
                {
                    // field select
                    next_phase(TurnPhase::FIELD_SELECT(Card::DerivativeCard(
                        derivative_card,
                    )));
                }
                Card::AlgebraicCard(algebraic_card)
                    if matches!(algebraic_card, AlgebraicCard::Div | AlgebraicCard::Mult) =>
                {
                    // multiselect
                    next_phase(TurnPhase::MULTISELECT(Card::AlgebraicCard(algebraic_card)));
                }
                card => {
                    // select
                    next_phase(TurnPhase::SELECT(card));
                }
            }
        }
        TurnPhase::SELECT(select_operator) => match select_operator {
            Card::BasisCard(basis_card) => {
                if id_key == "f" && game.field[id_val].basis.is_none() {
                    game.field[id_val] = FieldBasis::new(&Basis::BasisCard(basis_card));
                }
            }
            operator_card => {
                if id_key == "f" {
                    let selected_field_basis = &mut game.field[id_val];
                    if selected_field_basis.has_value(&operator_card) {
                        if matches!(
                            operator_card,
                            Card::DerivativeCard(DerivativeCard::Derivative)
                        ) {
                            selected_field_basis.derivative(None); // use saved derivative
                        } else if matches!(
                            operator_card,
                            Card::DerivativeCard(DerivativeCard::Integral)
                        ) {
                            selected_field_basis.integral(None); // use saved integral
                        }
                        return;
                    }

                    let result_basis =
                        apply_card(&operator_card)(selected_field_basis.basis.as_ref().unwrap());
                    if matches!(result_basis, Basis::BasisCard(BasisCard::Zero)) {
                        game.field[id_val] = FieldBasis::none();
                    } else {
                        if matches!(
                            operator_card,
                            Card::DerivativeCard(DerivativeCard::Derivative)
                        ) {
                            selected_field_basis.derivative(Some(&result_basis));
                        } else if matches!(
                            operator_card,
                            Card::DerivativeCard(DerivativeCard::Integral)
                        ) {
                            selected_field_basis.integral(Some(&result_basis));
                        }
                        game.field[id_val] = FieldBasis::new(&result_basis);
                    }

                    // player.remove()
                    next_turn();
                }
            }
        },
        TurnPhase::FIELD_SELECT(field_operator) => {
            if id_key == "f" {
                let is_laplacian = matches!(
                    field_operator,
                    Card::DerivativeCard(DerivativeCard::Laplacian)
                );

                let card_range;
                if id_val > 3 {
                    card_range = 0..3;
                } else {
                    card_range = 3..6;
                }

                // for each basis on the field
                for i in card_range {
                    let selected_field_basis = &mut game.field[i];
                    if selected_field_basis.has_value(&field_operator) {
                        selected_field_basis.derivative(None);
                        if is_laplacian {
                            selected_field_basis.derivative(None);
                        }
                    } else {
                        let first_derivative = apply_card(&field_operator)(
                            selected_field_basis.basis.as_ref().unwrap(),
                        );
                        if matches!(first_derivative, Basis::BasisCard(BasisCard::Zero)) {
                            game.field[i] = FieldBasis::none();
                            continue;
                        }
                        selected_field_basis.derivative(Some(&first_derivative));
                        if is_laplacian {
                            let second_derivative = apply_card(&field_operator)(&first_derivative);
                            if matches!(second_derivative, Basis::BasisCard(BasisCard::Zero)) {
                                game.field[i] = FieldBasis::none();
                                continue;
                            }
                            selected_field_basis.derivative(Some(&second_derivative));
                        }
                    }
                }

                // resolve hand
                next_turn();
            }
        }
        TurnPhase::MULTISELECT(multi_operator) => {
            if id_key == "f" || id_key == format!("p{}", player_num) {
                // add to queue of operands to be mult / div (need special menu here)
            }
        }
        _ => console::log_1(&JsValue::from(format!(
            "unknown case, received id:{} on turn:{:?}",
            id, turn
        ))),
    }
}

pub fn next_phase(phase: TurnPhase) {
    let game = unsafe { GAME.as_mut().unwrap() };
    console::log_1(&JsValue::from(format!("entering phase: {:?}", phase)));
    game.turn = Turn {
        number: game.turn.number,
        phase: phase,
    };
}

pub fn next_turn() {
    let game = unsafe { GAME.as_mut().unwrap() };
    // console::log_1(&JsValue::from(format!("{:?}", game.field)));
    // console::log_1(&JsValue::from(format!("{:?}", game.player_1)));
    // console::log_1(&JsValue::from(format!("{:?}", game.player_2)));

    console::log_1(&JsValue::from(format!(
        "entering turn: {}",
        game.turn.number + 1
    )));
    game.turn = Turn {
        number: game.turn.number + 1,
        phase: TurnPhase::IDLE,
    };
    render::draw();
}
