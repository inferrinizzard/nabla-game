// outer crate imports
use crate::render::render;
// root imports
use crate::GAME;

/// delegates event handling based on turn num
pub fn handle_mousemove(id: String) {
    let game = unsafe { GAME.as_mut().unwrap() };

    let prev = game.active.hover.clone();
    // only allow player cards on hover
    game.active.hover = if id.chars().nth(0).unwrap_or('_') == 'p' {
        Some(id)
    } else {
        None
    };
    if matches!(game.active.hover, Some(_)) || (matches!(game.active.hover, None) && prev.is_some())
    {
        render::draw();
    }
}
