use crate::frame::tenth_open::TenthOpen;
use crate::frame::tenth_strike::TenthStrike;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A tenth frame with no rolls; first roll pending
#[derive(Debug)]
pub struct TenthPending {
    pub score: u16,
    pub bonuses: Multiplier,
}

impl Frame for TenthPending {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let score = self.score + pins * self.bonuses.multiplier();

        // Give no further bonus scoring for this roll, regardless of whether it's a strike.
        let bonuses = self.bonuses.open();

        if pins == 10 {
            Ok(Box::new(TenthStrike { score, bonuses }))
        } else {
            Ok(Box::new(TenthOpen {
                pins: 10 - pins,
                score,
                bonuses,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}
