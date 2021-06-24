use crate::entities::ball::Ball;

pub mod ai;
pub mod human;

#[derive(Debug)]
pub enum Player {
    Human,
    Computer,
}

pub struct Snapshot {
    pub(crate) paddle: ggez::graphics::Rect,
    pub(crate) ball: Ball,
}

impl Snapshot {
    pub fn new(paddle: &ggez::graphics::Rect, ball: &Ball) -> Self {
        Self {
            paddle: *paddle,
            ball: ball.clone(),
        }
    }
}

pub trait Move {
    fn make_move(&self, ctx: &mut ggez::Context, _snapshot: &Snapshot) -> f32;
    fn name(&self) -> &'static str;
}

impl std::fmt::Debug for dyn Move {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Move {{ member: {:?} }}", self.name())
    }
}
