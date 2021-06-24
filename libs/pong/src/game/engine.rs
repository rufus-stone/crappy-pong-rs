use ggez::graphics::Rect;

use crate::settings::*;

/// Move the specified paddle, but prevent it from moving off the screen
pub fn move_paddle(paddle: &mut Rect, amount: f32) {
    if paddle.top() + amount < 0.0 {
        paddle.y = 0.0;
    } else if paddle.bottom() + amount > SCREEN_HEIGHT {
        paddle.y = SCREEN_HEIGHT - PADDLE_HEIGHT;
    } else {
        paddle.y += amount;
    }
}
