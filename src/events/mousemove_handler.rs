// outer crate imports
use crate::render::anim;
use crate::render::render_constants::RenderId;
// use crate::render::render;
// root imports
use crate::{CANVAS, GAME};

/// delegates event handling based on turn num
pub fn handle_mousemove(id: String) {
    let game = unsafe { GAME.as_mut().unwrap() };

    let prev = game.active.hover.clone();
    // only allow player cards on hover
    game.active.hover = if id.chars().nth(0).unwrap_or('_') == 'p' {
        Some(id.clone())
    } else {
        None
    };
    if prev != game.active.hover {
        // render::draw();
        // render::render_player_katex()

        // anim::animate_hover(game.active.hover);
        anim::animate_hover(if matches!(game.active.hover, Some(_)) {
            Some(RenderId::from(id))
        } else {
            None
        });
    }
}
