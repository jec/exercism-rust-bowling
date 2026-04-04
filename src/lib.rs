mod frame;
mod multiplier;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

/// Implements a game of bowling
///
/// The `frame` attribute contains a single `Frame` object that represents the
/// current, cumulative state of the game. Every roll results in a new `Frame`
/// object, even if the game remains in the same frame. The frame number is
/// incremented, and the game score accumulates within the `Frame` object.
pub struct BowlingGame {
    // Current state of the game
    frame: Box<dyn frame::Frame>,
}

impl BowlingGame {
    /// Returns a new `BowlingGame`
    pub fn new() -> Self {
        Self {
            frame: Box::new(frame::first_pending::FirstPending),
        }
    }

    /// Applies a roll to the current `Frame` and saves the new `Frame` object
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.frame = self.frame.roll(pins)?;
        Ok(())
    }

    /// Returns the score of a completed game
    ///
    /// If the game has not been completed, returns `None`.
    pub fn score(&self) -> Option<u16> {
        self.frame.score()
    }
}
