use crate::frame::open::Open;
use crate::frame::pending::Pending;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// Initial state of game: first frame with no rolls
#[derive(Debug)]
pub struct FirstPending;

impl crate::frame::Frame for FirstPending {
    // Applies a roll and returns the next `Frame` object
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if pins == 10 {
            // Player rolled a strike; advance to frame 2 and award double points for next two rolls.
            Ok(Box::new(Pending::new(2, 10, Multiplier::TwoTwo)))
        } else {
            // Roll wasn't a strike; stay in frame 1.
            Ok(Box::new(Open::new(1, 10 - pins, pins, Multiplier::One)))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
