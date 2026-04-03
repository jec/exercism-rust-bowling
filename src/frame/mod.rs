pub mod first_pending;
pub mod open;
pub mod pending;
pub mod tenth_closed;
pub mod tenth_final;
pub mod tenth_open;
pub mod tenth_pending;
pub mod tenth_strike;

pub trait Frame {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, crate::Error>;
    fn score(&self) -> Option<u16>;
}
