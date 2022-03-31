// outer crate imports
use crate::render::anim;
use crate::render::util::RenderId;
// root imports
use crate::GAME;

/// delegates event handling based on turn num
pub fn handle_mousemove(str_id: String) {
    let game = unsafe { GAME.as_mut().unwrap() };

    let prev = game.active.hover.clone();
    if str_id.is_empty() {
        game.active.hover = None;
    } else {
        let id = RenderId::from(str_id);
        // only allow player cards on hover
        game.active.hover = if id.is_player() { Some(id) } else { None };
    }

    if prev != game.active.hover {
        anim::animate_hover(game.active.hover);
    }
}
