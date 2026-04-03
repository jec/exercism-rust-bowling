use crate::frame::tenth_final::TenthFinal;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A tenth frame with one roll that was a strike; second and third rolls pending
#[derive(Debug)]
pub struct TenthStrike {
    pub score: u16,
    pub bonuses: Multiplier,
}

impl Frame for TenthStrike {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // No bonuses carry over to next roll.
        let (score, _bonuses) = self.bonuses.calculate_score(pins, 10, self.score);

        if pins == 10 {
            Ok(Box::new(TenthFinal { pins: 10, score }))
        } else {
            Ok(Box::new(TenthFinal {
                pins: 10 - pins,
                score,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}
