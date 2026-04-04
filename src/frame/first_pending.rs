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
            Ok(Box::new(Pending {
                frame_number: 2,
                score: 10,
                bonuses: Multiplier::TwoTwo,
            }))
        } else {
            // Roll wasn't a strike; stay in frame 1.
            Ok(Box::new(Open {
                frame_number: 1,
                pins: 10 - pins,
                score: pins,
                bonuses: Multiplier::One,
            }))
        }
    }

    // Returns `None` since the game isn't finished
    fn score(&self) -> Option<u16> {
        None
    }
}
