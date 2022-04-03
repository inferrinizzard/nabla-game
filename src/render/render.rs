// wasm-bindgen imports
use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::*;
// external crate imports
use crate::cards::Card;
use crate::game::structs::*;
use crate::util::*;
// root imports
use crate::{CANVAS, GAME};
// internal crate imports
use super::katex::*;
use super::sprites::CornerSpriteKey;
use super::util::*;

/// main game render function, iterates through all game items to render
#[wasm_bindgen]
pub fn draw() {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let (player_1_colour, player_2_colour) = unsafe { (PLAYER_1_COLOUR, PLAYER_2_COLOUR) };
    let context = &canvas.context;
    let hit_context = &canvas.hit_context;

    context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);
    hit_context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);

    let render_ids = &canvas.render_items;

    // draw player 1 cards
    context.set_stroke_style(&JsValue::from(player_1_colour));
    (render_ids.keys())
        .filter(|id| id.to_string().starts_with("p1") || (id.is_field() && id.key_val().1 >= 3))
        .for_each(|id| render_item(*id));

    // draw player 2 cards
    context.set_stroke_style(&JsValue::from(player_2_colour));
    (render_ids.keys())
        .filter(|id| id.to_string().starts_with("p2") || (id.is_field() && id.key_val().1 < 3))
        .for_each(|id| render_item(*id));

    // draw other items
    context.set_stroke_style(&JsValue::from("#000"));
    (render_ids.keys())
        .filter(|id| !id.is_player() && !id.is_field())
        .for_each(|id| render_item(*id));
}

/// id-based render, dispatches to component render fns based on id
fn render_item(id: RenderId) {
    match id {
        RenderId::Deck => draw_deck(id),
        RenderId::TurnIndicator => draw_button(id, "Your Turn"),
        RenderId::Cancel => draw_button(id, "Cancel"),
        RenderId::Multidone => draw_button(id, "Finish"),
        _ if id.is_graveyard() => draw_graveyard(id),
        _ if id.is_field() => draw_field(id),
        _ if id.is_player() => draw_hand(id),
        _ => {
            let canvas = unsafe { CANVAS.as_mut().unwrap() };
            // if animation item is finished, remove from render list
            if !canvas.anim_controller.anim_items.contains_key(&id) {
                canvas.render_items.remove(&id);
                return;
            }

            if id == RenderId::Deal {
                draw_deal(id);
            }
        }
    }
}

/// draws a rectangle of given size and sets hit region for id
pub fn draw_rect(x: f64, y: f64, width: f64, height: f64, radius: f64, id: String) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    // draw rounded rectangle
    context.begin_path();
    context.move_to(x + radius, y);
    context.line_to(x + width - radius, y);
    context.quadratic_curve_to(x + width, y, x + width, y + radius);
    context.line_to(x + width, y + height - radius);
    context.quadratic_curve_to(x + width, y + height, x + width - radius, y + height);
    context.line_to(x + radius, y + height);
    context.quadratic_curve_to(x, y + height, x, y + height - radius);
    context.line_to(x, y + radius);
    context.quadratic_curve_to(x, y, x + radius, y);
    context.close_path();
    context.stroke();

    // draw rect onto hit canvas with random colour
    let existing_colour = hit_region_map.iter().find(|(_, v)| **v == id);
    let hit_colour = if existing_colour.is_some() {
        existing_colour.unwrap().0.clone()
    } else {
        random_hit_colour(&hit_region_map)
    };
    hit_context.set_fill_style(&JsValue::from(&hit_colour));
    hit_context.fill_rect(x, y, width, height);
    hit_region_map.insert(hit_colour, id);
}

/// draws a button-like component
fn draw_button(id: RenderId, text: &str) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    if id == RenderId::Cancel && game.active.selected.is_empty() {
        return;
    }
    if id == RenderId::Multidone && !matches!(game.turn.phase, TurnPhase::MULTISELECT(_)) {
        return;
    }
    let button = &canvas.render_items[&id];

    let gutter = button.w / 4.0;
    let y = if game.get_current_player_num() == 1 {
        canvas.canvas_bounds.y - gutter * 2.0 - button.h * 2.0 // buttom of canvas for p1
    } else {
        gutter // top of canvas for p2
    } + if id == RenderId::Multidone {
        button.h + gutter // bottom half of card for multidone
    } else {
        0.0
    };

    draw_rect(button.x, y, button.w, button.h, button.r, id.to_string());

    let context = &mut canvas.context;
    context.set_font("20px serif");
    context.set_text_baseline("middle");
    context.set_text_align("center");

    context
        .fill_text(text, button.x + button.w / 2.0, y + button.h / 2.0)
        .expect(&format!("Cannot print {}", id));
}

/// draws deck and num cards remaining
fn draw_deck(id: RenderId) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    let deck = &canvas.render_items[&id];

    draw_rect(deck.x, deck.y, deck.w, deck.h, deck.r, id.to_string());

    let context = &mut canvas.context;
    context.set_font("40px KaTeX_Main");
    context.set_text_baseline("middle");
    context.set_text_align("center");
    context
        .fill_text(
            game.deck.len().to_string().as_str(),
            deck.x + deck.w / 2.0,
            deck.y + deck.h / 2.0,
        )
        .expect(&format!("Cannot printsize for deck"));
}

/// draws deck and num cards remaining
fn draw_deal(id: RenderId) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let deal = &canvas.render_items[&id];

    canvas.context.clear_rect(deal.x, deal.y, deal.w, deal.h);
    draw_rect(deal.x, deal.y, deal.w, deal.h, deal.r, id.to_string());
}

/// draws graveyard and last 3 cards played
fn draw_graveyard(_id: RenderId) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    let Sizes {
        width: player_card_width,
        height: player_card_height,
        radius: player_card_radius,
        ..
    } = canvas.render_constants.player_sizes;
    let Sizes {
        width: field_basis_width,
        height: field_basis_height,
        gutter: field_basis_gutter,
        ..
    } = canvas.render_constants.field_sizes;

    let center = &canvas.canvas_center;

    let card_size = Vector2 {
        x: player_card_width * 1.5,
        y: player_card_height * 1.5,
    };
    let graveyard_start = Vector2 {
        x: center.x + field_basis_width * 3.0 - field_basis_gutter * 2.0,
        y: center.y - field_basis_height + field_basis_gutter / 2.0,
    };
    let graveyard_end = Vector2 {
        x: graveyard_start.x,
        y: center.y + field_basis_gutter / 2.0 + field_basis_height - card_size.y,
    };

    let graveyard = &game.graveyard;
    for i in (0..3).rev() {
        if i + 1 > graveyard.len() {
            continue;
        }

        let id = format!("g={}", i + 1);
        let card_pos = Vector2 {
            x: graveyard_start.x + field_basis_gutter / 4.0 * i as f64,
            y: graveyard_start.y + (graveyard_end.y - graveyard_start.y) / 2.0 * i as f64,
        };
        canvas
            .context
            .clear_rect(card_pos.x, card_pos.y, card_size.x, card_size.y);
        draw_rect(
            card_pos.x,
            card_pos.y,
            card_size.x,
            card_size.y,
            player_card_radius * 1.5,
            id.clone(),
        );

        draw_katex(
            &graveyard[graveyard.len() - i - 1],
            format!("katex-item_{}", id),
            "Large",
            Vector2 {
                x: card_pos.x + card_size.x / 2.0,
                y: card_pos.y + card_size.y / 2.0,
            },
        );
    }

    let context = &mut canvas.context;
    context.set_font("20px serif");
    context.set_text_baseline("middle");
    context.set_text_align("center");
    context
        .fill_text(
            "Last 3 cards played:",
            graveyard_start.x + field_basis_width / 2.0,
            graveyard_start.y - field_basis_gutter / 2.0,
        )
        .expect(&format!("Cannot print header for graveyard"));
}

/// applies line dash style to context, or clears if dash_num is 0
fn set_line_dash(context: &CanvasRenderingContext2d, dash_num: u32, dash_size: f64) {
    let dash_array = if dash_num > 0 {
        // fill array from 0 to dash_num of dash_size
        Array::new_with_length(dash_num).fill(&JsValue::from(dash_size), 0, dash_num)
    } else {
        js_sys::Array::new()
    };

    context
        .set_line_dash(&JsValue::from(&dash_array))
        .expect("Cannot set line dash");
}

/// renders 6 field basis slots
fn draw_field(id: RenderId) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_mut().unwrap()) };
    let field = &game.field;
    let context = &canvas.context;

    let card = &canvas.render_items[&id];
    let val = id.key_val().1;

    let card_data = &field[val];
    if card_data.basis.is_none() {
        set_line_dash(context, 2, 10.0) // set line dash for empty field basis
    }
    if game.active.selected.contains(&id) {
        context.set_line_width(5.0);
    }
    draw_rect(card.x, card.y, card.w, card.h, card.r, id.to_string());
    set_line_dash(context, 0, 0.0);
    context.set_line_width(2.0);

    let katex_element_id = format!("katex-item_{}", id.to_string());
    if let Some(basis) = &card_data.basis {
        draw_katex(
            basis,
            katex_element_id,
            "Huge",
            Vector2 {
                y: card.y + card.h / 2.0,
                x: card.x + card.w / 2.0,
            },
        );
    } else {
        clear_katex_element(katex_element_id);
    }
}

/// renders player hands
fn draw_hand(id: RenderId) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_ref().unwrap()) };
    let context = &canvas.context;
    let card = &canvas.render_items[&id];

    let (key, val) = id.key_val();
    let player_num = key.chars().nth(1).unwrap().to_digit(10).unwrap();
    let hand = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    if game.active.selected.contains(&id) {
        canvas.context.set_line_width(5.0);
    }
    if val >= hand.len() {
        set_line_dash(context, 2, 10.0) // set line dash for empty field basis
    } else {
        let player_card = hand[val]; // get Card from hand

        // draw main center sprite
        let center_sprite_dims = canvas.sprite_lookup.get_card(&player_card);
        render_katex_sprite(
            center_sprite_dims,
            *card,
            |card, (dw, dh)| Vector2 {
                x: card.x + (card.w - dw) / 2.0, // centered based on sprite dimensions
                y: card.y + (card.h - dh) / 2.0, // centered based on sprite dimensions
            },
            format!("Cannot draw katex sprite for {}", id).as_str(),
        );

        // get corner icon
        let (left_corner, right_corner) = if let Card::BasisCard(_) = player_card {
            (CornerSpriteKey::ElementLeft, CornerSpriteKey::ElementRight)
        } else {
            (
                CornerSpriteKey::FunctionLeft,
                CornerSpriteKey::FunctionRight,
            )
        };

        let left_corner_dims = canvas.sprite_lookup.get_corner(left_corner);
        render_katex_sprite(
            left_corner_dims,
            *card,
            |card, _| Vector2 {
                x: card.x, // aligned to top left
                y: card.y, // aligned to top left
            },
            format!("Cannot draw left corner sprite for {}", id).as_str(),
        );
        let right_corner_dims = canvas.sprite_lookup.get_corner(right_corner);
        render_katex_sprite(
            right_corner_dims,
            *card,
            |card, (dw, dh)| Vector2 {
                x: card.x + card.w - dw, // aligned to bottom right
                y: card.y + card.h - dh, // aligned to bottom right
            },
            format!("Cannot draw right corner sprite for {}", id).as_str(),
        );
    }
    draw_rect(card.x, card.y, card.w, card.h, card.r, id.to_string());
    if game.active.selected.contains(&id) {
        canvas.context.set_line_width(2.0);
    }
    set_line_dash(context, 0, 0.0);
}
