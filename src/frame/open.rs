use crate::frame::pending::Pending;
use crate::frame::tenth_pending::TenthPending;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
pub struct Open {
    pub frame_number: u16,
    pub pins: u16,
    pub score: u16,
    pub bonuses: Multiplier,
}

impl Frame for Open {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (score, bonuses) = self.bonuses.calculate_score(pins, self.pins, self.score);

        if pins == self.pins {
            if self.frame_number == 9 {
                Ok(Box::new(TenthPending { score, bonuses }))
            } else {
                Ok(Box::new(Pending {
                    frame_number: self.frame_number + 1,
                    score,
                    bonuses,
                }))
            }
        } else if self.frame_number == 9 {
            Ok(Box::new(TenthPending { score, bonuses }))
        } else {
            Ok(Box::new(Pending {
                frame_number: self.frame_number + 1,
                score,
                bonuses,
            }))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
