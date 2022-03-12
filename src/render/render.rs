// std imports
use rand::Rng;
use std::collections::HashMap;
// wasm-bindgen imports
use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::*;
// external crate imports
use crate::game::structs::*;
use crate::util::*;
use crate::{CANVAS, GAME, MENU};
// internal crate imports
use super::katex::*;
use super::render_constants::*;

/// main draw function, delegates to respective draw functions based on game state
#[wasm_bindgen]
pub fn draw() {
    let (game, menu) = unsafe { (GAME.as_ref().unwrap(), MENU.as_ref().unwrap()) };
    match game.state {
        GameState::PLAYAI | GameState::PLAYVS => {
            menu.close();
            render_play_screen()
        }
        GameState::MENU => {
            menu.open();
        }
        _ => {}
    }
}

/// main game render function, iterates through all game items to render
pub fn render_play_screen() {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let (player_1_colour, player_2_colour) = unsafe { (PLAYER_1_COLOUR, PLAYER_2_COLOUR) };
    let context = &canvas.context;
    let hit_context = &canvas.hit_context;

    context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);
    hit_context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);

    // draw field
    for i in 0..6 {
        if i >= 3 {
            context.set_stroke_style(&JsValue::from(player_1_colour));
        } else {
            context.set_stroke_style(&JsValue::from(player_2_colour));
        }
        render_item(format!("f={}", i));
        context.set_stroke_style(&JsValue::from("#000"));
    }
    // draw players
    for i in 1..=2 {
        if i == 1 {
            context.set_stroke_style(&JsValue::from(player_1_colour));
        } else if i == 2 {
            context.set_stroke_style(&JsValue::from(player_2_colour));
        }
        for j in 0..7 {
            render_item(format!("p{}={}", i, j));
        }
        context.set_stroke_style(&JsValue::from("#000"));
    }
    ["d=1", "x=0", "x=1", "g=0", "t=0"].iter().for_each(|id| {
        render_item(id.to_string());
    });
}

/// id-based render, dispatches to component render fns based on id
fn render_item(id: String) {
    let (key, val) = get_key_val(&id);

    match key.as_str() {
        "d" => draw_deck(id),
        "g" => draw_graveyard(id),
        "f" => draw_field(val, id),
        "p1" => draw_hand(1, val, id),
        "p2" => draw_hand(2, val, id),
        "t" => draw_turn_indicator(id),
        "x" if val == 0 => draw_cancel(id),
        "x" if val == 1 => draw_multi_done(id),
        _ => {}
    }
}

/// generates a random 6 digit Hex color code for Hit Region mapping
fn random_hit_colour(hit_region_map: &HashMap<String, String>) -> String {
    let mut hex_colour = String::new();

    while hex_colour.is_empty() || hit_region_map.contains_key(&hex_colour) {
        hex_colour = vec![0; 6]
            .iter()
            .map(|_| format!("{:X}", rand::thread_rng().gen_range(0..16)))
            .collect::<Vec<String>>()
            .join("");
    }

    format!("#{}", hex_colour)
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

/// draws the escape button for SELECT phase
fn draw_cancel(id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    if game.active.selected.is_empty() {
        return;
    }

    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;
    let player_num = game.get_current_player_num();
    let cancel_size = Vector2 {
        x: player_card_width,
        y: (player_card_height - player_card_gutter) / 2.0,
    };
    let cancel_pos = Vector2 {
        x: canvas.canvas_center.x + player_card_width * 3.5 + player_card_gutter * 4.0,
        y: if player_num == 1 {
            canvas.canvas_bounds.y - player_card_height - player_card_gutter
        } else {
            player_card_gutter
        },
    };

    draw_rect(
        cancel_pos.x,
        cancel_pos.y,
        cancel_size.x,
        cancel_size.y,
        player_card_radius / 2.0,
        id,
    );

    let context = &mut canvas.context;
    context.set_font("20px serif");
    context.set_text_baseline("middle");
    context.set_text_align("center");

    context
        .fill_text(
            "Cancel",
            cancel_pos.x + cancel_size.x / 2.0,
            cancel_pos.y + cancel_size.y / 2.0,
        )
        .expect(&format!("Cannot print cancel"));
}

/// draws the button that ends MULTI_SELECT phase
fn draw_multi_done(id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    if !matches!(game.turn.phase, TurnPhase::MULTISELECT(_)) {
        return;
    }

    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;
    let player_num = game.get_current_player_num();
    let multidone_size = Vector2 {
        x: player_card_width,
        y: (player_card_height - player_card_gutter) / 2.0,
    };
    let multidone_pos = Vector2 {
        x: canvas.canvas_center.x + player_card_width * 3.5 + player_card_gutter * 4.0,
        y: if player_num == 1 {
            canvas.canvas_bounds.y - player_card_gutter - multidone_size.y
        } else {
            player_card_gutter * 2.0 + multidone_size.y
        },
    };

    draw_rect(
        multidone_pos.x,
        multidone_pos.y,
        multidone_size.x,
        multidone_size.y,
        player_card_radius / 2.0,
        id,
    );

    let context = &mut canvas.context;
    context.set_font("20px serif");
    context.set_text_baseline("middle");
    context.set_text_align("center");

    context
        .fill_text(
            "Finish",
            multidone_pos.x + multidone_size.x / 2.0,
            multidone_pos.y + multidone_size.y / 2.0,
        )
        .expect(&format!("Cannot print multidone"));
}

/// draw marker to show whose turn it is
fn draw_turn_indicator(id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;

    let player_num = game.get_current_player_num();
    let turn_indicator_size = Vector2 {
        x: player_card_width + player_card_gutter,
        y: (player_card_height - player_card_gutter) / 2.0,
    };
    let turn_indicator_pos = Vector2 {
        x: canvas.canvas_center.x
            - player_card_width * 3.5
            - player_card_gutter * 4.0
            - turn_indicator_size.x,
        y: if player_num == 1 {
            canvas.canvas_bounds.y - player_card_height - player_card_gutter
        } else {
            player_card_gutter
        },
    };

    draw_rect(
        turn_indicator_pos.x,
        turn_indicator_pos.y,
        turn_indicator_size.x,
        turn_indicator_size.y,
        player_card_radius / 2.0,
        id,
    );

    let context = &mut canvas.context;
    context.set_font("20px serif");
    context.set_text_baseline("middle");
    context.set_text_align("center");

    context
        .fill_text(
            "Your Turn",
            turn_indicator_pos.x + turn_indicator_size.x / 2.0,
            turn_indicator_pos.y + turn_indicator_size.y / 2.0,
        )
        .expect(&format!("Cannot print cancel"));
}

/// draws deck and num cards remaining
fn draw_deck(id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_ref().unwrap()) };
    let Sizes {
        width: field_basis_width,
        height: field_basis_height,
        gutter: field_basis_gutter,
        radius: field_basis_radius,
    } = canvas.render_constants.field_sizes;

    let center = &canvas.canvas_center;
    let deck_pos = Vector2 {
        x: center.x - field_basis_width * 2.5 - field_basis_gutter * 2.0,
        y: center.y - field_basis_height / 2.0,
    };
    draw_rect(
        deck_pos.x,
        deck_pos.y,
        field_basis_width,
        field_basis_height,
        field_basis_radius,
        id,
    );

    let context = &mut canvas.context;
    context.set_font("40px KaTeX_Main");
    context.set_text_baseline("middle");
    context.set_text_align("center");
    context
        .fill_text(
            game.deck.len().to_string().as_str(),
            deck_pos.x + field_basis_width / 2.0,
            deck_pos.y + field_basis_height / 2.0,
        )
        .expect(&format!("Cannot printsize for deck"));
}

/// draws graveyard and last 3 cards played
fn draw_graveyard(_id: String) {
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
fn draw_field(val: usize, id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_mut().unwrap()) };
    let field = &game.field;
    let context = &canvas.context;
    let Sizes {
        width: field_basis_width,
        height: field_basis_height,
        gutter: field_basis_gutter,
        radius: field_basis_radius,
    } = canvas.render_constants.field_sizes;

    let card_pos = Vector2 {
        x: canvas.canvas_center.x + ((val % 3) as f64) * (field_basis_width + field_basis_gutter)
            - field_basis_width * 1.5
            - field_basis_gutter,
        y: canvas.canvas_center.y + ((val / 3) as f64) * (field_basis_height + field_basis_gutter)
            - field_basis_height
            - field_basis_gutter / 2.0,
    };

    let card = &field[val];
    if card.basis.is_none() {
        set_line_dash(context, 2, 10.0) // set line dash for empty field basis
    }
    if game.active.selected.contains(&id) {
        context.set_line_width(5.0);
    }
    draw_rect(
        card_pos.x,
        card_pos.y,
        field_basis_width,
        field_basis_height,
        field_basis_radius,
        id.clone(),
    );
    set_line_dash(context, 0, 0.0);
    context.set_line_width(2.0);

    let katex_element_id = format!("katex-item_{}", &id);
    if let Some(basis) = &card.basis {
        draw_katex(
            basis,
            katex_element_id,
            "Huge",
            Vector2 {
                y: card_pos.y + field_basis_height / 2.0,
                x: card_pos.x + field_basis_width / 2.0,
            },
        );
    } else {
        clear_katex_element(katex_element_id);
    }
}

fn get_player_card_bounds(player_num: u32, val: usize) -> (Vector2, Vector2, f64) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_ref().unwrap()) };
    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;

    let (hover_key, hover_val) =
        get_key_val(game.active.hover.as_ref().unwrap_or(&"p0=69".to_string())); // 69 shall be NULL
    let hover_player_num = hover_key.chars().nth(1).unwrap().to_digit(10).unwrap();

    let hover_card_size = Vector2 {
        x: player_card_width + player_card_gutter,
        y: player_card_height + player_card_gutter,
    };
    let card_size = if hover_player_num == player_num && hover_val == val {
        hover_card_size.clone()
    } else {
        Vector2 {
            x: player_card_width,
            y: player_card_height,
        }
    };

    let start_pos = Vector2 {
        x: canvas.canvas_center.x
            - ((player_card_gutter * 6.0 + player_card_width * 6.0) // width of 6 cards
                + if hover_player_num == player_num && hover_val != 69 { // width of potential hover card
                    hover_card_size.x
                } else {
                    player_card_width
                })
                / 2.0, // divide by 2 for distance from center
        y: if player_num == 1 {
            canvas.canvas_bounds.y - player_card_gutter - card_size.y // bottom of canvas if p1
        } else {
            player_card_gutter // top of canvas if p2
        },
    };

    let card_pos = Vector2 {
        x: start_pos.x
            + (val as f64) * (player_card_width + player_card_gutter)
            // add extra space for cards after hover
            + if hover_player_num == player_num && val > hover_val {
                hover_card_size.x - player_card_width
            } else {
                0.0
            },
        y: start_pos.y,
    };

    (card_pos, card_size, player_card_radius)
}

/// renders player hands
fn draw_hand(player_num: u32, val: usize, id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_ref().unwrap()) };
    let (card_pos, card_size, card_radius) = get_player_card_bounds(player_num, val);
    if game.active.selected.contains(&id) {
        canvas.context.set_line_width(5.0);
    }
    draw_rect(
        card_pos.x,
        card_pos.y,
        card_size.x,
        card_size.y,
        card_radius,
        id.clone(),
    );
    if game.active.selected.contains(&id) {
        canvas.context.set_line_width(2.0);
    }
}

pub fn render_player_katex() {
    for i in 1..=2 {
        for j in 0..7 {
            render_player_katex_item(i, j, format!("p{}={}", i, j));
        }
    }
}

fn render_player_katex_item(player_num: u32, val: usize, id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_ref().unwrap()) };
    let Sizes {
        gutter: player_card_gutter,
        ..
    } = canvas.render_constants.player_sizes;
    let hand = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    let (card_pos, card_size, _) = get_player_card_bounds(player_num, val);

    draw_player_card_katex(
        &hand[val],
        id,
        // middle of card
        Vector2 {
            x: card_pos.x + card_size.x / 2.0,
            y: card_pos.y + card_size.y / 2.0,
        },
        // top left of card
        Vector2 {
            x: card_pos.x + player_card_gutter * 0.75,
            y: card_pos.y + player_card_gutter * 0.75,
        },
        // bottom right of card
        Vector2 {
            x: card_pos.x + card_size.x - player_card_gutter * 0.75,
            y: card_pos.y + card_size.y - player_card_gutter * 0.75,
        },
    )
}
