// std imports
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
// wasm-bindgen imports
use gloo::render::{request_animation_frame, AnimationFrame};
// local imports
use super::pos;
use super::render;
use super::util::{RenderId, RenderItem};
// root imports
use crate::{CANVAS, GAME};
// util imports
use crate::util::min;

/// requestAnimationFrame callback
pub fn on_animation_frame(time: f64) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let anim_items = &mut canvas.anim_controller.anim_items;
    let mut finished: Vec<RenderId> = Vec::new();

    for (id, anim_item) in anim_items {
        if anim_item.start.is_none() {
            anim_item.start = Some(time);
        }

        let mut current = RenderItem::default();
        // simple lerp
        let delta = min::<f64>(
            (time - anim_item.start.unwrap()) / anim_item.duration / 1000.0,
            1.0,
        );
        for (attr, val) in anim_item.attributes.iter() {
            let (start, end) = val;
            current[*attr] = start + delta * (end - start);
        }
        if delta >= 1.0 {
            finished.push(*id);
        }

        canvas.render_items.insert(*id, current);
        render::draw();
        // render::render_player_katex();
    }
    let anim_items = &mut canvas.anim_controller.anim_items;
    for id in finished {
        let removed = anim_items.remove(&id).unwrap();
        removed.callback.iter().for_each(|f| f());
    }

    if canvas.anim_controller.anim_items.len() > 0 {
        canvas.anim_controller.render_animation_frame_handle =
            request_animation_frame(on_animation_frame);
    }
}

/// starts hover animation on player cards
pub fn animate_hover(id: Option<RenderId>) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let render_items = &canvas.render_items;

    let target_pos = if id.is_some() {
        let (key, val) = id.unwrap().key_val();
        let player_num = key.chars().nth(1).unwrap().to_digit(10).unwrap();
        pos::get_hover_player_pos(player_num, val)
    } else {
        pos::get_base_player_pos()
    };

    canvas
        .anim_controller
        .anim_items
        .extend(target_pos.iter().map(|(id, item)| {
            (
                *id,
                AnimItem {
                    start: None,
                    duration: 0.1,
                    attributes: HashMap::from([
                        (AnimAttribute::X, (render_items[id].x, item.x)),
                        (AnimAttribute::Y, (render_items[id].y, item.y)),
                        (AnimAttribute::W, (render_items[id].w, item.w)),
                        (AnimAttribute::H, (render_items[id].h, item.h)),
                        (AnimAttribute::R, (render_items[id].r, item.r)),
                    ]),
                    callback: vec![],
                },
            )
        }));

    canvas.anim_controller.start_anim();
}

pub fn animate_deal(id: RenderId) {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let render_items = &canvas.render_items;
    let deck_pos = &render_items[&RenderId::Deck];
    let target_pos = &render_items[&id];

    canvas.anim_controller.anim_items.insert(
        RenderId::Deal,
        AnimItem {
            start: None,
            duration: 0.3,
            attributes: HashMap::from([
                (AnimAttribute::X, (deck_pos.x, target_pos.x)),
                (AnimAttribute::Y, (deck_pos.y, target_pos.y)),
                (AnimAttribute::W, (deck_pos.w, target_pos.w)),
                (AnimAttribute::H, (deck_pos.h, target_pos.h)),
                (AnimAttribute::R, (deck_pos.r, target_pos.r)),
            ]),
            callback: vec![|| {
                let game = unsafe { GAME.as_mut().unwrap() };
                // inverted since the turn is already advanced
                let player = if game.get_current_player_num() == 1 {
                    &mut game.player_2
                } else {
                    &mut game.player_1
                };
                player.push(game.deck.pop().unwrap());
                render::draw();
                render::render_player_katex();
            }],
        },
    );

    canvas.anim_controller.start_anim();
}

#[derive(Debug)]
pub struct AnimController {
    pub anim_items: HashMap<RenderId, AnimItem>, // map of currently animated items
    pub anim_chain: HashMap<RenderId, (RenderId, AnimItem)>, // map of chain animation callbacks
    pub render_animation_frame_handle: AnimationFrame, // current raf handle
}

impl AnimController {
    /// starts requestAnimationFrame callback
    pub fn start_anim(&mut self) {
        self.render_animation_frame_handle = request_animation_frame(on_animation_frame);
    }
}

/// generic animation item container
#[derive(Clone, Debug)]
pub struct AnimItem {
    pub start: Option<f64>, // beginning timestamp of animation
    pub duration: f64,      // duration of animation in seconds
    pub attributes: HashMap<AnimAttribute, (f64, f64)>, // (start, end)
    pub callback: Vec<fn()>, // callback list for animation end
}

/// attributes of a render item that are able to be interpolated
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AnimAttribute {
    X, // x position of animated component
    Y, // y position of animated component
    W, // width of animated component
    H, // height of animated component
    R, // border radius of animated component
}

impl Display for AnimAttribute {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AnimAttribute::X => write!(f, "X"),
            AnimAttribute::Y => write!(f, "Y"),
            AnimAttribute::W => write!(f, "W"),
            AnimAttribute::H => write!(f, "H"),
            AnimAttribute::R => write!(f, "R"),
        }
    }
}
