use crate::frame::pending::Pending;
use crate::frame::tenth_pending::TenthPending;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
pub struct Open {
    frame_number: u16,
    pins: u16,
    score: u16,
    bonuses: Multiplier,
}

impl Open {
    pub fn new(frame_number: u16, pins: u16, score: u16, bonuses: Multiplier) -> Self {
        Open {
            frame_number,
            pins,
            score,
            bonuses,
        }
    }
}

impl Frame for Open {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (score, bonuses) = self.bonuses.calculate_score(pins, self.pins, self.score);

        if self.frame_number == 9 {
            // Advance to 10th frame.
            Ok(Box::new(TenthPending::new(score, bonuses)))
        } else {
            // Advance to next frame.
            Ok(Box::new(Pending::new(
                self.frame_number + 1,
                score,
                bonuses,
            )))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
