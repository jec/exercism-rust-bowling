pub mod first_pending;
pub mod open;
pub mod pending;
pub mod tenth_closed;
pub mod tenth_final;
pub mod tenth_open;
pub mod tenth_pending;
pub mod tenth_strike;

/// Represents the current frame in a game of bowling
pub trait Frame {
    /// Apply a roll to the current frame and return the resulting new `Frame`
    /// object or error
    ///
    /// A roll always returns a new `Frame` object (unless it errors), even if
    /// the game remains in the same frame.
    fn roll(&mut self, pins: u16) -> Result<Box<dyn Frame>, crate::Error>;

    /// Returns the optional score
    ///
    /// A score can only be calculated once the game has been completed. Before
    /// that, it always returns `None`.
    fn score(&self) -> Option<u16>;
}
