use crate::frame::Frame;
use crate::Error;

/// A tenth frame that has been completed with either two or three rolls; no more rolls allowed
#[derive(Debug)]
pub struct TenthClosed {
    pub score: u16,
}

impl Frame for TenthClosed {
    fn roll(&mut self, _pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, _pins);

        Err(Error::GameComplete)
    }

    fn score(&self) -> Option<u16> {
        Some(self.score)
    }
}
