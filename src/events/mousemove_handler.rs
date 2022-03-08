// root imports
use crate::GAME;

/// delegates event handling based on turn num
pub fn handle_mousemove(id: String) {
    let game = unsafe { GAME.as_mut().unwrap() };

    // only allow player cards on hover
    game.active.hover = if id.chars().nth(0).unwrap_or('_') == 'p' {
        Some(id)
    } else {
        None
    }
}
