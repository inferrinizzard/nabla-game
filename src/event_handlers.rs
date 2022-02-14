use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

use super::game::*;
use super::render;
use super::GAME;
use super::{basis::*, cards::*};

pub fn handle_mousedown(id: String) {
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

fn get_key_val(id: &String) -> (String, usize) {
    let kvp = id.split("=").collect::<Vec<&str>>();
    (kvp[0].to_string(), kvp[1].parse::<usize>().unwrap())
}

pub fn branch_turn_phase(id: String, player_num: u32) {
    let game = unsafe { GAME.as_mut().unwrap() };
    let turn = &game.turn;
    let player = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    let (id_key, id_val) = get_key_val(&id);

    if id_key == "x" && id_val == 0 {
        game.active.clear();
        next_phase(TurnPhase::IDLE);
        return;
    }

    match turn.phase {
        TurnPhase::IDLE if id_key == format!("p{}", player_num) => {
            game.active.selected.push(id.to_string());
            idle_turn_phase(player[id_val]);
        }
        TurnPhase::SELECT(select_operator) => select_turn_phase(select_operator, (id_key, id_val)),
        TurnPhase::FIELD_SELECT(field_operator) if id_key == "f" => {
            field_select_phase(field_operator, (id_key, id_val))
        }
        TurnPhase::MULTISELECT(multi_operator) => {
            multi_select_phase(multi_operator, id, player_num)
        }
        _ => console::log_1(&JsValue::from(format!(
            "unknown case, received id:{} on turn:{:?}",
            id, turn
        ))),
    }
}

fn idle_turn_phase(card: Card) {
    let game = unsafe { GAME.as_mut().unwrap() };

    // match against current card in player hand
    match card {
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

fn select_turn_phase(select_operator: Card, (id_key, id_val): (String, usize)) {
    let game = unsafe { GAME.as_mut().unwrap() };

    match select_operator {
        Card::BasisCard(basis_card) => {
            if id_key == "f"
                && game.field[id_val].basis.is_none()
                && !matches!(basis_card, BasisCard::Zero)
            {
                game.field[id_val] = FieldBasis::new(&Basis::from_card(basis_card));
                end_turn();
            }
        }
        operator_card => {
            if id_key == "f" {
                if matches!(
                    operator_card,
                    Card::DerivativeCard(DerivativeCard::Derivative | DerivativeCard::Integral)
                ) {
                    handle_derivative_card(operator_card, id_val);
                } else {
                    let selected_field_basis = &mut game.field[id_val];
                    let result_basis =
                        apply_card(&operator_card)(selected_field_basis.basis.as_ref().unwrap());
                    if result_basis.is_num(0) {
                        game.field[id_val] = FieldBasis::none();
                    } else {
                        game.field[id_val] = FieldBasis::new(&result_basis);
                    }
                }
                end_turn();
            }
        }
    }
}

fn field_select_phase(field_operator: Card, (id_key, id_val): (String, usize)) {
    let card_range = if id_val < 3 { 0..3 } else { 3..6 };
    // for each basis on one half of the field
    for i in card_range {
        handle_derivative_card(field_operator, i);
    }
    end_turn();
}

fn handle_derivative_card(card: Card, i: usize) {
    let game = unsafe { GAME.as_mut().unwrap() };

    let is_laplacian = matches!(card, Card::DerivativeCard(DerivativeCard::Laplacian));
    let is_integral = matches!(card, Card::DerivativeCard(DerivativeCard::Integral));
    let is_derivative = matches!(
        card,
        Card::DerivativeCard(DerivativeCard::Derivative | DerivativeCard::Nabla)
    );

    let selected_field_basis = &mut game.field[i];
    if selected_field_basis.basis.is_none() {
        return;
    }

    // console::log_1(&JsValue::from(&format!("Derivative of: {:?}", selected_field_basis.basis)));
    // shortcut if already in history
    if selected_field_basis.has_value(&card) {
        if is_derivative || is_laplacian {
            selected_field_basis.derivative(None);
        } else if is_integral {
            selected_field_basis.integral(None);
        }
        if is_laplacian {
            selected_field_basis.derivative(None);
        }
    } else {
        let result_basis = apply_card(&card)(selected_field_basis.basis.as_ref().unwrap());
        if result_basis.is_num(0) {
            game.field[i] = FieldBasis::none();
            return;
        } else {
            if is_derivative || is_laplacian {
                selected_field_basis.derivative(Some(&result_basis));
            } else if is_integral {
                selected_field_basis.integral(Some(&result_basis));
            }
        }
        if is_laplacian {
            let second_derivative = apply_card(&card)(&result_basis);
            if second_derivative.is_num(0) {
                game.field[i] = FieldBasis::none();
                return;
            }
            selected_field_basis.derivative(Some(&second_derivative));
        }
    }
}

fn multi_select_phase(multi_operator: Card, id: String, player_num: u32) {
    let game = unsafe { GAME.as_mut().unwrap() };
    let player = if &game.turn.number % 2 == 0 {
        &mut game.player_1
    } else {
        &mut game.player_2
    };
    let field = &mut game.field;
    let (id_key, id_val) = get_key_val(&id);
    if id_key == "f"
        || (id_key == format!("p{}", player_num) && matches!(player[id_val], Card::BasisCard(_)))
    {
        game.active.selected.push(id.to_string());
        console::log_1(&JsValue::from(format!("added to multiselect: {}", id)));
    }

    if id_key == "x"
        && id_val == 1
        && game // must have at least 1 field basis
            .active
            .selected
            .iter()
            .find(|sel_id| sel_id.as_str().starts_with("f"))
            .is_some()
    {
        let result_basis = apply_multi_card(
            &multi_operator,
            game.active
                .selected
                .iter()
                .filter_map(|sel_id| {
                    let (sel_key, sel_val) = get_key_val(&sel_id);

                    if sel_key == "f" {
                        return Some(field[sel_val].basis.as_ref().unwrap().clone());
                    } else if sel_key == format!("p{}", player_num) {
                        if let Card::AlgebraicCard(_operator) = player[sel_val] {
                            // skip the mult_operator
                            return None;
                        } else if let Card::BasisCard(basis_card) = player[sel_val] {
                            return Some(Basis::from_card(basis_card));
                        }
                    }
                    panic!("invalid card selected! {}", sel_id);
                })
                .collect::<Vec<Basis>>(),
        );
        let used_field_bases = game
            .active
            .selected
            .iter()
            .filter_map(|sel_id| {
                let (sel_key, sel_val) = get_key_val(&sel_id);
                if sel_key == "f" {
                    return Some(sel_val);
                }
                None
            })
            .collect::<Vec<usize>>();
        used_field_bases // clear used field bases
            .iter()
            .for_each(|field_index| field[*field_index] = FieldBasis::none());
        field[used_field_bases[0]] = FieldBasis::new(&result_basis); // assign result basis to any newly empty field
        end_turn();
    }
}

fn end_turn() {
    let game = unsafe { GAME.as_mut().unwrap() };
    // get vector indices of cards used by player this turn
    let mut selected_indices = game
        .active
        .selected
        .iter()
        .filter(|card| card.get(0..1).unwrap() == "p")
        .map(|card| {
            card.split("=").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<usize>>();
    selected_indices.sort();
    selected_indices.reverse();

    let player = if &game.turn.number % 2 == 0 {
        &mut game.player_1
    } else {
        &mut game.player_2
    };
    // remove used cards
    for i in selected_indices.iter() {
        player.remove(*i);
    }

    let deck = &mut game.deck;
    // replenish from deck if possible
    for _ in player.len()..7 {
        if deck.len() > 0 {
            player.push(deck.pop().unwrap());
        }
    }
    next_turn();
}

fn next_phase(phase: TurnPhase) {
    let game = unsafe { GAME.as_mut().unwrap() };
    console::log_1(&JsValue::from(format!("entering phase: {:?}", phase)));
    game.turn = Turn {
        number: game.turn.number,
        phase: phase,
    };
    render::draw();
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
    game.active.clear();
    render::draw();
}
