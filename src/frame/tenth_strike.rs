use crate::frame::tenth_final::TenthFinal;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// A 10th frame with one roll that was a strike; second and third rolls pending
///
/// No `pins` attribute: Since frame 10 started with a strike, this one always
/// has 10 pins.
#[derive(Debug)]
pub struct TenthStrike {
    pub score: u16,
    pub bonuses: Multiplier,
}

impl Frame for TenthStrike {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Advance to the third and final roll in frame 10.
        Ok(Box::new(TenthFinal {
            pins: if pins == 10 { 10 } else { 10 - pins },
            score: self.score + pins * self.bonuses.multiplier(),
        }))
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
