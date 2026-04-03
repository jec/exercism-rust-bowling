mod frame;
mod multiplier;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frame: Box<dyn frame::Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame: Box::new(frame::first_pending::FirstPending),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.frame = self.frame.roll(pins)?;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.frame.score()
    }
}
