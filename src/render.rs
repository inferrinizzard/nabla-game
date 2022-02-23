use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::*;

use rand::Rng;
use std::collections::HashMap;

use super::util::*;
use super::{CANVAS, GAME};
use crate::katex::*;

pub fn draw() {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let context = &canvas.context;

    context.clear_rect(0.0, 0.0, canvas.canvas_bounds.x, canvas.canvas_bounds.y);

    draw_field();
    draw_hand(1);
    draw_hand(2);
    draw_x();
    draw_multi_done();
    // build list of eleemnts to draw here
}

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

fn draw_x() {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };
    if game.active.selected.is_empty() {
        return;
    }

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    context.stroke_rect(10.0, 10.0, 25.0, 25.0);

    // draw rect onto hit canvas with random colour
    let hit_colour = random_hit_colour(&hit_region_map);
    hit_context.set_fill_style(&JsValue::from(&hit_colour));
    hit_context.fill_rect(10.0, 10.0, 25.0, 25.0);
    hit_region_map.insert(hit_colour, "x=0".to_string());
}

fn draw_multi_done() {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };

    let context = &canvas.context;
    let bounds = &canvas.canvas_bounds;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    context.stroke_rect(bounds.x - 35.0, 10.0, 25.0, 25.0);

    // draw rect onto hit canvas with random colour
    let hit_colour = random_hit_colour(&hit_region_map);
    hit_context.set_fill_style(&JsValue::from(&hit_colour));
    hit_context.fill_rect(bounds.x - 35.0, 10.0, 25.0, 25.0);
    hit_region_map.insert(hit_colour, "x=1".to_string());
}

fn draw_field() {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };
    let field = &game.field;

    let rect_height = 200.0;
    let rect_width = 150.0;
    let gutter = 50.0;

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    for (i, card) in field.iter().enumerate() {
        let id = format!("f={}", i);

        if card.basis.is_none() {
            context
                .set_line_dash(&JsValue::from(&Array::fill(
                    &Array::new_with_length(2),
                    &JsValue::from(10i32),
                    0,
                    2,
                )))
                .expect(format!("Cannot set line dash for {:?}", card).as_str());
        } else {
            context
                .set_line_dash(&JsValue::from(&js_sys::Array::new()))
                .expect(format!("Cannot set line dash for {:?}", card).as_str());
        }
        let card_pos = Vector2 {
            x: canvas.canvas_center.x + ((i % 3) as f64) * (rect_width + gutter)
                - rect_width * 1.5
                - gutter,
            y: canvas.canvas_center.y + ((i / 3) as f64) * (rect_height + gutter)
                - rect_height
                - gutter / 2.0,
        };
        context.stroke_rect(card_pos.x, card_pos.y, rect_width, rect_height);
        // draw rect onto hit canvas with random colour
        let hit_colour = random_hit_colour(&hit_region_map);
        hit_context.set_fill_style(&JsValue::from(&hit_colour));
        hit_context.fill_rect(card_pos.x, card_pos.y, rect_width, rect_height);
        hit_region_map.insert(hit_colour, id.clone());

        if let Some(basis) = &card.basis {
            let katex_element_id = format!("katex-item_{}", &id);
            let element = render_katex_element(basis.clone(), katex_element_id, "Huge");

            let style_string = format!(
                "position: absolute; top: {}px; left: {}px;",
                card_pos.y + rect_width / 2.0,
                card_pos.x + rect_width / 2.0
            );
            element
                .set_attribute("style", style_string.as_str())
                .expect(format!("Cannot set style for {:?}", card).as_str());
        }
    }
}

fn draw_hand(player_num: u32) {
    let (canvas, game) = unsafe { (CANVAS.as_mut().unwrap(), GAME.as_mut().unwrap()) };
    let hand = if player_num == 1 {
        &game.player_1
    } else {
        &game.player_2
    };

    let rect_height = 100.0;
    let rect_width = 75.0;
    let gutter = 25.0;

    let context = &canvas.context;
    let hit_context = &canvas.hit_context;
    let hit_region_map = &mut canvas.hit_region_map;

    for (i, card) in hand.iter().enumerate() {
        let id = format!("p{}={}", player_num, i);

        let y_pos = if player_num == 1 {
            canvas.canvas_bounds.y - gutter - rect_height
        } else {
            gutter
        };

        let card_pos = Vector2 {
            x: canvas.canvas_center.x - (rect_width * 3.5) - gutter * 3.0
                + (i as f64) * (rect_width + gutter),
            y: y_pos,
        };
        context.stroke_rect(card_pos.x, card_pos.y, rect_width, rect_height);
        // draw rect onto hit canvas with random colour
        let hit_colour = random_hit_colour(&hit_region_map);
        hit_context.set_fill_style(&JsValue::from(&hit_colour));
        hit_context.fill_rect(card_pos.x, card_pos.y, rect_width, rect_height);
        hit_region_map.insert(hit_colour, id.clone());

        let katex_element_id = format!("katex-item_{}", &id);
        let element = render_katex_element(card.clone(), katex_element_id, "Large");

        let style_string = format!(
            "position: absolute; top: {}px; left: {}px;",
            card_pos.y + rect_width / 2.0,
            card_pos.x + rect_width / 2.0
        );
        element
            .set_attribute("style", style_string.as_str())
            .expect(format!("Cannot set style for {:?}", card).as_str());
    }
}
