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
    score: u16,
    bonuses: Multiplier,
}

impl TenthStrike {
    pub fn new(score: u16, bonuses: Multiplier) -> Self {
        TenthStrike { score, bonuses }
    }
}

impl Frame for TenthStrike {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Advance to the third and final roll in frame 10.
        Ok(Box::new(TenthFinal::new(
            if pins == 10 { 10 } else { 10 - pins },
            self.score + pins * self.bonuses.multiplier(),
        )))
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
