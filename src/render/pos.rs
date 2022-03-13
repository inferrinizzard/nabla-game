// std imports
use std::collections::HashMap;
// external crate imports
use crate::util::*;
use crate::{CANVAS, GAME};
// internal crate imports
use super::render_constants::*;

pub fn get_base_player_pos() -> RenderHash {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let center = &canvas.canvas_center;
    let bounds = &canvas.canvas_bounds;

    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;

    let mut player_pos: RenderHash = HashMap::new();
    for player_num in 1..=2 {
        let start_pos = Vector2 {
            x: center.x - 3.5 * player_card_width - 3.0 * player_card_gutter,
            y: if player_num == 1 {
                bounds.y - player_card_height - player_card_gutter // bottom of canvas if p1
            } else {
                player_card_gutter // top of canvas if p2
            },
        };

        for i in 0..7 {
            player_pos.insert(
                RenderId::from(format!("p{player_num}={i}")),
                RenderItem {
                    x: start_pos.x + (i as f64) * (player_card_width + player_card_gutter),
                    y: start_pos.y,
                    w: player_card_width,
                    h: player_card_height,
                    r: player_card_radius,
                },
            );
        }
    }

    player_pos
}

pub fn get_hover_player_pos(player_num: u32, hover_val: usize) -> RenderHash {
    let canvas = unsafe { CANVAS.as_ref().unwrap() };

    let Sizes {
        width: player_card_width,
        height: player_card_height,
        gutter: player_card_gutter,
        radius: player_card_radius,
    } = canvas.render_constants.player_sizes;

    let start_pos = Vector2 {
        x: canvas.canvas_center.x
            - (
                (player_card_gutter * 7.0 + player_card_width * 7.0)
                // width of 6 cards
            ) / 2.0, // divide by 2 for distance from center
        y: if player_num == 1 {
            canvas.canvas_bounds.y - player_card_gutter - player_card_height // bottom of canvas if p1
        } else {
            player_card_gutter // top of canvas if p2
        },
    };

    let mut player_pos: RenderHash = HashMap::new();
    for i in 0..7 {
        let extra_size = if i == hover_val {
            player_card_gutter
        } else {
            0.0
        };
        player_pos.insert(
            RenderId::from(format!("p{player_num}={i}")),
            RenderItem {
                x: start_pos.x
                + (i as f64) * (player_card_width + player_card_gutter)
                // add extra space for cards after hover
                + if i > hover_val {
                    player_card_gutter
                } else {
                    0.0
                },
                y: start_pos.y - extra_size,
                w: player_card_width + extra_size,
                h: player_card_height + extra_size,
                r: player_card_radius,
            },
        );
    }

    player_pos
}

pub fn get_base_field_pos() -> RenderHash {
    let canvas = unsafe { CANVAS.as_mut().unwrap() };
    let center = &canvas.canvas_center;

    let Sizes {
        width: field_basis_width,
        height: field_basis_height,
        gutter: field_basis_gutter,
        radius: field_basis_radius,
    } = canvas.render_constants.field_sizes;

    let mut field_pos: RenderHash = HashMap::new();
    for i in 0..6 {
        field_pos.insert(
            RenderId::from(format!("f={i}")),
            RenderItem {
                x: center.x + ((i % 3) as f64) * (field_basis_width + field_basis_gutter)
                    - field_basis_width * 1.5
                    - field_basis_gutter,
                y: center.y + ((i / 3) as f64) * (field_basis_height + field_basis_gutter)
                    - field_basis_height
                    - field_basis_gutter / 2.0,
                w: field_basis_width,
                h: field_basis_height,
                r: field_basis_radius,
            },
        );
    }

    field_pos
}

pub fn get_base_button_pos(field_pos: &RenderHash, player_pos: &RenderHash) -> RenderHash {
    let (canvas, game) = unsafe { (CANVAS.as_ref().unwrap(), GAME.as_ref().unwrap()) };
    let center = &canvas.canvas_center;
    let player_num = game.get_current_player_num();

    let Sizes {
        width: field_basis_width,
        height: field_basis_height,
        gutter: field_basis_gutter,
        radius: field_basis_radius,
    } = canvas.render_constants.field_sizes;
    let Sizes {
        height: button_height,
        gutter: button_gutter,
        radius: button_radius,
        width: button_width,
    } = canvas.render_constants.button_sizes;

    let deck_pos = RenderItem {
        x: field_pos[&RenderId::Field0].x - field_basis_width - field_basis_gutter,
        y: center.y - field_basis_height / 2.0,
        w: field_basis_width,
        h: field_basis_height,
        r: field_basis_radius,
    };

    let cancel_pos = RenderItem {
        x: player_pos[&RenderId::PlayerOne6].x + button_width + button_gutter,
        y: player_pos[&RenderId::from(format!("p{player_num}=0"))].y,
        w: button_width,
        h: button_height,
        r: button_radius,
    };

    let multidone_pos = RenderItem {
        x: player_pos[&RenderId::PlayerOne6].x + button_width + button_gutter,
        y: player_pos[&RenderId::from(format!("p{player_num}=0"))].y
            + button_height
            + button_gutter,
        w: button_width,
        h: button_height,
        r: button_radius,
    };

    let turn_indicator_pos = RenderItem {
        x: player_pos[&RenderId::PlayerOne0].x - button_width - button_gutter,
        y: player_pos[&RenderId::from(format!("p{player_num}=0"))].y,
        w: button_width,
        h: button_height,
        r: button_radius,
    };

    HashMap::from([
        (RenderId::Deck, deck_pos),
        (RenderId::Cancel, cancel_pos),
        (RenderId::Multidone, multidone_pos),
        (RenderId::TurnIndicator, turn_indicator_pos),
    ])
}
