use crate::frame::tenth_closed::TenthClosed;
use crate::frame::Frame;
use crate::Error;

/// A 10th frame with the final roll pending
///
/// This `Frame` follows either one or two rolls in the 10th frame. If the
/// first roll of the 10th frame was a strike, then this frame awaits the third
/// and final roll.
///
/// If the first roll of the 10th frame was not a strike and the second roll
/// was not a spare, than this frame awaits the second and final roll.
///
/// In both cases, there are no more bonuses to be applied, so there is no
/// `bonuses` attribute.
#[derive(Debug)]
pub struct TenthFinal {
    pub pins: u16,
    pub score: u16,
}

impl Frame for TenthFinal {
    // Applies a roll and returns the next `Frame` object
    //
    // The next frame is always a `TenthClosed` frame since we're on the final
    // roll of the 10th frame.
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(Box::new(TenthClosed {
            score: self.score + pins,
        }))
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
