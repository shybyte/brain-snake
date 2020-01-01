use crate::engine::common::*;
use raylib::consts::KeyboardKey::*;
use raylib::RaylibHandle;

pub fn get_cursor_key(rl: &RaylibHandle) -> Option<Direction> {
    if rl.is_key_down(KEY_LEFT) {
        Some(Direction::Left)
    } else if rl.is_key_down(KEY_RIGHT) {
        Some(Direction::Right)
    } else if rl.is_key_down(KEY_UP) {
        Some(Direction::Up)
    } else if rl.is_key_down(KEY_DOWN) {
        Some(Direction::Down)
    } else {
        None
    }
}
