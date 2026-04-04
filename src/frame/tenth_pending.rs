use crate::frame::tenth_open::TenthOpen;
use crate::frame::tenth_strike::TenthStrike;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A 10th frame with no rolls; first roll pending
#[derive(Debug)]
pub struct TenthPending {
    score: u16,
    bonuses: Multiplier,
}

impl TenthPending {
    pub fn new(score: u16, bonuses: Multiplier) -> Self {
        Self { score, bonuses }
    }
}

impl Frame for TenthPending {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let score = self.score + pins * self.bonuses.multiplier();

        // No bonuses are awarded in the 10th frame, but apply any bonuses from
        // previous frames.
        let bonuses = self.bonuses.open();

        if pins == 10 {
            // If first roll in 10th frame is a strike, progress to the
            // `TenthStrike` frame so the player gets a third roll.
            Ok(Box::new(TenthStrike::new(score, bonuses)))
        } else {
            // Else progress to the `TenthOpen` frame so the player requires a
            // spare on the second roll to get a third.
            Ok(Box::new(TenthOpen::new(10 - pins, score, bonuses)))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
