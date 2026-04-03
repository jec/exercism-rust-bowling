use crate::frame::tenth_closed::TenthClosed;
use crate::frame::tenth_final::TenthFinal;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A tenth frame with one roll that was not a strike; second roll pending
#[derive(Debug)]
pub struct TenthOpen {
    pub pins: u16,
    pub score: u16,
    pub bonuses: Multiplier,
}

impl Frame for TenthOpen {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        // No bonuses carry over to next roll.
        let (score, _bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == self.pins {
            Ok(Box::new(TenthFinal { pins: 10, score }))
        } else {
            Ok(Box::new(TenthClosed { score }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}
