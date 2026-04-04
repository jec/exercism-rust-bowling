use crate::frame::tenth_closed::TenthClosed;
use crate::frame::tenth_final::TenthFinal;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A 10th frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
pub struct TenthOpen {
    pins: u16,
    score: u16,
    bonuses: Multiplier,
}

impl TenthOpen {
    pub fn new(pins: u16, score: u16, bonuses: Multiplier) -> Self {
        Self {
            pins,
            score,
            bonuses,
        }
    }
}

impl Frame for TenthOpen {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        let score = self.score + pins * self.bonuses.multiplier();

        if pins == self.pins {
            // Player rolled a spare; award a third roll in this 10th frame.
            Ok(Box::new(TenthFinal::new(10, score)))
        } else {
            // The game is over.
            Ok(Box::new(TenthClosed::new(score)))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
