use crate::frame::tenth_closed::TenthClosed;
use crate::frame::Frame;
use crate::Error;

/// A tenth frame with two previous rolls; third and final roll pending
#[derive(Debug)]
pub struct TenthFinal {
    pub pins: u16,
    pub score: u16,
}

impl Frame for TenthFinal {
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, pins);

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(Box::new(TenthClosed {
            score: self.score + pins,
        }))
    }

    fn score(&self) -> Option<u16> {
        None
    }
}
