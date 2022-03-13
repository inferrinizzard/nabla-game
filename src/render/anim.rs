// std imports
use std::collections::HashMap;
// wasm-bindgen imports
use gloo::render::request_animation_frame;
// local imports
use super::pos;
use super::render;
use super::render::draw_rect;
use super::render_constants::RenderId;
// root imports
use crate::CANVAS;

use crate::util::*;

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn on_animation_frame(time: f64) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let anim_items = &mut canvas.anim_items;
    let mut finished: Vec<String> = Vec::new();

    for (id, anim_item) in anim_items {
        if anim_item.start.is_none() {
            anim_item.start = Some(time);
        }

        let mut current: HashMap<AnimAttribute, f64> = HashMap::new();

        for (attr, val) in anim_item.attributes.iter() {
            let (start, end) = val;
            let delta = min(
                (time - anim_item.start.unwrap()) / anim_item.duration / 1000.0,
                1.0,
            );
            current.insert(*attr, start + delta * (end - start));

            if delta >= 1.0 {
                finished.push(id.clone());
            }
        }

        render::draw();
        canvas.context.clear_rect(
            current[&AnimAttribute::X],
            current[&AnimAttribute::Y],
            current[&AnimAttribute::W],
            current[&AnimAttribute::H],
        );
        draw_rect(
            current[&AnimAttribute::X],
            current[&AnimAttribute::Y],
            current[&AnimAttribute::W],
            current[&AnimAttribute::H],
            current[&AnimAttribute::R],
            id.clone(),
        )
    }

    let anim_items = &mut canvas.anim_items;
    for id in finished {
        anim_items.remove(&id);
    }

    if canvas.anim_items.len() > 0 {
        canvas.render_animation_frame_handle = request_animation_frame(on_animation_frame);
    }
}

pub fn animate_hover(id: Option<RenderId>) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let sizes = &canvas.render_constants.player_sizes;
    let render_items = &canvas.render_items;

    let target_pos = if id.is_some() {
        let (key, val) = id.unwrap().key_val();
        let player_num = key.chars().nth(1).unwrap().to_digit(10).unwrap();
        pos::get_hover_player_pos(player_num, val)
    } else {
        pos::get_base_player_pos()
    };

    canvas.anim_items.extend(target_pos.iter().map(|(id, pos)| {
        (
            id.to_string(),
            AnimItem {
                start: None,
                duration: 1.0,
                attributes: HashMap::from([
                    (AnimAttribute::X, (render_items[id].x, pos.x)),
                    (AnimAttribute::Y, (render_items[id].y, pos.y)),
                    (AnimAttribute::W, (sizes.width, sizes.width)),
                    (AnimAttribute::H, (sizes.height, sizes.height)),
                    (AnimAttribute::R, (sizes.radius, sizes.radius)),
                ]),
            },
        )
    }));

    canvas.start_anim();
}

#[derive(Clone, Debug)]
pub struct AnimItem {
    pub start: Option<f64>,
    pub duration: f64,
    pub attributes: HashMap<AnimAttribute, (f64, f64)>,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AnimAttribute {
    X,
    Y,
    W,
    H,
    R,
}
