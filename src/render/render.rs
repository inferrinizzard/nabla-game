use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::*;

use rand::Rng;
use std::collections::HashMap;

use super::katex::*;
use super::render_constants::*;
use crate::util::*;
use crate::{CANVAS, GAME};

/// main render function, iterates through all items to render
pub fn draw() {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let context = &canvas.context;

    context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);

    // draw field
    for i in 0..6 {
        render_item(format!("f={}", i));
    }
    for i in 1..=2 {
        for j in 0..7 {
            render_item(format!("p{}={}", i, j));
        }
    }
    render_item("x=0".to_string());
    render_item("x=1".to_string());
}

/// id-based render, dispatches to component render fns based on id
fn render_item(id: String) {
    let kvp = id.split("=").collect::<Vec<&str>>();
    let key = kvp[0];
    let val = kvp[1].parse::<usize>().unwrap();

    match key {
        "f" => draw_field(val, id),
        "p1" => draw_hand(1, val, id),
        "p2" => draw_hand(2, val, id),
        "x" => {
            if val == 0 {
                draw_x();
            } else if val == 1 {
                draw_multi_done();
            }
        }
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

// draws a rectangle of given size and sets hit region for id
fn draw_rect(x: f64, y: f64, width: f64, height: f64, id: String) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    context.stroke_rect(x, y, width, height);

    // draw rect onto hit canvas with random colour
    let hit_colour = random_hit_colour(&hit_region_map);
    hit_context.set_fill_style(&JsValue::from(&hit_colour));
    hit_context.fill_rect(x, y, width, height);
    hit_region_map.insert(hit_colour, id);
}

/// draws the escape button for SELECT phase
fn draw_x() {
    let game = unsafe { GAME.as_mut().unwrap() };
    if game.active.selected.is_empty() {
        return;
    }

    draw_rect(10.0, 10.0, 25.0, 25.0, "x=0".to_string());
}

/// draws the button that ends MULTI_SELECT phase
fn draw_multi_done() {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };

    let bounds = &canvas.canvas_bounds;
    draw_rect(bounds.x - 35.0, 10.0, 25.0, 25.0, "x=1".to_string());
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

/// renders KaTeX item at pos with given size & id
fn draw_katex<T>(item: &T, id: String, size: &str, pos: Vector2)
where
    T: ToLatex,
    T: Clone,
    T: std::fmt::Debug,
{
    let element = render_katex_element(item.clone(), id, size);
    let style_string = format!("position: absolute; top: {}px; left: {}px;", pos.y, pos.x);
    element
        .set_attribute("style", style_string.as_str())
        .expect(format!("Cannot set style for {:?}", item).as_str());
}

/// renders 6 field basis slots
fn draw_field(val: usize, id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };
    let field = &game.field;
    let context = &canvas.context;

    let card_pos = Vector2 {
        x: canvas.canvas_center.x + ((val % 3) as f64) * (FIELD_BASIS_WIDTH + FIELD_BASIS_GUTTER)
            - FIELD_BASIS_WIDTH * 1.5
            - FIELD_BASIS_GUTTER,
        y: canvas.canvas_center.y + ((val / 3) as f64) * (FIELD_BASIS_HEIGHT + FIELD_BASIS_GUTTER)
            - FIELD_BASIS_HEIGHT
            - FIELD_BASIS_GUTTER / 2.0,
    };

    let card = &field[val];
    if card.basis.is_none() {
        set_line_dash(context, 2, 10.0) // set line dash for empty field basis
    }
    draw_rect(
        card_pos.x,
        card_pos.y,
        FIELD_BASIS_WIDTH,
        FIELD_BASIS_HEIGHT,
        id.clone(),
    );
    set_line_dash(context, 0, 0.0);

    let katex_element_id = format!("katex-item_{}", &id);
    if let Some(basis) = &card.basis {
        draw_katex(
            basis,
            katex_element_id,
            "Huge",
            Vector2 {
                y: card_pos.y + FIELD_BASIS_HEIGHT / 2.0,
                x: card_pos.x + FIELD_BASIS_WIDTH / 2.0,
            },
        );
    } else {
        clear_katex_element(katex_element_id);
    }
}

/// renders player hands
fn draw_hand(player_num: u32, val: usize, id: String) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };
    let hand = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    let y_pos = if player_num == 1 {
        canvas.canvas_bounds.y - PLAYER_CARD_GUTTER - PLAYER_CARD_HEIGHT
    } else {
        PLAYER_CARD_GUTTER
    };

    let card = &hand[val];
    let card_pos = Vector2 {
        x: canvas.canvas_center.x - (PLAYER_CARD_WIDTH * 3.5) - PLAYER_CARD_GUTTER * 3.0
            + (val as f64) * (PLAYER_CARD_WIDTH + PLAYER_CARD_GUTTER),
        y: y_pos,
    };

    draw_rect(
        card_pos.x,
        card_pos.y,
        PLAYER_CARD_WIDTH,
        PLAYER_CARD_HEIGHT,
        id.clone(),
    );

    let katex_element_id = format!("katex-item_{}", &id);
    draw_katex(
        card,
        katex_element_id,
        "Large",
        Vector2 {
            y: card_pos.y + PLAYER_CARD_HEIGHT / 2.0,
            x: card_pos.x + PLAYER_CARD_WIDTH / 2.0,
        },
    );
}
