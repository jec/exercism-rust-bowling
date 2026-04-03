use crate::frame::open::Open;
use crate::frame::pending::Pending;
use crate::frame::Frame;
use crate::multiplier::Multiplier;
use crate::Error;

/// First frame with no rolls in game
#[derive(Debug)]
pub struct FirstPending;

impl crate::frame::Frame for FirstPending {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if pins == 10 {
            Ok(Box::new(Pending {
                frame_number: 2,
                score: 10,
                bonuses: Multiplier::TwoTwo,
            }))
        } else {
            Ok(Box::new(Open {
                frame_number: 1,
                pins: 10 - pins,
                score: pins,
                bonuses: Multiplier::One,
            }))
        }
    }

    fn score(&self) -> Option<u16> {
        None
    }
}
