use crate::frame::open::Open;
use crate::frame::tenth_pending::TenthPending;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A frame with no rolls; first roll pending
#[derive(Debug)]
pub struct Pending {
    frame_number: u16,
    score: u16,
    bonuses: Multiplier,
}

impl Pending {
    pub fn new(frame_number: u16, score: u16, bonuses: Multiplier) -> Self {
        Self {
            frame_number,
            score,
            bonuses,
        }
    }
}

impl Frame for Pending {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let (score, bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == 10 {
            if self.frame_number == 9 {
                Ok(Box::new(TenthPending::new(score, bonuses)))
            } else {
                Ok(Box::new(Pending {
                    frame_number: self.frame_number + 1,
                    score,
                    bonuses,
                }))
            }
        } else {
            Ok(Box::new(Open::new(
                self.frame_number,
                10 - pins,
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
