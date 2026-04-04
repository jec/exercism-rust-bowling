use crate::frame::Frame;
use crate::Error;

/// A 10th frame that has been completed with either two or three rolls; no
/// more rolls allowed
///
/// This is the only frame that will return a score.
#[derive(Debug)]
pub struct TenthClosed {
    pub score: u16,
}

impl Frame for TenthClosed {
    // Returns an error since the game is finished
    fn roll(&mut self, _pins: u16) -> Result<Box<dyn Frame>, Error> {
        println!("roll({:?}, {})", self, _pins);

        Err(Error::GameComplete)
    }

    // Returns the score
    fn score(&self) -> Option<u16> {
        Some(self.score)
    }
}
